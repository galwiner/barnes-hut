extern crate nannou;

use std::fmt;
use std::ops::Range;
use nannou::draw::properties::ColorScalar;
use nannou::rand;
use nannou::prelude::*;
use rand_distr::{Distribution, Normal};
use crate::Model;

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
    radius: f32,
    pub(crate) color: [ColorScalar; 4],
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Point2::new(x, y);
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 10.0;
        let radius = 0.1;
        let color = [0.0, 1.0, 0.0, 1.0];
        Self {
            position,
            velocity,
            acceleration,
            mass,
            radius,
            color,
        }
    }
    pub fn new_random() -> Self {
        let normal = Normal::new(0.0, WINDOW_SIZE / 10.0).unwrap();
        let random_sample = || normal.sample(&mut rand::thread_rng());
        let position = Point2::new(random_sample(), random_sample());
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 10.0;
        let radius = 0.1;
        let color = [0.0, 1.0, 0.0, 1.0];
        Self {
            position,
            velocity,
            acceleration,
            mass,
            radius,
            color,
        }
    }
    pub fn set_color(&mut self, color: [ColorScalar; 4]) {
        self.color = color;
    }
    pub fn draw(&self, draw: &Draw) {
        // println!("color: {:?}", self.color);
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(self.radius * 50.0, self.radius * 50.0)
            .rgba(self.color[0], self.color[1], self.color[2], self.color[3])
            .stroke(rgba(0.0, 0.0, 0.0, 1.0));
    }
    pub fn update(&mut self) {
        // self.position += self.velocity;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundary {
    center: Point2,
    width: f32,
    height: f32,
}

// #[derive(Debug, Clone)]
// pub struct Boundary2 {
//     x_range: Range<f32>,
//     y_range: Range<f32>,
// }
//
// impl Boundary2 {
//     pub fn contains(self, particle: &Particle) -> bool {
//         self.x_range.contains(&particle.position.x) && self.y_range.contains(&particle.position.y)
//     }
// }

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
    pub fn update(&mut self, center: Point2) {
        self.center = center;
    }
    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.center.x, self.center.y)
            .w_h(self.width, self.height)
            .rgba(0.0, 0.0, 0.0, 0.0)
            .stroke(rgba(1.0, 0.0, 0.0, 1.0))
            .stroke_weight(1.0);
    }
}


#[derive(Debug)]
pub struct ParticleContainer {
    particles: [Option<Particle>; CAPACITY],
    sub_trees: Option<Vec<Box<QuadTree>>>,
}

impl ParticleContainer {
    pub fn new() -> Self {
        Self {
            particles: [None; CAPACITY],
            sub_trees: None,
        }
    }
}


#[derive(Debug)]
pub struct QuadTree {
    pub boundary: Boundary,
    pub particle_container: ParticleContainer,
}

impl fmt::Display for QuadTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.boundary.center.x, self.boundary.center.y)
    }
}

impl QuadTree {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            boundary,
            particle_container: ParticleContainer::new(),
        }
    }

    pub fn draw(&self, draw: &Draw, model: &Model) {
        self.boundary.draw(draw);
        if model.draw_particles {
            self.particle_container.particles.iter().for_each(|p| {
                if let Some(p) = p {
                    p.draw(draw);
                }
            });
        }

        self.particle_container.sub_trees.iter().for_each(|t| {
            t.iter().for_each(|t| {
                // println!("draw sub tree");
                t.draw(draw, model);
            });
        });
    }

    pub fn update(&mut self) {
        self.particle_container.particles.iter_mut().for_each(|p| {
            if let Some(p) = p {
                p.update();
            }
        });
        self.particle_container.sub_trees.iter_mut().for_each(|t| {
            t.iter_mut().for_each(|t| {
                t.update();
            });
        });
    }

    pub fn insert(&mut self, particle: Particle) -> bool {
        if !self.boundary.contains(&particle) {
            return false;
        }

        if self.particle_container.particles.iter().filter(|p| p.is_some()).count() < CAPACITY {
            fix_overlapping_particles(&mut self.particle_container.particles, &particle);
            self.particle_container.particles.iter_mut().filter(|p| p.is_none()).next().unwrap().replace(particle);
            true
        } else {
            return if self.particle_container.sub_trees.is_none() {
                self.subdivide();
                self.insert(particle)
            } else {
                self.particle_container.sub_trees
                    .iter_mut().map(|t| t.iter_mut().map(|t| t.insert(particle)).any(|r| r))
                    .any(|r| r)
            };
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let width: f32 = self.boundary.width;
        let height: f32 = self.boundary.height;
        let signs = [-1.0, 1.0];
        let quadrants = signs.iter().flat_map(|x| signs.iter().map(move |y| (x, y)));
        let sub_trees: Vec<Box<QuadTree>> = quadrants
            .map(|(s1, s2)| Box::new(QuadTree::new(Boundary::new(Point2::new(x + s1 * width / 4.0, y + s2 * height / 4.0), width / 2.0, height / 2.0))))
            .collect();

        self.particle_container.sub_trees = Some(sub_trees);
        self.particle_container.particles.iter_mut().for_each(|p| {
            if let Some(p) = p {
                self.particle_container.sub_trees.iter_mut().map(|t| t.iter_mut().map(|t| t.insert(p.clone())).any(|r| r)).any(|r| r);
            }
        });
    }

    pub fn query(&self, range: &Boundary) -> Vec<&Particle> {
        let mut particles: Vec<&Particle> = Vec::new();
        if !self.boundary.intersects(range) {
            return particles;
        }
        self.particle_container.particles.iter().for_each(|p| {
            if let Some(p) = p {
                if range.contains(p) {
                    particles.push(p);
                }
            }
        });
        if let Some(sub_trees) = &self.particle_container.sub_trees {
            sub_trees.iter().for_each(|t| {
                particles.append(&mut t.query(range));
            });
        }
        // println!("particles: {:?}", particles);
        particles
    }

    pub fn mutate_each_in_range<F>(&mut self, range: &Boundary, mut operation: &mut F)
        where F: FnMut(&mut Particle)
    {
        if self.boundary.intersects(range) {
            self.particle_container.particles.iter_mut().flatten()
                .filter(|particle| range.contains(particle))
                .for_each(&mut operation);

            self.particle_container.sub_trees.iter_mut().flatten()
                .for_each(|subtree| {
                    subtree.mutate_each_in_range(range, operation);
                });
        }
    }
}

fn fix_overlapping_particles(particle_container: &mut [Option<Particle>; 4], particle: &Particle) {
    particle_container.iter_mut().for_each(|p| {
        if let Some(p) = p {
            if p.position == particle.position {
                p.position.x += p.radius;
                p.position.y += p.radius;
            }
        }
    });
}



