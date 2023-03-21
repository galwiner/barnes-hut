extern crate nannou;

use nannou::prelude::*;
use crate::quad_tree::Boundary;
use nannou::rand::{Rng,random_range};
mod quad_tree;



const WINDOW_SIZE:f32=800.0;
fn main() {
    nannou::app(model).update(update).run()
}


struct Model {
    qt : quad_tree::QuadTree
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let mut qt = quad_tree::QuadTree::new(Boundary::new(Point2::new(0.0,0.0),WINDOW_SIZE as f32,WINDOW_SIZE as f32));
    for _ in 0..300 {
        qt.insert(quad_tree::Particle::new_random()); }

    println!("{:?}", qt);
    Model{qt}
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.qt.update();
    model.qt.insert(quad_tree::Particle::new(random_range(-WINDOW_SIZE/4.0,WINDOW_SIZE/4.0),random_range(-WINDOW_SIZE/4.0,WINDOW_SIZE/4.0)));
    let _win_rect = app.window_rect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.qt.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
