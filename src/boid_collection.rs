use crate::boid::{Boid, TreePoint};
use macroquad::prelude::*;
use rstar::RTree;
use std::collections::HashSet;

// a collection of boids
pub struct BoidCollection {
    boids: Vec<Boid>,
}

impl BoidCollection {
    // create some random boids
    pub fn new(n: usize) -> Self {
        let mut s = Self { boids: Vec::new() };
        for _ in 0..n {
            s.boids.push(Boid::random_boid());
        }
        s // why do people always do this?
    }

    // compute one step of the simulation
    pub fn step(&mut self) {
        let mut new: Vec<Boid> = Vec::new();
        let tree: RTree<TreePoint> = RTree::bulk_load(
            self.boids
                .iter()
                .enumerate()
                .map(|(n, biot)| TreePoint {
                    x: biot.pos.x as f64,
                    y: biot.pos.y as f64,
                    idx: n,
                })
                .collect(),
        );
        // move the boids according to their rules
        for n in 0..(self.boids.len()) {
            let mut move_dir = Some(vec2(self.boids[n].pos.x, self.boids[n].pos.y).normalize());
            let off = self.boids[n].step(&tree, move_dir);
        }
    }

    pub fn draw(&self) {
        for boid in self.boids.iter() {
            draw_circle(boid.pos.x, boid.pos.y, 7., BLUE); // I know these should be triangles but just doing dots for now
        }
    }

    pub fn len(&self) -> usize {
        return self.boids.len();
    }
}
