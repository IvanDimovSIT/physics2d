pub struct Constraint {
    point1: u64,
    point2: u64,
    distance: f32,
}
impl Constraint {
    pub fn new(point1: u64, point2: u64, distance: f32) -> Self {
        Self {
            point1,
            point2,
            distance,
        }
    }

    pub fn get_point1(&self) -> u64 {
        self.point1
    }

    pub fn get_point2(&self) -> u64 {
        self.point2
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }
}
