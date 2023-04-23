use nannou::prelude::*;
use MouseScrollDelta::*;

use crate::drawing::{alpha, draw_rect, Drawable};
use crate::physics::Universe;
use crate::simulation::Simulation;
use crate::view_state::ViewState;

pub struct AppModel {
    simulation: Simulation<Universe>,
    view_state: ViewState,
}

const INITIAL_PARTICLE_COUNT: usize = 10000;
const KEYBOARD_PAN_DISTANCE: f32 = 50.0;
const ZOOM_FACTOR: f32 = 1.1;

pub fn init_app(app: &App) -> AppModel {
    app.new_window()
        .size(1200, 800)
        .view(view)
        .event(event_handler)
        .build()
        .unwrap();

    AppModel {
        simulation: Simulation::new(Universe::new(INITIAL_PARTICLE_COUNT)),
        view_state: Default::default(),
    }
}

pub fn update(_app: &App, model: &mut AppModel, _: Update) {
    model.simulation.update();
}

fn view(app: &App, app_model: &AppModel, frame: Frame) {
    let app_draw = app.draw();
    app_draw.background().color(BLACK);
    let sim_draw = app_draw.transform(app_model.view_state.universe_to_app_transform());
    let sim_bounds = app_model.view_state.to_universe_rect(frame.rect());

    let universe = &app_model.simulation.model;
    universe.draw(&sim_draw, sim_bounds, &app_model.view_state);

    if let Some(inspector) = app_model.view_state.inspector_app_bounds() {
        draw_rect(inspector, &app_draw, alpha(LIGHTCORAL, 0.8));
    }
    // Write the result of our drawing to the window's frame.
    app_draw.to_frame(app, &frame).unwrap();
}

fn event_handler(app: &App, model: &mut AppModel, event: WindowEvent) {
    let view = &mut model.view_state;
    let universe = &mut model.simulation.model;

    match event {
        // mouse events:
        MouseMoved(position) => view.inspect_at(position),
        MousePressed(MouseButton::Left) => {
            let universe_position = view.to_universe_point(app.mouse.position());
            universe.add_particle_at(universe_position);
        }
        MouseWheel(LineDelta(x, y), _phase) => {
            view.zoom_at(app.mouse.position(), ZOOM_FACTOR.powf(x + y))
        }
        // key events:
        KeyPressed(Key::Space) => view.cycle_drawn_stuff(),
        KeyPressed(Key::Back /* backspace */) => universe.clear(),
        KeyPressed(Key::P) => universe.add_random_particles(200),
        KeyPressed(Key::R) => {
            view.reset_zoom();
            view.reset_pan();
        }
        KeyPressed(Key::S) => model.simulation.reset_stats(),
        KeyPressed(Key::Up) => view.pan.y -= KEYBOARD_PAN_DISTANCE,
        KeyPressed(Key::Down) => view.pan.y += KEYBOARD_PAN_DISTANCE,
        KeyPressed(Key::Left) => view.pan.x += KEYBOARD_PAN_DISTANCE,
        KeyPressed(Key::Right) => view.pan.x += -KEYBOARD_PAN_DISTANCE,
        _ => {}
    }
}
