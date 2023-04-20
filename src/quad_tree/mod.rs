extern crate nannou;

use std::mem;

use nannou::geom::{Point2, Rect};

use iterator::DepthFirstIter;
use QuadTreeChildren::{Leaves, Nodes};

pub mod iterator;

pub trait Positioned {
    fn position(&self) -> Point2;
}

pub const TARGET_MAX_LEAVES: usize = 2;

#[derive(Debug, Clone)]
enum QuadTreeChildren<Leaf> {
    Leaves(Vec<Leaf>),
    Nodes(Box<[QuadTree<Leaf>; 4]>),
}

#[derive(Debug, Clone)]
pub struct QuadTree<Leaf> {
    boundary: Rect,
    children: QuadTreeChildren<Leaf>,
}

#[derive(Debug, Clone)]
pub enum Error {
    OutOfBounds,
}

impl<Leaf> QuadTree<Leaf> {
    pub fn new(boundary: Rect) -> Self {
        Self {
            boundary,
            children: Leaves(Vec::new()),
        }
    }

    pub fn boundary(&self) -> Rect {
        self.boundary
    }

    pub fn iter(&self) -> DepthFirstIter<Leaf> {
        DepthFirstIter::new(self)
    }

    pub fn insert(&mut self, item: Leaf) -> Result<(), Error>
    where
        Leaf: Positioned,
    {
        let position = item.position();
        if !self.boundary.contains(position) {
            return Err(Error::OutOfBounds);
        }

        match &mut self.children {
            Leaves(ref mut leaves) => {
                let split_tree = leaves.len() >= TARGET_MAX_LEAVES
                    && !leaves.iter().any(|leaf| leaf.position() == position);
                leaves.push(item);

                if split_tree {
                    let leaves = mem::take(leaves);
                    let subtrees = self.boundary.subdivisions().map(QuadTree::<Leaf>::new);
                    self.children = Nodes(Box::new(subtrees));
                    leaves.into_iter().try_for_each(|l| self.insert(l))?;
                }
                Ok(())
            }
            Nodes(nodes) => {
                for node in nodes.iter_mut() {
                    if node.boundary.contains(position) {
                        return node.insert(item);
                    }
                }
                panic!("position {} should be in a subtree", position);
            }
        }
    }
}
