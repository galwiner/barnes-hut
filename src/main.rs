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

use log::LevelFilter::{Debug, Error, Warn};
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
        .filter_module(module_path!(), Debug)
        .filter_module("wgpu_hal::dx12::instance", Error)
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
        .mouse_wheel(on_mouse_wheel)
        .key_pressed(on_key_pressed)
        .build()
        .unwrap();

    AppModel {
        simulation: Simulation::new(Universe::new(INITIAL_PARTICLE_COUNT)),
        view_state: Default::default(),
    }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let app_draw = app.draw();
    app_draw.background().color(BLACK);

    let sim_draw = app_draw.transform(model.view_state.universe_to_app_transform());
    let sim_bounds = model.view_state.to_universe_rect(frame.rect());
    model.univ().draw(&sim_draw, sim_bounds, &model.view_state);

    if let Some(inspector) = model.view_state.inspector_app_bounds() {
        draw_rect(inspector, &app_draw, alpha(LIGHTCORAL, 0.8));
    }
    // Write the result of our drawing to the window's frame.
    app_draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut AppModel, _: Update) {
    model.simulation.update();
}

fn on_mouse_pressed(app: &App, model: &mut AppModel, button: MouseButton) {
    match button {
        MouseButton::Left => {
            let universe_position = model.view_state.to_universe_point(app.mouse.position());
            model.univ_m().add_particle_at(universe_position);
        }
        _ => {}
    }
}

fn on_mouse_moved(_app: &App, model: &mut AppModel, position: Point2) {
    model.view_state.inspect_at(position);
}

fn on_key_pressed(_app: &App, model: &mut AppModel, key: Key) {
    const PAN_DISTANCE: f32 = 50.0;
    match key {
        Key::Space => {
            model.view_state.cycle_drawn_stuff();
        }
        Key::Back /* backspace */ => {
            model.univ_m().clear();
        }
        Key::P => {
            model.univ_m().add_random_particles(200);
        }
        Key::R => {
            model.view_state.reset_zoom();
            model.view_state.reset_pan();
        }
        Key::S => {
            model.simulation.reset_stats();
        }
        Key::Up => {
            model.view_state.pan.y -= PAN_DISTANCE;
        }
        Key::Down => {
            model.view_state.pan.y += PAN_DISTANCE;
        }
        Key::Left => {
            model.view_state.pan.x += PAN_DISTANCE;
        }
        Key::Right => {
            model.view_state.pan.x += -PAN_DISTANCE;
        }
        _ => {}
    }
}

fn on_mouse_wheel(app: &App, model: &mut AppModel, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            const ZOOM_FACTOR: f32 = 1.1;
            model
                .view_state
                .zoom_at(app.mouse.position(), ZOOM_FACTOR.powf(y));
        }
        MouseScrollDelta::PixelDelta(_position) => {}
    }
}
