use std::slice;

use itertools::Either;
use Either::{Left, Right};

use crate::quad_tree::QuadTree;
use crate::quad_tree::QuadTreeChildren::{Leaves, Nodes};

pub enum TreePosition<'a, Leaf> {
    Leaf(&'a Leaf),
    Node(&'a QuadTree<Leaf>),
}

#[derive(Clone)]
pub struct Iter<'a, Leaf> {
    children: Either<&'a [Leaf], &'a [QuadTree<Leaf>]>,
    parent: Option<Box<Self>>,
}

#[derive(Clone)]
pub struct LeafIter<'a, Leaf>(Iter<'a, Leaf>);

#[derive(Clone)]
pub struct NodeIter<'a, Leaf>(Iter<'a, Leaf>);

impl<'a, Leaf> Iter<'a, Leaf> {
    pub(super) fn new(tree: &'a QuadTree<Leaf>) -> Self {
        Self {
            children: Right(slice::from_ref(tree)),
            parent: None,
        }
    }

    pub fn leaves(self) -> LeafIter<'a, Leaf> {
        LeafIter(self)
    }

    pub fn nodes(self) -> NodeIter<'a, Leaf> {
        NodeIter(self)
    }

    fn ascend(&mut self) -> bool {
        match self.parent.take() {
            None => false,
            Some(parent) => {
                *self = *parent;
                true
            }
        }
    }
}

impl<'a, Leaf> Iterator for Iter<'a, Leaf> {
    type Item = TreePosition<'a, Leaf>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.children {
            Left([head, rest @ ..]) => {
                self.children = Left(rest);
                Some(TreePosition::Leaf(head))
            }
            Right([head, rest @ ..]) => {
                *self = Self {
                    children: match head.children {
                        Leaves(ref leaves) => Left(leaves),
                        Nodes(ref nodes) => Right(nodes.as_slice()),
                    },
                    parent: Some(Box::new(Self {
                        children: Right(rest),
                        parent: self.parent.take(),
                    })),
                };
                Some(TreePosition::Node(head))
            }
            _ => {
                if self.ascend() {
                    self.next()
                } else {
                    None
                }
            }
        }
    }
}

impl<Leaf> Default for Iter<'_, Leaf> {
    fn default() -> Self {
        Self {
            children: Left(&[]),
            parent: None,
        }
    }
}

impl<'a, Leaf> Iterator for LeafIter<'a, Leaf> {
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(TreePosition::Leaf(leaf)) => Some(leaf),
            Some(TreePosition::Node(_)) => self.next(),
            None => None,
        }
    }
}

impl<'a, Leaf> Iterator for NodeIter<'a, Leaf> {
    type Item = &'a QuadTree<Leaf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(TreePosition::Leaf(_)) => {
                self.0.ascend();
                self.next()
            }
            Some(TreePosition::Node(node)) => Some(node),
            None => None,
        }
    }
}

// unit tests:
#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use nannou::geom::pt2;

    use crate::geometry::BoundingBox;

    use super::*;

    #[test]
    fn test_iter() {
        let mut tree = QuadTree::new(BoundingBox::from_w_h(20.0, 20.0));
        (-9..9).for_each(|x| {
            tree.insert(pt2(x as f32, x as f32));
        });

        let xs = tree
            .iter()
            .leaves()
            .map(|p| p.x as i32)
            .sorted()
            .collect_vec();
        println!("leaves at: {xs:?}");

        assert!(xs == (-9..9).collect_vec());

        let node_iter = tree.iter().nodes();
        println!(
            "nodes ({}) at: {}",
            node_iter.clone().count(),
            node_iter
                .map(|qt| { format!("{:?}", qt.boundary.x_y()) })
                .join(", "),
        );
    }
}
