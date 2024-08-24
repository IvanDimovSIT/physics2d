use macroquad::{
    color::Color,
    math::Vec2,
    shapes::{draw_line, draw_rectangle},
    text::draw_text,
    time::get_fps,
};

use crate::point::Point;

const DEBUG_WINDOW_WIDTH_COEF: f32 = 8.0;
const DEBUG_WINDOW_HEIGHT_COEF: f32 = 2.5;

pub struct UiParams {
    pub paused_text_location: (f32, f32),
    pub paused_text_size: f32,
    pub paused_text_color: Color,

    pub line_size: f32,
    pub line_color: Color,

    pub debug_text_location: (f32, f32),
    pub debug_text_size: f32,
    pub debug_text_color: Color,

    pub debug_point_text_color: Color,
    pub debug_point_text_size: f32,
    pub debug_point_box_color: Color,
    pub debug_point_velocity_line_size: f32,
    pub debug_point_velocity_line_length: f32,
    pub debug_point_velocity_line_color: Color,

    pub speed_text_location: (f32, f32),
    pub speed_text_size: f32,
    pub speed_text_color: Color,
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

    pub fn draw_debug_text(
        &self,
        screen_size: (f32, f32),
        mouse_pos: (f32, f32),
        points: usize,
        constraints: usize,
    ) {
        draw_text(
            &format!("FPS:{}", get_fps()),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1,
            self.params.debug_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_text_color,
        );
        draw_text(
            &format!("X:{:.3} Y:{:.3}", mouse_pos.0, mouse_pos.1,),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1
                + self.params.debug_text_size * screen_size.1,
            self.params.debug_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_text_color,
        );
        draw_text(
            &format!("points:{} constraints:{}", points, constraints),
            self.params.debug_text_location.0 * screen_size.0,
            self.params.debug_text_location.1 * screen_size.1
                + self.params.debug_text_size * screen_size.1 * 2.0,
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

    pub fn draw_point_info(&self, screen_size: (f32, f32), id: u64, point: &Point, at: (f32, f32)) {
        let line_to = point.velocity.normalize_or_zero()
            * self.params.debug_point_velocity_line_length
            + point.location;

        let rect_width =
            self.params.debug_point_text_size * DEBUG_WINDOW_WIDTH_COEF * screen_size.0;
        let rect_height =
            self.params.debug_point_text_size * DEBUG_WINDOW_HEIGHT_COEF * screen_size.1;

        let origin_x = if at.0 * screen_size.0 + rect_width > screen_size.0 {
            screen_size.0 - rect_width
        } else {
            at.0 * screen_size.0
        };
        let origin_y = if at.1 * screen_size.1 + rect_height > screen_size.1 {
            screen_size.1 - rect_height
        } else {
            at.1 * screen_size.1
        };

        draw_line(
            point.location.x * screen_size.0,
            point.location.y * screen_size.1,
            line_to.x * screen_size.0,
            line_to.y * screen_size.1,
            self.params.debug_point_velocity_line_size * screen_size.0.max(screen_size.1),
            self.params.debug_point_velocity_line_color,
        );

        draw_rectangle(
            origin_x,
            origin_y,
            rect_width,
            rect_height,
            self.params.debug_point_box_color,
        );

        draw_text(
            &format!(
                "id:{} X:{:.3} Y:{:.3}",
                id, point.location.x, point.location.y
            ),
            origin_x + self.params.debug_point_text_size * 0.2 * screen_size.0,
            origin_y + self.params.debug_point_text_size * screen_size.1,
            self.params.debug_point_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_point_text_color,
        );

        draw_text(
            &format!("VX:{:.3} VY:{:.3}", point.velocity.x, point.velocity.y),
            origin_x + self.params.debug_point_text_size * 0.2 * screen_size.0,
            origin_y + self.params.debug_point_text_size * screen_size.1 * 2.0,
            self.params.debug_point_text_size * screen_size.0.min(screen_size.1),
            self.params.debug_point_text_color,
        );
    }

    pub fn draw_simulation_speed(&self, screen_size: (f32, f32), speed: f32) {
        draw_text(
            &format!("Speed: X{:.2}", speed),
            self.params.speed_text_location.0 * screen_size.0,
            self.params.speed_text_location.1 * screen_size.1,
            self.params.speed_text_size * screen_size.0.min(screen_size.1),
            self.params.speed_text_color,
        );
    }
}
