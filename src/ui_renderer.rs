use macroquad::{color::Color, math::Vec2, shapes::draw_line, text::draw_text, time::get_fps, ui::Ui};

use crate::point;

pub struct UiParams {
    pub paused_text_location: (f32, f32),
    pub paused_text_size: f32,
    pub paused_text_color: Color,
    pub line_size: f32,
    pub line_color: Color,
    pub debug_text_location: (f32, f32),
    pub debug_text_size: f32,
    pub debug_text_color: Color
}

pub struct UiRenderer {
    params: UiParams,
}
impl UiRenderer {
    pub fn new(params: UiParams) -> Self {
        Self { params }
    }

    pub fn draw_line(&self, point1: Vec2, point2: Vec2, screen_size: (f32, f32)) {
        draw_line(
            point1.x * screen_size.0,
            point1.y * screen_size.1,
            point2.x * screen_size.0,
            point2.y * screen_size.1,
            self.params.line_size * screen_size.0.max(screen_size.1),
            self.params.line_color,
        );
    }

    pub fn draw_debug_text(&self, screen_size: (f32, f32), mouse_pos: (f32, f32), points: usize, constraints: usize) {
        draw_text(
            &format!(
                "FPS:{}",
                get_fps()
            ),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1,
            self.params.debug_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_text_color,
        );
        draw_text(
            &format!(
                "X:{:.3} Y:{:.3}",
                mouse_pos.0,
                mouse_pos.1,
            ),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1 + self.params.debug_text_size * screen_size.1,
            self.params.debug_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_text_color,
        );
        draw_text(
            &format!(
                "points:{} constraints:{}",
                points,
                constraints
            ),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1 + self.params.debug_text_size * screen_size.1 * 2.0,
            self.params.debug_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_text_color,
        );
    }

    pub fn draw_paused_text(&self, screen_size: (f32, f32)) {
        draw_text(
            "Paused",
            self.params.paused_text_location.0 * screen_size.0,
            self.params.paused_text_location.1 * screen_size.1,
            self.params.paused_text_size * screen_size.0.min(screen_size.1),
            self.params.paused_text_color,
        );
    }
}
