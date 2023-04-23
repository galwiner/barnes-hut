use std::borrow::BorrowMut;
use std::ops::AddAssign;

use super::point_mass::PointMass;
use super::space::DivisibleSpace;
use super::space::Space2D;

pub type GravityField2D = GravityField<Space2D, 4>;

#[derive(Debug, Clone)]
pub struct GravityField<S, const SUBDIVS: usize>
where
    S: DivisibleSpace<SUBDIVS>,
{
    /// The point about which space is subdivided.  Currently this will be taken as the midpoint
    /// of the first two nodes to end up in a subdivision.
    /// This could result in pathological cases e.g. where nodes are placed in order along a line,
    /// and in theory means that the subdivision debt is unbounded. Hopefully in practice the tree
    /// will be fairly balanced.  If not, we could consider using a different pivot point, or e.g.
    /// inserting points into the tree in a random order.
    pivot: S::Vector,

    /// The minimal axis-aligned bounding box that contains all points at or under this node.
    bounds: S::Bounds,

    /// The mass and center of mass of all points at or under this node.
    total: PointMass<S>,

    /// The `None` case represents a leaf node, otherwise the `total` at this node is an aggregate
    /// of the subtrees.
    subdivisions: Option<Box<[Self; SUBDIVS]>>,
}

impl<S, const SUBDIVS: usize> Default for GravityField<S, SUBDIVS>
where
    S: DivisibleSpace<SUBDIVS>,
    [(); SUBDIVS]: Default,
{
    fn default() -> Self {
        Self {
            pivot: S::ZERO_VECTOR,
            bounds: S::EMPTY_BOUNDS,
            total: Default::default(),
            subdivisions: None,
        }
    }
}

impl<S, const NUM_SUBDIVISIONS: usize> GravityField<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
    [Self; NUM_SUBDIVISIONS]: Default,
{
    pub fn insert(&mut self, rhs: PointMass<S>) {
        if rhs.mass == S::ZERO {
            return;
        }

        if self.total.mass == S::ZERO {
            self.bounds = S::point_bounds(rhs.position);
            self.total = rhs;
            return;
        }

        self.bounds = S::expand_bounds(self.bounds, rhs.position);
        if self.subdivisions.is_none() {
            self.subdivisions = Some(Default::default());
            self.pivot = S::midpoint(self.bounds);
            self.add_to_subdivision(self.total);
        }
        self.total += rhs;
        self.add_to_subdivision(rhs);
    }

    pub fn estimate_net_g(
        &self,
        at: S::Vector,
        theta: S::Scalar,
        grav_const: S::Scalar,
    ) -> S::Vector {
        let mut accel = S::ZERO_VECTOR;
        GravityFieldIterator {
            stack: vec![&self],
            location: at,
            theta,
        }
        .for_each(|body_or_aggregate| {
            accel += body_or_aggregate.g_at(at, grav_const);
        });
        accel
    }

    fn add_to_subdivision(&mut self, body: PointMass<S>)
    where
        [Self; NUM_SUBDIVISIONS]: Default,
        Self: AddAssign<PointMass<S>>,
    {
        let index = S::subdivision_index(self.pivot, body.position);
        let subdivisions: &mut [Self; NUM_SUBDIVISIONS] = self
            .subdivisions
            .get_or_insert_with(Default::default)
            .borrow_mut();
        subdivisions[index] += body;
    }
}

impl<S, const NUM_SUBDIVISIONS: usize> AddAssign<PointMass<S>> for GravityField<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
    [Self; NUM_SUBDIVISIONS]: Default,
{
    fn add_assign(&mut self, rhs: PointMass<S>) {
        self.insert(rhs);
    }
}

struct GravityFieldIterator<'a, S, const SUBDIVS: usize>
where
    S: DivisibleSpace<SUBDIVS>,
{
    stack: Vec<&'a GravityField<S, SUBDIVS>>,
    location: S::Vector,
    theta: S::Scalar,
}

impl<'a, S, const SUBDIVS: usize> Iterator for GravityFieldIterator<'a, S, SUBDIVS>
where
    S: DivisibleSpace<SUBDIVS>,
{
    type Item = PointMass<S>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tree) = self.stack.pop() {
            if tree.total.mass == S::ZERO {
                continue;
            }
            if tree.subdivisions.is_none() {
                if tree.total.position != self.location {
                    return Some(tree.total);
                } else {
                    continue;
                }
            }
            if !S::contains(tree.bounds, self.location) {
                let distance = S::magnitude(tree.total.position - self.location);
                let size = S::max_dimension(tree.bounds);
                if size / distance < self.theta {
                    return Some(tree.total);
                }
            }
            let subtrees = &tree.subdivisions.as_ref().unwrap()[..];
            self.stack.extend(subtrees)
        }
        None
    }
}
