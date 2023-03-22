extern crate nannou;

use rand_distr::{Normal, Distribution};

use nannou::prelude::*;
use nannou::rand;
use nannou::winit::window::CursorIcon::Default;

const CAPACITY: usize = 4;
const WINDOW_SIZE: f32 = 800.0;
const NDIMS: usize = 2;
const ORTHANT_NUM: usize = usize::pow(2, NDIMS as u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Point2::new(x, y);
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 1.0;
        Self {
            position,
            velocity,
            acceleration,
            mass,
        }
    }
    pub fn new_random() -> Self {
        let normal = Normal::new(0.0, WINDOW_SIZE / 2.0).unwrap();
        let random_sample = || normal.sample(&mut rand::thread_rng());
        let position = Point2::new(random_sample(), random_sample());
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 1.0;
        Self {
            position,
            velocity,
            acceleration,
            mass,
        }
    }
    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(5.0, 5.0)
            .rgba(102.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 0.4)
            .stroke(rgba(0.0, 0.0, 0.0, 1.0));
        // .stroke_weight(2.0);
    }
    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundary {
    center: Point2,
    width: f32,
    height: f32,
}

impl Boundary {
    pub fn new(center: Point2, width: f32, height: f32) -> Self {
        Self {
            center,
            width,
            height,
        }
    }
    pub fn contains(self, particle: &Particle) -> bool {
        let x = particle.position.x;
        let y = particle.position.y;
        let x1 = self.center.x - self.width / 2.0;
        let x2 = self.center.x + self.width / 2.0;
        let y1 = self.center.y - self.height / 2.0;
        let y2 = self.center.y + self.height / 2.0;
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return true;
        }
        false
    }
    pub fn intersects(self, range: &Boundary) -> bool {
        let x1 = self.center.x - self.width / 2.0;
        let x2 = self.center.x + self.width / 2.0;
        let y1 = self.center.y - self.height / 2.0;
        let y2 = self.center.y + self.height / 2.0;
        let x3 = range.center.x - range.width / 2.0;
        let x4 = range.center.x + range.width / 2.0;
        let y3 = range.center.y - range.height / 2.0;
        let y4 = range.center.y + range.height / 2.0;
        if x1 > x4 || x2 < x3 || y1 > y4 || y2 < y3 {
            return false;
        }
        true
    }

    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.center.x, self.center.y)
            .w_h(self.width, self.height)
            .rgba(0.0, 0.0, 0.0, 1.0)
            .stroke(rgba(1.0, 0.0, 0.0, 1.0))
            .stroke_weight(1.0);
    }
}

//
// #[derive(PartialEq, Debug)]
// enum ParticleContainer {
//     Particles([Option<Particle>; CAPACITY]),
//     Divided,
// }
#[derive(Debug)]
pub struct ParticleContainer {
    particles: [Option<Particle>; CAPACITY],
    sub_trees: Vec<Option<Box<QuadTree>>>,
}

impl ParticleContainer {
    pub fn new() -> Self {
        Self {
            particles: [None; CAPACITY],
            sub_trees: Vec::with_capacity(ORTHANT_NUM),
        }
    }
}


#[derive(Debug)]
pub struct QuadTree {
    boundary: Boundary,
    particle_container: ParticleContainer,
}

impl QuadTree {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            boundary,
            particle_container: ParticleContainer::new(),
        }
    }
    pub fn new_sub_tree(&mut self, boundary: Boundary)->Self{
        Self {
            Boundary::new(boundary.center, boundary.width / 2.0, boundary.height / 2.0)
            particle_container: ParticleContainer::new(),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.boundary.draw(draw);
        self.particle_container.particles.iter().for_each(|p| {
            if let Some(p) = p {
                p.draw(draw);
            }
        });
        self.particle_container.sub_trees.iter().for_each(|t| {
            if let Some(t) = t {
                t.draw(draw);
            }
        });
    }

    pub fn update(&mut self) {
        self.particle_container.particles.iter_mut().for_each(|p| {
            if let Some(p) = p {
                p.update();
            }
        });
        self.particle_container.sub_trees.iter_mut().for_each(|t| {
            if let Some(t) = t {
                t.update();
            }
        });
    }

    pub fn insert(&mut self, particle: Particle) -> bool {
        if !self.boundary.contains(&particle) {
            return false;
        }
        if self.particle_container.particles.iter().filter(|p| p.is_some()).count() < CAPACITY {
            self.particle_container.particles.iter_mut().filter(|p| p.is_none()).next().unwrap().replace(particle);
            true
        } else {
            self.subdivide();
            return self.insert(particle);
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let width: f32 = self.boundary.width;
        let height: f32 = self.boundary.height;
        self.particle_container.sub_trees.push(
            Some(Box::new(QuadTree::new(Boundary::new(Point2::new(x - width / 4.0, y - height / 4.0), width / 2.0, height / 2.0)))));
    }

    // self.particle_container = ParticleContainer::Divided; //i think this is bug. all the old particles should be redistributed, here they are killed...
    let north_west = QuadTree::new(Boundary::new(Point2::new(x - width / 4.0, y + height / 4.0), width / 2.0, height / 2.0));
    let north_east = QuadTree::new(Boundary::new(Point2::new(x + width / 4.0, y + height / 4.0), width / 2.0, height / 2.0));
    let south_west = QuadTree::new(Boundary::new(Point2::new(x - width / 4.0, y - height / 4.0), width / 2.0, height / 2.0));
    let south_east = QuadTree::new(Boundary::new(Point2::new(x + width / 4.0, y - height / 4.0), width / 2.0, height / 2.0));
    self .north_west = Some(Box::new(north_west));
    self .north_east = Some(Box::new(north_east));
    self .south_west = Some(Box::new(south_west));
    self .south_east = Some(Box::new(south_east));
}

pub fn query(self, particle: &Particle) -> bool {
    return false;
}
}
