use std::{mem, slice};

use itertools::Either;
use Either::{Left, Right};

use crate::quad_tree::QuadTree;
use crate::quad_tree::QuadTreeChildren::{Leaves, Nodes};

pub struct Iter<'a, Leaf> {
    children: Either<&'a [Leaf], &'a [QuadTree<Leaf>]>,
    parent: Option<Box<Self>>,
}

impl<'a, Leaf> Iter<'a, Leaf> {
    pub(super) fn new(tree: &'a QuadTree<Leaf>, parent: Option<Box<Self>>) -> Self {
        Self {
            children: Right(slice::from_ref(tree)),
            parent,
        }
    }

    fn pop(&mut self) -> Option<&'a Leaf> {
        match self.parent.take() {
            Some(parent) => {
                *self = *parent;
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, Leaf> Iterator for Iter<'a, Leaf> {
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.children {
            Left(leaves) => {
                if let Some((head, rest)) = leaves.split_first() {
                    *leaves = rest;
                    return Some(head);
                }
            }
            Right(nodes) => {
                if let Some((head, rest)) = nodes.split_first() {
                    *nodes = rest;
                    *self = Self {
                        children: match &head.children {
                            Leaves(leaves) => Left(leaves),
                            Nodes(nodes) => Right(nodes.as_slice()),
                        },
                        parent: Some(Box::new(mem::take(self))),
                    };
                    return self.next();
                }
            }
        }
        self.pop()
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

        let iter = Iter::new(&tree, None);
        let xs = iter.map(|p| p.x as i32).sorted().collect_vec();
        println!("{:?}", xs);
        assert!(xs == (-9..9).collect_vec())
    }
}
