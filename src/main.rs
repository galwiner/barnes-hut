#[macro_use]
extern crate derivative;
extern crate nannou;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

use nannou::prelude::*;

use view_state::ViewState;

use crate::drawing::{alpha, draw_rect};
use crate::simulation::Simulation;

mod drawing;
mod geometry;
mod quad_tree;
mod simulation;
mod view_state;

fn main() {
    nannou::app(app_init).update(update).run()
}

struct AppModel {
    simulation: Simulation,
    view_state: ViewState,
}

fn app_init(app: &App) -> AppModel {
    app.new_window()
        .size(1200, 800)
        .view(view)
        .mouse_pressed(on_mouse_pressed)
        .mouse_moved(on_mouse_moved)
        .key_pressed(on_key_pressed)
        .build()
        .unwrap();

    let mut simulation = Simulation::new();
    simulation.add_random_particles(1000);
    AppModel {
        simulation: simulation,
        view_state: ViewState::new(),
    }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.simulation.draw(&draw, frame.rect(), &model.view_state);
    if let Some(inspector) = model.view_state.inspector {
        draw_rect(inspector, &draw, alpha(LIGHTCORAL, 0.8));
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut AppModel, update: Update) {
    model.simulation.update(update);
}

fn on_mouse_pressed(app: &App, model: &mut AppModel, _button: MouseButton) {
    model.simulation.add_particle_at(app.mouse.position());
}

fn on_mouse_moved(_app: &App, model: &mut AppModel, position: Point2) {
    model.view_state.inspect_at(position);
}

fn on_key_pressed(_app: &App, model: &mut AppModel, key: Key) {
    match key {
        Key::Space => {
            model.view_state.toggle_draw_particles();
        }
        _ => {}
    }
}
