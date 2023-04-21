#[macro_use]
extern crate derivative;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nannou;
#[cfg(feature = "parallel")]
extern crate rayon;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

use log::LevelFilter::{Debug, Warn};
use nannou::prelude::*;

use view_state::ViewState;

use crate::drawing::{alpha, draw_rect, Drawable};
use crate::physics::Universe;
use crate::simulation::Simulation;

#[macro_use]
mod macros;
mod created;
mod drawing;
mod physics;
mod quad_tree;
mod simulation;
mod view_state;

fn main() {
    env_logger::Builder::new()
        .filter_level(Warn)
        .filter(Some(module_path!()), Debug)
        .parse_default_env()
        .init();
    nannou::app(init_app).update(update).run()
}

struct AppModel {
    simulation: Simulation<Universe>,
    view_state: ViewState,
}

impl AppModel {
    fn univ(&self) -> &Universe {
        &self.simulation.model
    }

    fn univ_m(&mut self) -> &mut Universe {
        &mut self.simulation.model
    }
}

const INITIAL_PARTICLE_COUNT: usize = 1000;

fn init_app(app: &App) -> AppModel {
    app.new_window()
        .size(1200, 800)
        .view(view)
        .mouse_pressed(on_mouse_pressed)
        .mouse_moved(on_mouse_moved)
        .key_pressed(on_key_pressed)
        .build()
        .unwrap();

    AppModel {
        simulation: Simulation::new(Universe::new(INITIAL_PARTICLE_COUNT)),
        view_state: ViewState::new(),
    }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.univ().draw(&draw, frame.rect(), &model.view_state);
    if let Some(inspector) = model.view_state.inspector {
        draw_rect(inspector, &draw, alpha(LIGHTCORAL, 0.8));
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut AppModel, _: Update) {
    model.simulation.update();
}

fn on_mouse_pressed(app: &App, model: &mut AppModel, _button: MouseButton) {
    model.univ_m().add_particle_at(app.mouse.position());
}

fn on_mouse_moved(_app: &App, model: &mut AppModel, position: Point2) {
    model.view_state.inspect_at(position);
}

fn on_key_pressed(_app: &App, model: &mut AppModel, key: Key) {
    match key {
        Key::Space => {
            model.view_state.toggle_draw_particles();
        }
        Key::Back /* backspace */ => {
            model.univ_m().clear();
        }
        Key::P => {
            model.univ_m().add_random_particles(200);
        }
        Key::R => {
            model.simulation.reset_stats();
        }
        _ => {}
    }
}
