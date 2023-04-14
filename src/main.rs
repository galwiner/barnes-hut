extern crate nannou;

use nannou::prelude::*;
use nannou::rand::Rng;

use crate::quad_tree::Boundary;

mod quad_tree;


const WINDOW_SIZE:f32=800.0;

fn main() {
    nannou::app(model).update(update).run()
}


struct Model {
    inspector: Boundary,
    qt : quad_tree::QuadTree
}

fn model(app: &App) -> Model {
    let inspector = Boundary::new(Point2::new(app.mouse.x,app.mouse.y),100 as f32,100 as f32);
    let mut qt = quad_tree::QuadTree::new(Boundary::new(Point2::new(0.0,0.0),WINDOW_SIZE as f32,WINDOW_SIZE as f32));
    let _window = app.new_window().size(800, 800)
        .view(view)
        .mouse_pressed(handle_mouse)
        .mouse_moved(handle_mouse_move)
        .key_pressed(handle_key)
        .build().unwrap();
    for _ in 0..100{
        qt.insert(quad_tree::Particle::new_random()); }

    Model{inspector,qt}
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.qt.update();
    model.inspector.update(Point2::new(app.mouse.x,app.mouse.y));

    let _win_rect = app.window_rect();
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
     draw.background().color(BLACK);
    model.qt.draw(&draw);
    model.inspector.draw(&draw);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
fn handle_mouse(app: &App, model: &mut Model, button: MouseButton) {
    model.qt.insert(quad_tree::Particle::new(app.mouse.x,app.mouse.y));
}

fn handle_mouse_move(app: &App,model:&mut Model, _pt: Point2){
    // println!("mouse x: {}, mouse y: {}", app.mouse.x, app.mouse.y);
    model.inspector.update(Point2::new(app.mouse.x,app.mouse.y));
    let highlighted = model.qt.query(&model.inspector);
    highlighted.iter().for_each(|p| p.set_color([1.0,0.0,0.0,1.0]));
}

fn handle_key(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.qt.draw_particles = !model.qt.draw_particles;
            println!("draw_particles: {}", model.qt.draw_particles)
        }
        _ => {}
    }
}