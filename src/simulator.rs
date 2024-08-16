use std::{collections::HashSet, ops::Deref};

use macroquad::{math::{vec2, Vec2}, rand::ChooseRandom};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    physics_system::{self, PhysicsSystem},
    point,
    renderer::DrawParams,
};

pub struct SimulationParams {
    pub gravity: f32,
    pub air_resistence: f32,
    pub point_size: f32,
}
pub struct SimulationBoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

pub struct Simulator {
    params: SimulationParams,
    bounding_box: SimulationBoundingBox,
}
impl Simulator {
    pub fn new(params: SimulationParams, bounding_box: SimulationBoundingBox) -> Self {
        Self {
            params,
            bounding_box,
        }
    }

    fn apply_gravity(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        for point in physics_system.get_points_mut() {
            if point.is_static {
                continue;
            }
            point.velocity.y += self.params.gravity * delta;
        }
    }

    fn apply_velocity(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        for point in physics_system.get_points_mut() {
            if point.is_static {
                point.velocity = vec2(0.0, 0.0);
            } else {
                point.location += point.velocity * self.params.air_resistence * delta;
            }
        }
    }

    fn fit_in_screen(&self, physics_system: &mut PhysicsSystem) {
        for point in physics_system.get_points_mut() {
            if point.location.x + self.params.point_size > self.bounding_box.max_x {
                point.location.x = self.bounding_box.max_x - self.params.point_size;
                point.velocity.x = 0.0;
            }

            if point.location.y + self.params.point_size > self.bounding_box.max_y {
                point.location.y = self.bounding_box.max_y - self.params.point_size;
                point.velocity.y = 0.0;
            }

            if point.location.x - self.params.point_size < self.bounding_box.min_x {
                point.location.x = self.bounding_box.min_x + self.params.point_size;
                point.velocity.x = 0.0;
            }

            if point.location.y - self.params.point_size < self.bounding_box.min_y {
                point.location.y = self.bounding_box.min_y + self.params.point_size;
                point.velocity.y = 0.0;
            }
        }
    }

    fn apply_point_changes(&self, physics_system: &mut PhysicsSystem, point_changes: &[(u64, u64, Vec2, Vec2, Vec2)]) {
        for (id1, id2, delta_location, delta_velocity1, delta_velocity2) in point_changes {
            let point1 = physics_system.get_point_mut(*id1).expect("Invalid id");
            point1.location += *delta_location;
            point1.velocity += *delta_velocity1;
            //point1.velocity *= *compression;
            //point1.velocity += *delta_distance*(1.0-compression);

            let point2 = physics_system.get_point_mut(*id2).expect("Invalid id");
            point2.location -= *delta_location;
            point2.velocity += *delta_velocity2;
            //point2.velocity *= *compression;
            //point2.velocity -= *delta_distance*(1.0-compression);

            //assert!(*compression > 0.1, "Compression out of normal ranges!");
        }
    }

    fn calculate_push_out_velocity(vector1: Vec2, vector2: Vec2) -> Vec2 {
        let x = if vector1.x * vector2.x < 0.0 {
            vector2.x
        } else {
            0.0
        };
        let y = if vector1.y * vector2.y < 0.0 {
            vector2.y
        } else {
            0.0
        };

        vec2(x, y)
    }

    fn apply_constraints(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        let point_changes: Vec<_> = physics_system
            .get_constraints()
            .par_iter()
            .map(|c| {
                let point1 = physics_system
                    .get_point(c.get_point1())
                    .expect("Invalid constraint: point is none");
                let point2 = physics_system
                    .get_point(c.get_point2())
                    .expect("Invalid constraint: point is none");

                //let distance_between_points = point1.location.distance(point2.location);
                let delta_distance =
                    (c.get_distance() - point1.location.distance(point2.location)) / 2.0;
                let directon = (point1.location - point2.location).normalize();
                
                let delta_location = directon * delta_distance;

                let delta_velocity1 = Self::calculate_push_out_velocity(point1.velocity, delta_location)*delta;
                let delta_velocity2 = Self::calculate_push_out_velocity(point2.velocity, -delta_location)*delta;

                let required_change = (
                    c.get_point1(),
                    c.get_point2(),
                    delta_location,
                    delta_velocity1,
                    delta_velocity2
                );
                required_change
            })
            .collect();

        self.apply_point_changes(physics_system, &point_changes);
    }

    pub fn next_step(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        self.apply_gravity(physics_system, delta);
        self.apply_constraints(physics_system, delta);
        self.apply_velocity(physics_system, delta);
        self.fit_in_screen(physics_system);
    }
}
