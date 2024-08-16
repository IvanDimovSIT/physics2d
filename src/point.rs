use macroquad::math::Vec2;

#[derive(Debug)]
pub struct Point {
    pub location: Vec2,
    pub velocity: Vec2,
    pub is_static: bool,
}
impl Point {
    pub fn new(location: Vec2, velocity: Vec2, is_static: bool) -> Self {
        Self {
            location,
            velocity,
            is_static,
        }
    }
}
