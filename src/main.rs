extern crate nannou;

use nannou::prelude::*;

use drawable::draw_bounding_box;

use crate::drawable::Drawable;
use crate::entity::Entity;
use crate::geometry::BoundingBox;
use crate::particle::Particle;
use crate::quad_tree::QuadTree;

mod constants;
mod drawable;
mod entity;
mod geometry;
mod particle;
mod quad_tree;

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
    let mut qt = QuadTree::new(BoundingBox::from_w_h(
        WINDOW_SIZE as f32,
        WINDOW_SIZE as f32,
    ));

    for _ in 0..5000 {
        qt.insert(Particle::new_random());
    }

    let _window = app
        .new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .mouse_pressed(handle_mouse)
        .mouse_moved(handle_mouse_move)
        .key_pressed(handle_key)
        .build()
        .unwrap();

    Model {
        inspector: get_moused_inspector(app),
        qt,
        draw_particles: true,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.qt.update();
    model.inspector = get_moused_inspector(app);

    let _win_rect = app.window_rect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.qt.draw(&draw, &model);
    draw_bounding_box(model.inspector, &draw, LIGHTCORAL);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn handle_mouse(app: &App, model: &mut Model, _button: MouseButton) {
    model.qt.insert(Particle::new(app.mouse.x, app.mouse.y));
}

fn handle_mouse_move(app: &App, model: &mut Model, _pt: Point2) {
    model.inspector = get_moused_inspector(app);
}

fn get_moused_inspector(app: &App) -> Rect {
    BoundingBox::from_xy_wh(app.mouse.position(), pt2(100.0, 100.0))
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
