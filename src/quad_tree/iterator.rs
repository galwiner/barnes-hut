use std::slice;

use itertools::Either;
use Either::{Left, Right};

use crate::geometry::{BoundingBox, Positioned};
use crate::quad_tree::QuadTree;
use crate::quad_tree::QuadTreeChildren::{Leaves, Nodes};

pub enum TreePosition<'a, Leaf> {
    Leaf(&'a Leaf),
    Node(&'a QuadTree<Leaf>),
}

pub trait TreeIterator: Iterator {
    fn leaves(self) -> LeafIter<Self>
    where
        Self: Sized,
    {
        LeafIter::new(self)
    }

    fn nodes(self) -> NodeIter<Self>
    where
        Self: Sized,
    {
        NodeIter::new(self)
    }

    fn bounded(self, bounds: BoundingBox) -> Bounded<Self>
    where
        Self: Sized,
    {
        Bounded::new(self, bounds)
    }

    /// Skip all nodes/leaves under the last visited node if any remain.
    fn skip_subtree(&mut self);
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct DepthFirstIter<'a, Leaf> {
    children: Either<&'a [Leaf], &'a [QuadTree<Leaf>]>,
    parent: Option<Box<Self>>,
}

impl<'a, Leaf> DepthFirstIter<'a, Leaf> {
    pub(super) fn new(tree: &'a QuadTree<Leaf>) -> Self {
        Self {
            children: Right(slice::from_ref(tree)),
            parent: None,
        }
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

impl<'a, Leaf> TreeIterator for DepthFirstIter<'a, Leaf> {
    fn skip_subtree(&mut self) {
        self.ascend();
    }
}

impl<'a, Leaf> Iterator for DepthFirstIter<'a, Leaf> {
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

impl<Leaf> Default for DepthFirstIter<'_, Leaf> {
    fn default() -> Self {
        Self {
            children: Left(&[]),
            parent: None,
        }
    }
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LeafIter<Inner> {
    inner: Inner,
}

impl<Inner> LeafIter<Inner> {
    pub(self) fn new(inner: Inner) -> Self {
        Self { inner }
    }

    #[allow(dead_code)]
    fn bounded(self, bounds: BoundingBox) -> NodeIter<Bounded<Inner>> {
        NodeIter::new(Bounded::new(self.inner, bounds))
    }
}

impl<'a, Leaf, Inner> Iterator for LeafIter<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
{
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(TreePosition::Leaf(leaf)) => Some(leaf),
            Some(TreePosition::Node(_)) => self.next(),
            None => None,
        }
    }
}

impl<'a, Leaf, Inner> TreeIterator for LeafIter<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
{
    fn skip_subtree(&mut self) {
        self.inner.skip_subtree();
    }
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NodeIter<Inner> {
    inner: Inner,
}

impl<Inner> NodeIter<Inner> {
    fn new(inner: Inner) -> Self {
        Self { inner }
    }

    #[allow(dead_code)]
    fn bounded(self, bounds: BoundingBox) -> NodeIter<Bounded<Inner>> {
        NodeIter::new(Bounded::new(self.inner, bounds))
    }
}

impl<'a, Leaf, Inner> Iterator for NodeIter<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
{
    type Item = &'a QuadTree<Leaf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(TreePosition::Leaf(_)) => {
                self.inner.skip_subtree();
                self.next()
            }
            Some(TreePosition::Node(node)) => Some(node),
            None => None,
        }
    }
}

impl<'a, Leaf, Inner> TreeIterator for NodeIter<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
{
    fn skip_subtree(&mut self) {
        self.inner.skip_subtree();
    }
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Bounded<Inner> {
    inner: Inner,
    bounds: BoundingBox,
}

impl<Inner> Bounded<Inner> {
    fn new(inner: Inner, bounds: BoundingBox) -> Self {
        Self { inner, bounds }
    }
}

impl<'a, Leaf, Inner> Iterator for Bounded<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
    Leaf: Positioned,
{
    type Item = Inner::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(TreePosition::Leaf(leaf)) => {
                    if self.bounds.contains(leaf.position()) {
                        return Some(TreePosition::Leaf(leaf));
                    }
                }
                Some(TreePosition::Node(node)) => {
                    if self.bounds.overlap(node.boundary).is_some() {
                        return Some(TreePosition::Node(node));
                    }
                    self.inner.skip_subtree();
                }
                None => return None,
            }
        }
    }
}

impl<'a, Leaf, Inner> TreeIterator for Bounded<Inner>
where
    Inner: TreeIterator<Item = TreePosition<'a, Leaf>>,
    Leaf: 'a,
    Leaf: Positioned,
{
    fn skip_subtree(&mut self) {
        self.inner.skip_subtree();
    }
}

// unit tests:
#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use nannou::geom::pt2;

    use crate::geometry::BoundingBox;

    use super::*;

    const SIZE: f32 = 1024.0;

    fn test_iter<F, L>(mut f: F)
    where
        F: FnMut(i32) -> L,
        L: Positioned + Clone,
    {
        let mut tree = QuadTree::new(BoundingBox::from_w_h(SIZE, SIZE));
        (0..10).for_each(|x| {
            tree.insert(f(x));
        });
        let bounded = tree
            .iter()
            .bounded(BoundingBox::from_corner_points([-2.0, -2.0], [3.0, 3.0]));

        let xs = bounded
            .clone()
            .leaves()
            .map(|p| p.position().x as i32)
            .sorted()
            .collect_vec();
        println!("leaves at: {xs:?}");

        assert_eq!(xs, (0..=3).collect_vec());

        let node_iter = bounded.clone().nodes();
        println!(
            "nodes ({}) at: {}",
            node_iter.clone().count(),
            node_iter
                .map(|qt| { format!("{:?}", qt.boundary.x_y()) })
                .join(", "),
        );
    }

    #[test]
    fn test_iter_owned() {
        test_iter(|x| pt2(x as f32, x as f32));
    }

    #[test]
    fn boxed_items() {
        test_iter(|x| Box::new(pt2(x as f32, x as f32)));
    }
}

//region Description
pub struct IntoIter<Leaf> {
    leaves: Vec<Leaf>,
    nodes: Vec<QuadTree<Leaf>>,
}

impl<Leaf> Iterator for IntoIter<Leaf> {
    type Item = Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(leaf) = self.leaves.pop() {
            return Some(leaf);
        }
        if let Some(node) = self.nodes.pop() {
            match node.children {
                Leaves(leaves) => {
                    self.leaves = leaves;
                    self.next()
                }
                Nodes(nodes) => {
                    self.nodes.extend(*nodes);
                    self.next()
                }
            }
        } else {
            None
        }
    }
}

impl<Leaf> IntoIterator for QuadTree<Leaf> {
    type Item = Leaf;
    type IntoIter = IntoIter<Leaf>;

    fn into_iter(self) -> Self::IntoIter {
        match self.children {
            Leaves(leaves) => IntoIter {
                leaves,
                nodes: Vec::new(),
            },
            Nodes(nodes) => IntoIter {
                leaves: Vec::new(),
                nodes: nodes.into_iter().collect(),
            },
        }
    }
}
//endregion
