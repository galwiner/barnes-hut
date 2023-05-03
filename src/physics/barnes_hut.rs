use std::ops::AddAssign;
use nannou::geom::{Rect, vec2};


use crate::physics::space_2d::Space2D;
use crate::view_state::ViewState;
use super::point_mass::PointMass;
use super::space::DivisibleSpace;

pub type GravityField2D = GravityField<Space2D, 4>;

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
enum Child<S, const NUM_SUBDIVISIONS: usize>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    #[derivative(Default)]
    Empty,
    Body(PointMass<S>),
    Aggregate(Box<MassAggregate<S, NUM_SUBDIVISIONS>>),
}

#[derive(Debug, Clone)]
pub struct MassAggregate<S, const NUM_SUBDIVISIONS: usize>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    /// The mass and center of mass of all points at or under this node.
    total: PointMass<S>,
    pivot: S::Vector,
    width: S::Scalar,

    subdivisions: [Child<S, NUM_SUBDIVISIONS>; NUM_SUBDIVISIONS],
}

impl<S, const NUM_SUBDIVISIONS: usize>  MassAggregate<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    pub fn new(pivot: S::Vector, width: S::Scalar) -> MassAggregate<S, NUM_SUBDIVISIONS> {
        MassAggregate {
            total: PointMass::default(),
            pivot,
            width,
            subdivisions: S::subdivisions_array_default(),
        }
    }
}

impl MassAggregate<Space2D, 4> {
    pub(crate) fn get_bounding_rect(&self) -> Rect {

        Rect::from_xy_wh(self.pivot, vec2(self.width*0.5, self.width*0.5))
    }
}

// impl<S, const NUM_SUBDIVISIONS: usize> Default for MassAggregate<S, NUM_SUBDIVISIONS>
// where
//     S: DivisibleSpace<NUM_SUBDIVISIONS>,
// {
//     fn default() -> Self {
//         Self {
//             total: PointMass::default(),
//             subdivisions: S::subdivisions_array_default(),
//         }
//     }
// }

impl<S, const NUM_SUBDIVISIONS: usize> MassAggregate<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    fn insert(&mut self, body: PointMass<S>) {
        self.total += body;
        let subdivision_index = S::subdivision_index(self.pivot, body.position);
        let child = &mut self.subdivisions[subdivision_index];

        match child {
            Child::Empty => {
                *child = Child::Body(body);
            }
            Child::Body(existing_body) => {
                let (width, pivot) = S::subtree_width_pivot(subdivision_index, self.width, self.pivot);
                if width < S::EPSILON {
                    *existing_body += body;
                    return;
                }
                let mut aggregate = MassAggregate::new(pivot, width);
                aggregate.insert( *existing_body);
                aggregate.insert( body);
                *child = Child::Aggregate(Box::new(aggregate));
            }
            Child::Aggregate(aggregate) => {
                aggregate.insert( body);
            }
        }
    }

    pub fn estimate_net_g(
        &self,
        other_position: S::Vector,
        pivot: S::Vector,
        width: S::Scalar,
        theta_squared: S::Scalar,
        grav_const: S::Scalar,
    ) -> S::Vector {
        let to_self: S::Vector = self.total.position - other_position;
        let distance_squared: S::Scalar = S::magnitude_squared(to_self);
        if (width * width) <= theta_squared * distance_squared {
            return self.total.g_at(other_position, grav_const);
        }

        let mut sum = S::VECTOR_ZERO;

        self.subdivisions
            .iter()
            .enumerate()
            .for_each(|(i, child)| match child {
                Child::Empty => {}
                Child::Body(body) => {
                    sum += body.g_at(other_position, grav_const);
                }
                Child::Aggregate(aggregate) => {
                    let (width, pivot) = S::subtree_width_pivot(i, width, pivot);
                    sum += aggregate.estimate_net_g(
                        other_position,
                        pivot,
                        width,
                        theta_squared,
                        grav_const,
                    );
                }
            });
        sum
    }
}

#[derive(Debug, Clone)]
pub struct GravityField<S, const NUM_SUBDIVISIONS: usize>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    origin: S::Vector,

    /// The length in each dimension of the space covered by this field.  At present this must be set large enough up-front.
    width: S::Scalar,

    root: MassAggregate<S, NUM_SUBDIVISIONS>,
}

impl GravityField2D{
    pub(crate) fn get_bounding_boxes(&self) -> Vec<Rect> {
        let mut mass_aggregates = vec![self.root.clone()];
        let mut rects = Vec::new();
        while let Some(mass_aggreate) = mass_aggregates.pop(){
            rects.push(mass_aggreate.get_bounding_rect());
            for child in mass_aggreate.subdivisions{
                match child{
                    Child::Empty => {}
                    Child::Body(_) => {}
                    Child::Aggregate(aggregate) => {
                        mass_aggregates.push(*aggregate);
                    }
                }
            }
        }
        rects
    }
}

impl<S, const NUM_SUBDIVISIONS: usize> GravityField<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    pub fn new(size: S::Scalar) -> Self {
        Self {
            origin: S::VECTOR_ZERO,
            width: size,
            root: MassAggregate::new(S::VECTOR_ZERO, size),
        }
    }

    pub fn insert(&mut self, rhs: PointMass<S>) {
        if rhs.mass == S::SCALAR_ZERO {
            return;
        }
        if S::max_abs_dimension(rhs.position) >= self.width {
            // TODO!
            warn!("PointMass out of bounds: {:?}", rhs);
            return;
        }
        self.root.insert( rhs);
    }

    pub fn estimate_net_g(
        &self,
        at: S::Vector,
        theta: S::Scalar,
        grav_const: S::Scalar,
    ) -> S::Vector {
        self.root
            .estimate_net_g(at, self.origin, self.width, theta * theta, grav_const)
    }
}

impl<S, const NUM_SUBDIVISIONS: usize> AddAssign<PointMass<S>> for GravityField<S, NUM_SUBDIVISIONS>
where
    S: DivisibleSpace<NUM_SUBDIVISIONS>,
{
    fn add_assign(&mut self, rhs: PointMass<S>) {
        self.insert(rhs);
    }
}
