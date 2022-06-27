use macroquad::prelude::{rand, screen_height, screen_width, vec2, Vec2};
use rstar::{PointDistance, RTree, RTreeObject, AABB};

/// Modulus operator to get toroidal world topology
fn modulus<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy,
{
    ((a % b) + b) % b
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub pos: Vec2,
    speed: Vec2,
    coherence: f32,          // how quickly boids want to steer towards other boids
    separation: f32,         // how much a boid tries to avoid other boids
    alignment: f32,          // how much a boid tries to match the speed of other boids
    visual_sight_range: f32, // how far a boid can see
}

impl Boid {
    pub fn random_boid() -> Self {
        let mut s = Self {
            pos: vec2(
                rand::gen_range(0., 1.) * screen_width(),
                rand::gen_range(0., 1.) * screen_height(),
            ),
            speed: vec2(0., 0.),
            coherence: 100.,
            separation: 100.,
            alignment: 100.,
            visual_sight_range: 100.,
        };
        s // returns s? idk just saw the other guy do this too
    }

    pub fn step(&mut self, rtree: &RTree<TreePoint>, move_dir: Option<Vec2>) {
        self.pos += self.speed;
        self.pos.x = modulus(self.pos.x, screen_width());
        self.pos.y = modulus(self.pos.y, screen_height());
    }
}

/// Helper structure used for the rstar geometric data structure. This data structure is used for
/// computing interaction between boids fluidly even with thousands of them
pub struct TreePoint {
    pub x: f64,
    pub y: f64,
    pub idx: usize,
}

impl RTreeObject for TreePoint {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y])
    }
}

impl PointDistance for TreePoint {
    fn distance_2(
        &self,
        point: &<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point,
    ) -> <<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar
    {
        (self.x - point[0]) * (self.x - point[0]) + (self.y - point[1]) * (self.y - point[1])
    }
}
