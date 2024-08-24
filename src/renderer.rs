use macroquad::{
    color::Color,
    math::Vec2,
    miniquad::window::screen_size,
    shapes::{draw_ellipse, draw_line}, window::clear_background,
};

use crate::{physics_system::PhysicsSystem, point::Point};

const POINT_BORDER_SIZE: f32 = 0.2;

pub struct DrawParams {
    pub bg_color: Color,
    pub point_size: f32,
    pub line_size: f32,
    pub point_color: Color,
    pub point_border_color: Color,
    pub line_color: Color,
    pub stressed_line_color: Color,
    pub static_point_color: Color,
}

pub struct Renderer {
    draw_params: DrawParams,
}
impl Renderer {
    pub fn new(draw_params: DrawParams) -> Self {
        Renderer { draw_params }
    }

    fn draw_point(&self, point: &Point, screen_size: (f32, f32)) {
        let inner_color = if point.is_static {
            self.draw_params.static_point_color
        } else {
            self.draw_params.point_color
        };

        draw_ellipse(
            point.location.x * screen_size.0,
            point.location.y * screen_size.1,
            self.draw_params.point_size * screen_size.0,
            self.draw_params.point_size * screen_size.1,
            0.0,
            self.draw_params.point_border_color,
        );
        draw_ellipse(
            point.location.x * screen_size.0,
            point.location.y * screen_size.1,
            self.draw_params.point_size * screen_size.0 * (1.0 - POINT_BORDER_SIZE),
            self.draw_params.point_size * screen_size.1 * (1.0 - POINT_BORDER_SIZE),
            0.0,
            inner_color,
        );
    }

    fn draw_constraint(
        &self,
        constraint_length: f32,
        from: Vec2,
        to: Vec2,
        screen_size: (f32, f32),
    ) {
        let points_distance = from.distance(to);
        let constraint_stress =
            ((points_distance - constraint_length).abs() / constraint_length).min(1.0);

        let line_color = Color::new(
            self.draw_params.line_color.r * (1.0 - constraint_stress)
                + self.draw_params.stressed_line_color.r * constraint_stress,
            self.draw_params.line_color.g * (1.0 - constraint_stress)
                + self.draw_params.stressed_line_color.g * constraint_stress,
            self.draw_params.line_color.b * (1.0 - constraint_stress)
                + self.draw_params.stressed_line_color.b * constraint_stress,
            self.draw_params.line_color.a * (1.0 - constraint_stress)
                + self.draw_params.stressed_line_color.a * constraint_stress,
        );

        draw_line(
            from.x * screen_size.0,
            from.y * screen_size.1,
            to.x * screen_size.0,
            to.y * screen_size.1,
            self.draw_params.line_size * screen_size.0.max(screen_size.1),
            line_color,
        );
    }

    pub fn draw(&self, physics_system: &PhysicsSystem) {
        clear_background(self.draw_params.bg_color);
        let screen_size = screen_size();
        let points_ids = physics_system.get_points_ids();

        for constraint in physics_system.get_constraints() {
            self.draw_constraint(
                constraint.get_distance(),
                points_ids.get(&constraint.get_point1()).unwrap().location,
                points_ids.get(&constraint.get_point2()).unwrap().location,
                screen_size,
            );
        }

        for point in physics_system.get_points_ids().values() {
            self.draw_point(point, screen_size);
        }
    }

    pub fn get_draw_params(&self) -> &DrawParams {
        &self.draw_params
    }
}
