extern crate nannou;

use nannou::prelude::*;

use crate::drawable::{draw_bounding_box, Drawable};
use crate::entity::Entity;
use crate::geometry::BoundingBox;
use crate::particle::Particle;
use crate::quad_tree::QuadTree;

mod quad_tree;
mod particle;
mod constants;
mod geometry;
mod entity;
mod drawable;


const WINDOW_SIZE: u32 = 800;

fn main() {
    nannou::app(model).update(update).run()
}


pub struct Model {
    inspector: BoundingBox,
    qt: QuadTree<Particle>,
    draw_particles: bool,
}

fn model(app: &App) -> Model {
    let inspector = BoundingBox::about_point(&app.mouse.position(), 100.0);
    let mut qt = QuadTree::new(BoundingBox::about_point(&Point2::new(0.0, 0.0), WINDOW_SIZE as f32));
    let _window = app.new_window().size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .mouse_pressed(handle_mouse)
        .mouse_moved(handle_mouse_move)
        .key_pressed(handle_key)
        .build().unwrap();
    for _ in 0..5000 {
        qt.insert(particle::Particle::new_random());
    }

    Model { inspector, qt, draw_particles: true }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.qt.update();
    update_inspector(app, model);

    let _win_rect = app.window_rect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.qt.draw(&draw, &model);
    draw_bounding_box(&model.inspector, &draw, LIGHTCORAL);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn handle_mouse(app: &App, model: &mut Model, _button: MouseButton) {
    model.qt.insert(particle::Particle::new(app.mouse.x, app.mouse.y));
}

fn handle_mouse_move(app: &App, model: &mut Model, _pt: Point2) {
    update_inspector(app, model);
}

fn update_inspector(app: &App, model: &mut Model) {
    model.inspector = BoundingBox::about_point(&app.mouse.position(), 100.0);
}

fn handle_key(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.draw_particles = !model.draw_particles;
            println!("draw_particles: {}", model.draw_particles)
        }
        _ => {}
    }
}
