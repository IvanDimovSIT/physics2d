use macroquad::{
    color::Color,
    math::Vec2,
    miniquad::window::screen_size,
    shapes::{draw_ellipse, draw_line},
};

use crate::{physics_system::PhysicsSystem, point::Point};

pub struct DrawParams {
    pub point_size: f32,
    pub line_size: f32,
    pub point_color: Color,
    pub line_color: Color,
    pub static_point_color: Color,
}

pub struct Renderer {
    draw_params: DrawParams,
}
impl Renderer {
    pub fn new(draw_params: DrawParams) -> Self {
        Renderer { draw_params }
    }

    fn draw_point(&self, point: &Point, color: Color, screen_size: (f32, f32)) {
        draw_ellipse(
            point.location.x * screen_size.0,
            point.location.y * screen_size.1,
            self.draw_params.point_size * screen_size.0,
            self.draw_params.point_size * screen_size.1,
            0.0,
            color,
        );
    }

    fn draw_constraint(&self, from: Vec2, to: Vec2, screen_size: (f32, f32)) {
        draw_line(
            from.x * screen_size.0,
            from.y * screen_size.1,
            to.x * screen_size.0,
            to.y * screen_size.1,
            self.draw_params.line_size * screen_size.0.max(screen_size.1),
            self.draw_params.line_color,
        );
    }

    pub fn draw(&self, physics_system: &PhysicsSystem) {
        let screen_size = screen_size();
        let points_ids = physics_system.get_points_ids();

        for constraint in physics_system.get_constraints() {
            self.draw_constraint(
                points_ids.get(&constraint.get_point1()).unwrap().location,
                points_ids.get(&constraint.get_point2()).unwrap().location,
                screen_size,
            );
        }

        for point in physics_system.get_points_ids().values() {
            if point.is_static {
                self.draw_point(point, self.draw_params.static_point_color, screen_size);
            } else {
                self.draw_point(point, self.draw_params.point_color, screen_size);
            }
        }
    }

    pub fn get_draw_params(&self) -> &DrawParams {
        &self.draw_params
    }
}
