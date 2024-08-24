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
    let point_size = 0.015;
    let line_size = 0.005;
    let physics_system = PhysicsSystem::new();
    let simulator = Simulator::new(
        SimulationParams {
            air_resistence: 0.96,
            gravity: 0.09,
            point_size,
            spring_coeff: 550.0,
            damping: 6.0,
            collision_force: 100.0,
            push_from_sides_force: 0.02,
        },
        SimulationBoundingBox {
            max_x: 1.0,
            max_y: 1.0,
            min_x: 0.0,
            min_y: 0.0,
        },
    );
    let renderer = Renderer::new(DrawParams {
        bg_color: Color::from_rgba(10, 10, 40, 255),
        point_size,
        line_size,
        point_color: Color::from_rgba(50, 255, 50, 255),
        point_border_color: Color::from_rgba(255, 255, 255, 255),
        static_point_color: Color::from_rgba(255, 50, 50, 255),
        line_color: Color::from_rgba(255, 255, 255, 255),
        stressed_line_color: Color::from_rgba(255, 0, 0, 255),
    });
    let ui_renderer = UiRenderer::new(UiParams {
        paused_text_location: (0.45, 0.08),
        paused_text_size: 0.08,
        paused_text_color: WHITE,
        line_size,
        line_color: Color::from_rgba(255, 255, 255, 100),
        debug_text_location: (0.01, 0.05),
        debug_text_size: 0.05,
        debug_text_color: Color::from_rgba(255, 255, 255, 160),
        debug_point_text_color: Color::from_rgba(230, 230, 230, 255),
        debug_point_text_size: 0.04,
        debug_point_box_color: Color::from_rgba(50, 50, 50, 200),
        debug_point_velocity_line_size: 0.002,
        debug_point_velocity_line_color: Color::from_rgba(255, 40, 40, 255),
        debug_point_velocity_line_length: 0.06,
        speed_text_location: (0.01, 0.98),
        speed_text_size: 0.04,
        speed_text_color: WHITE,
    });

    Controller::new(physics_system, simulator, renderer, ui_renderer)
}

#[macroquad::main("Physics")]
async fn main() {
    let mut controller = construct_controller();

    loop {
        let delta = get_frame_time();
        let screen_size = screen_size();

        controller.handle_input(&get_input(screen_size), delta);
        controller.next_step(delta);
        controller.draw_frame();

        next_frame().await;
    }
}
