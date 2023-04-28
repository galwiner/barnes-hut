use async_std::task;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use task::block_on;
use MouseScrollDelta::*;

use crate::drawing::{alpha, draw_rect, Drawable};
use crate::physics::Universe;
use crate::simulation::Simulation;
use crate::view_state::ViewState;

struct AppModel {
    simulation: Simulation<Universe>,
    view_state: ViewState,
}

const INITIAL_PARTICLE_COUNT: usize = 1000;
const KEYBOARD_PAN_DISTANCE: f32 = 50.0;
const ZOOM_FACTOR: f32 = 1.1;

pub fn run_sync() {
    block_on(run_async());
}

pub async fn run_async() {
    app::Builder::new_async(|app| Box::new(init_nannou_app(app)))
        .update(update)
        .backends(Backends::PRIMARY | Backends::GL)
        .run_async()
        .await;
}

async fn init_nannou_app(app: &App) -> AppModel {
    create_window(app).await;
    AppModel {
        simulation: Simulation::new(Universe::new(INITIAL_PARTICLE_COUNT)),
        view_state: Default::default(),
    }
}

async fn create_window(app: &App) {
    app.new_window()
        .title("Barnes-Hut Simulation")
        //.size(1200, 800)
        .view(view)
        .event(event_handler)
        .device_descriptor(DeviceDescriptor {
            limits: Limits {
                max_texture_dimension_2d: 8192,
                ..Limits::downlevel_webgl2_defaults()
            },
            ..Default::default()
        })
        .build_async()
        .await
        .unwrap();
}

fn update(_app: &App, model: &mut AppModel, _: Update) {
    model.simulation.update();
}

fn view(app: &App, app_model: &AppModel, frame: Frame) {
    let app_draw = app.draw();
    app_draw.background().color(BLACK);
    let sim_draw = app_draw.transform(app_model.view_state.universe_to_app_transform());
    let sim_bounds = app_model.view_state.as_universe_rect(frame.rect());

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
    let mouse = &app.mouse;

    match event {
        // mouse events:
        MouseMoved(position) => {
            if mouse.buttons.middle().is_down() {
                view.mouse_pan(mouse.position());
            }
            view.inspect_at(position);
        }
        MouseReleased(MouseButton::Middle) => view.end_mouse_pan(),
        MousePressed(MouseButton::Left) => {
            let universe_position = view.as_universe_point(app.mouse.position());
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
