use macroquad::prelude::*;

mod biot;
mod biot_collection;
mod boid;
mod boid_collection;

use biot_collection::BiotCollection;
use boid_collection::BoidCollection;

fn window_conf() -> Conf {
    Conf {
        window_title: "Life".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    rand::srand(miniquad::date::now().to_bits());
    let mut biots = BiotCollection::new(600); // looks like a grid of 600x600 biots
    let mut boids = BoidCollection::new(600); // looks like a grid of 600x600 boids

    loop {
        // biots.step();
        boids.step();
        clear_background(Color::new(0., 0., 0.1, 1.0));
        // biots.draw();
        boids.draw();
        draw_text(
            &format!("FPS: {}, boids: {}", get_fps(), boids.len()),
            screen_width() - 200.,
            screen_height() - 5.,
            18.,
            LIGHTGRAY,
        );
        next_frame().await
    }
}
