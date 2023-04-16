extern crate nannou;

use std::mem::swap;

use nannou::prelude::*;

use QuadTreeChildren::{Leaves, Nodes};

use crate::entity::Entity;
use crate::geometry::{BoundingBox, Positioned};

pub const MAX_LEAVES: usize = 4;

pub enum QuadTreeChildren<Leaf> {
    Leaves(Vec<Leaf>),
    Nodes(Box<[QuadTree<Leaf>; 4]>),
}

pub struct QuadTree<Leaf> {
    pub boundary: BoundingBox,
    pub children: QuadTreeChildren<Leaf>,
}

impl<Leaf> QuadTree<Leaf> where Leaf: Positioned {
    pub fn new(boundary: BoundingBox) -> Self {
        Self {
            boundary,
            children: Leaves(Vec::new()),
        }
    }

    pub fn insert(&mut self, item: Leaf) -> bool where Leaf: Positioned {
        let position = item.position();
        if !self.boundary.contains_point(&position) {
            return false;
        }

        match &mut self.children {
            Leaves(leaves) => {
                leaves.push(item);
                if leaves.len() > MAX_LEAVES {
                    if leaves.iter().filter(|leaf| leaf.position() == position).count() > 1 {
                        println!("Not subdividing node with {} leaves as new leaf repeats an existing position", leaves.len());
                        return true;
                    }
                    let subtrees = self.boundary.subdivide().map(QuadTree::<Leaf>::new);
                    let mut swapped_children = Nodes(Box::new(subtrees));
                    swap(&mut self.children, &mut swapped_children);
                    match swapped_children {
                        Leaves(leaves) => leaves.into_iter().for_each(|leaf| {
                            self.insert(leaf);
                        }),
                        _ => panic!("swapped_children should be Leaves"),
                    }
                    true
                } else { false }
            }
            Nodes(nodes) => {
                for node in nodes.iter_mut() {
                    if node.boundary.contains_point(&position) {
                        return node.insert(item);
                    }
                }
                false
            }
        }
    }
}

impl<Leaf> Positioned for QuadTree<Leaf> {
    fn position(&self) -> Point2 {
        self.boundary.center()
    }
}

impl<Leaf> Entity for QuadTree<Leaf> where Leaf: Entity {
    fn update(&mut self) {
        match &mut self.children {
            Leaves(leaves) => {
                leaves.iter_mut().for_each(|leaf| {
                    leaf.update();
                });
            }
            Nodes(nodes) => {
                nodes.iter_mut().for_each(|node| {
                    node.update();
                });
            }
        }
    }
}

// impl QuadTree {
//
//     pub fn query(&self, range: &Boundary) -> Vec<&Particle> {
//         let mut particles: Vec<&Particle> = Vec::new();
//         if !self.boundary.intersects(range) {
//             return particles;
//         }
//         self.particle_container.particles.iter().for_each(|p| {
//             if let Some(p) = p {
//                 if range.contains(p) {
//                     particles.push(p);
//                 }
//             }
//         });
//         if let Some(sub_trees) = &self.particle_container.sub_trees {
//             sub_trees.iter().for_each(|t| {
//                 particles.append(&mut t.query(range));
//             });
//         }
//         // println!("particles: {:?}", particles);
//         particles
//     }
// }


