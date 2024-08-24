use std::{error::Error, fs::read_to_string};

use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::{
    controller::Controller,
    physics_system::PhysicsSystem,
    renderer::{DrawParams, Renderer},
    simulator::{SimulationBoundingBox, SimulationParams, Simulator},
    ui_renderer::{UiParams, UiRenderer},
};

#[derive(Debug, Serialize, Deserialize)]
struct SimulationConfig {
    air_resistence: f32,
    gravity: f32,
    point_size: f32,
    spring_coeff: f32,
    damping: f32,
    collision_force: f32,
    push_from_sides_force: f32,
}
impl Into<SimulationParams> for SimulationConfig {
    fn into(self) -> SimulationParams {
        SimulationParams {
            gravity: self.gravity,
            air_resistence: self.air_resistence,
            point_size: self.point_size,
            spring_coeff: self.spring_coeff,
            damping: self.damping,
            collision_force: self.collision_force,
            push_from_sides_force: self.push_from_sides_force,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundingBoxConfig {
    max_x: f32,
    max_y: f32,
    min_x: f32,
    min_y: f32,
}
impl Into<SimulationBoundingBox> for BoundingBoxConfig {
    fn into(self) -> SimulationBoundingBox {
        SimulationBoundingBox {
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: self.min_y,
            max_y: self.max_y,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RendererConfig {
    bg_color: [u8; 4],
    point_size: f32,
    line_size: f32,
    point_color: [u8; 4],
    point_border_color: [u8; 4],
    static_point_color: [u8; 4],
    line_color: [u8; 4],
    stressed_line_color: [u8; 4],
}
impl Into<DrawParams> for RendererConfig {
    fn into(self) -> DrawParams {
        DrawParams {
            bg_color: Color::from_rgba(
                self.bg_color[0],
                self.bg_color[1],
                self.bg_color[2],
                self.bg_color[3],
            ),
            point_size: self.point_size,
            line_size: self.line_size,
            point_color: Color::from_rgba(
                self.point_color[0],
                self.point_color[1],
                self.point_color[2],
                self.point_color[3],
            ),
            point_border_color: Color::from_rgba(
                self.point_border_color[0],
                self.point_border_color[1],
                self.point_border_color[2],
                self.point_border_color[3],
            ),
            static_point_color: Color::from_rgba(
                self.static_point_color[0],
                self.static_point_color[1],
                self.static_point_color[2],
                self.static_point_color[3],
            ),
            line_color: Color::from_rgba(
                self.line_color[0],
                self.line_color[1],
                self.line_color[2],
                self.line_color[3],
            ),
            stressed_line_color: Color::from_rgba(
                self.stressed_line_color[0],
                self.stressed_line_color[1],
                self.stressed_line_color[2],
                self.stressed_line_color[3],
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UiRendererConfig {
    paused_text_location: [f32; 2],
    paused_text_size: f32,
    paused_text_color: [u8; 4],
    line_size: f32,
    line_color: [u8; 4],
    debug_text_location: [f32; 2],
    debug_text_size: f32,
    debug_text_color: [u8; 4],
    debug_point_text_color: [u8; 4],
    debug_point_text_size: f32,
    debug_point_box_color: [u8; 4],
    debug_point_velocity_line_size: f32,
    debug_point_velocity_line_color: [u8; 4],
    debug_point_velocity_line_length: f32,
    speed_text_location: [f32; 2],
    speed_text_size: f32,
    speed_text_color: [u8; 4],
}
impl Into<UiParams> for UiRendererConfig {
    fn into(self) -> UiParams {
        UiParams {
            paused_text_location: (self.paused_text_location[0], self.paused_text_location[1]),
            paused_text_size: self.paused_text_size,
            paused_text_color: Color::from_rgba(
                self.paused_text_color[0],
                self.paused_text_color[1],
                self.paused_text_color[2],
                self.paused_text_color[3],
            ),
            line_size: self.line_size,
            line_color: Color::from_rgba(
                self.line_color[0],
                self.line_color[1],
                self.line_color[2],
                self.line_color[3],
            ),
            debug_text_location: (self.debug_text_location[0], self.debug_text_location[1]),
            debug_text_size: self.debug_text_size,
            debug_text_color: Color::from_rgba(
                self.debug_text_color[0],
                self.debug_text_color[1],
                self.debug_text_color[2],
                self.debug_text_color[3],
            ),
            debug_point_text_color: Color::from_rgba(
                self.debug_point_text_color[0],
                self.debug_point_text_color[1],
                self.debug_point_text_color[2],
                self.debug_point_text_color[3],
            ),
            debug_point_text_size: self.debug_point_text_size,
            debug_point_box_color: Color::from_rgba(
                self.debug_point_box_color[0],
                self.debug_point_box_color[1],
                self.debug_point_box_color[2],
                self.debug_point_box_color[3],
            ),
            debug_point_velocity_line_size: self.debug_point_velocity_line_size,
            debug_point_velocity_line_color: Color::from_rgba(
                self.debug_point_velocity_line_color[0],
                self.debug_point_velocity_line_color[1],
                self.debug_point_velocity_line_color[2],
                self.debug_point_velocity_line_color[3],
            ),
            debug_point_velocity_line_length: self.debug_point_velocity_line_length,
            speed_text_location: (self.speed_text_location[0], self.speed_text_location[1]),
            speed_text_size: self.speed_text_size,
            speed_text_color: Color::from_rgba(
                self.speed_text_color[0],
                self.speed_text_color[1],
                self.speed_text_color[2],
                self.speed_text_color[3],
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    bounding_box_config: BoundingBoxConfig,
    simulation_config: SimulationConfig,
    renderer_config: RendererConfig,
    ui_renderer_config: UiRendererConfig,
}
impl Config {
    pub fn load(filepath: &str) -> Result<Self, Box<dyn Error>> {
        Ok(from_str(&read_to_string(filepath)?)?)
    }
}
impl Into<Controller> for Config {
    fn into(self) -> Controller {
        Controller::new(
            PhysicsSystem::new(),
            Simulator::new(
                self.simulation_config.into(),
                self.bounding_box_config.into(),
            ),
            Renderer::new(self.renderer_config.into()),
            UiRenderer::new(self.ui_renderer_config.into()),
        )
    }
}
