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

    pub fn insert(&mut self, item: Leaf)
    where
        Leaf: Positioned,
    {
        let position = item.position();
        if !self.boundary.contains(position) {
            warn!("Dropped insert at out-of-bounds position {}", position);
            return;
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
                    leaves.into_iter().for_each(|l| self.insert(l));
                }
            }
            Nodes(nodes) => {
                for node in nodes.iter_mut() {
                    if node.boundary.contains(position) {
                        return node.insert(item);
                    }
                }
                unreachable!("position {} should be in a subtree", position);
            }
        }
    }
}

impl<Leaf: Positioned> Extend<Leaf> for QuadTree<Leaf> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Leaf>,
    {
        iter.into_iter().for_each(|p| self.insert(p));
    }
}
