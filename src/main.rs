use controller::Controller;
use input::get_input;
use macroquad::prelude::*;
use miniquad::window::screen_size;
use physics_system::PhysicsSystem;
use renderer::{DrawParams, Renderer};
use simulator::{SimulationBoundingBox, SimulationParams, Simulator};
use ui_renderer::{UiParams, UiRenderer};

mod constraint;
mod controller;
mod input;
mod physics_system;
mod point;
mod renderer;
mod simulator;
mod ui_renderer;

fn construct_controller() -> Controller {
    let point_size = 0.02;
    let line_size = 0.01;
    let physics_system = PhysicsSystem::new();
    let simulator = Simulator::new(
        SimulationParams {
            air_resistence: 0.999,
            gravity: 0.02,
            point_size,
            spring_coeff: 12.0,
            damping: 4.0,
        },
        SimulationBoundingBox {
            max_x: 1.0,
            max_y: 1.0,
            min_x: 0.0,
            min_y: 0.0,
        },
    );
    let renderer = Renderer::new(DrawParams {
        point_size,
        line_size,
        point_color: Color::from_rgba(255, 0, 0, 255),
        static_point_color: Color::from_rgba(0, 255, 0, 255),
        line_color: Color::from_rgba(255, 0, 0, 255),
    });
    let ui_renderer = UiRenderer::new(UiParams {
        paused_text_location: (0.08, 0.08),
        paused_text_size: 0.08,
        paused_text_color: WHITE,
        line_size,
        line_color: Color::from_rgba(255, 0, 0, 255),
    });

    Controller::new(physics_system, simulator, renderer, ui_renderer)
}

#[macroquad::main("Test")]
async fn main() {
    let mut controller = construct_controller();

    loop {
        clear_background(BLACK);

        let delta = get_frame_time();
        let screen_size = screen_size();

        controller.handle_input(&get_input(screen_size));
        controller.next_step(delta);
        controller.draw_frame();

        let pos = mouse_position();
        draw_text(
            &format!(
                "X:{} Y:{} FPS:{}",
                pos.0 / screen_size.0,
                pos.1 / screen_size.1,
                get_fps()
            ),
            0.05 * screen_size.0,
            0.95 * screen_size.1,
            0.04 * screen_size.1,
            WHITE,
        );
        next_frame().await;
    }
}
