extern crate nannou;

use std::mem::swap;

use iterator::DepthFirstIter;
use QuadTreeChildren::{Leaves, Nodes};

use crate::geometry::{BoundingBox, Positioned};

pub mod iterator;

pub const MAX_LEAVES: usize = 4;

#[derive(Debug)]
enum QuadTreeChildren<Leaf> {
    Leaves(Vec<Leaf>),
    Nodes(Box<[QuadTree<Leaf>; 4]>),
}

#[derive(Debug)]
pub struct QuadTree<Leaf> {
    boundary: BoundingBox,
    children: QuadTreeChildren<Leaf>,
}

impl<Leaf> QuadTree<Leaf> {
    pub fn new(boundary: BoundingBox) -> Self {
        Self {
            boundary,
            children: Leaves(Vec::new()),
        }
    }

    pub fn boundary(&self) -> BoundingBox {
        self.boundary
    }

    pub fn iter(&self) -> DepthFirstIter<Leaf> {
        DepthFirstIter::new(self)
    }

    pub fn insert(&mut self, item: Leaf) -> bool
    where
        Leaf: Positioned,
    {
        let position = item.position();
        if !self.boundary.contains(position) {
            return false;
        }

        match &mut self.children {
            Leaves(leaves) => {
                leaves.push(item);
                if leaves.len() > MAX_LEAVES {
                    if leaves
                        .iter()
                        .filter(|leaf| leaf.position() == position)
                        .count()
                        > 1
                    {
                        // not subdividing node with multiple leaves at the same position
                        return true;
                    }
                    let subtrees = self.boundary.subdivisions().map(QuadTree::<Leaf>::new);
                    let mut swapped_children = Nodes(Box::new(subtrees));
                    swap(&mut self.children, &mut swapped_children);
                    match swapped_children {
                        Leaves(leaves) => leaves.into_iter().for_each(|leaf| {
                            self.insert(leaf);
                        }),
                        _ => panic!("swapped_children should be Leaves"),
                    }
                    true
                } else {
                    false
                }
            }
            Nodes(nodes) => {
                for node in nodes.iter_mut() {
                    if node.boundary.contains(position) {
                        return node.insert(item);
                    }
                }
                false
            }
        }
    }
}

impl<Leaf: Positioned> Extend<Leaf> for QuadTree<Leaf> {
    fn extend<T: IntoIterator<Item = Leaf>>(&mut self, iter: T) {
        iter.into_iter().for_each(|item| {
            self.insert(item);
        });
    }
}
