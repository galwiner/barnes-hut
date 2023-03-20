extern crate nannou;

use nannou::prelude::*;


const WINDOW_SIZE: f32 = 800.0;
const CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Particle {
    pub fn new(x:f32,y:f32) -> Self {
        let position = Point2::new(x,y);
        let velocity = Vec2::new(0.0, 0.0);
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
            .w_h(10.0, 10.0)
            .rgba(0.0, 1.0, 0.0, 1.0)
            .stroke(rgba(0.0, 1.0, 0.0, 1.0))
            .stroke_weight(2.0);
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

    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.center.x, self.center.y)
            .w_h(self.width, self.height)
            .rgba(0.0, 0.0, 0.0, 1.0)
            .stroke(rgba(1.0, 0.0, 0.0, 1.0))
            .stroke_weight(1.0);
    }
}

#[derive(PartialEq, Debug)]
enum ParticleContainer {
    Particles([Option<Particle>; CAPACITY]),
    Divided,
}

#[derive(Debug)]
pub struct QuadTree {
    boundary: Boundary,
    particle_container: ParticleContainer,
    north_west: Option<Box<QuadTree>>,
    north_east: Option<Box<QuadTree>>,
    south_west: Option<Box<QuadTree>>,
    south_east: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            boundary,
            particle_container: ParticleContainer::Particles(Default::default()),
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }
    pub fn draw(&self, draw: &Draw) {
        self.boundary.draw(draw);

        match self.particle_container {
            ParticleContainer::Particles(particles) => {
                for particle in particles.iter().filter(|p| p.is_some()).map(|p| p.unwrap()) {
                    particle.draw(draw);
                }
            }
            ParticleContainer::Divided => {
                self.north_west.as_ref().unwrap().draw(draw);
                self.north_east.as_ref().unwrap().draw(draw);
                self.south_west.as_ref().unwrap().draw(draw);
                self.south_east.as_ref().unwrap().draw(draw);
            }
        }


    }
    pub fn insert(&mut self, particle: Particle) -> bool {
        if !self.boundary.contains(&particle) {
            return false;
        }

        match self.particle_container {
            ParticleContainer::Particles(ref mut particles) => {
                // let full = particles.iter_mut().all(|p| p.is_some());
                let particle_array = particles.iter_mut().filter(|p| p.is_none()).next();
                match particle_array {
                    Some(p) => {
                        *p = Some(particle);
                        true
                    }
                    None => {
                        self.subdivide();
                        self.insert(particle)
                    }
                }
            }
            ParticleContainer::Divided => {
                let nw = self.north_west.as_mut().unwrap().insert(particle.clone());
                let ne = self.north_east.as_mut().unwrap().insert(particle.clone());
                let sw = self.south_west.as_mut().unwrap().insert(particle.clone());
                let se = self.south_east.as_mut().unwrap().insert(particle.clone());

                if nw || ne || sw || se {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let width: f32 = self.boundary.width;
        let height: f32 = self.boundary.height;
        self.particle_container = ParticleContainer::Divided;
        let north_west = QuadTree::new(Boundary::new(Point2::new(x - width / 4.0, y + height / 4.0), width / 2.0, height / 2.0));
        let north_east = QuadTree::new(Boundary::new(Point2::new(x + width / 4.0, y + height / 4.0), width / 2.0, height / 2.0));
        let south_west = QuadTree::new(Boundary::new(Point2::new(x - width / 4.0, y - height / 4.0), width / 2.0, height / 2.0));
        let south_east = QuadTree::new(Boundary::new(Point2::new(x + width / 4.0, y - height / 4.0), width / 2.0, height / 2.0));
        self.north_west = Some(Box::new(north_west));
        self.north_east = Some(Box::new(north_east));
        self.south_west = Some(Box::new(south_west));
        self.south_east = Some(Box::new(south_east));

    }
}
