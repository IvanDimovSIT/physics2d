use macroquad::math::{vec2, Vec2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{physics_system::PhysicsSystem, point::Point};

pub struct SimulationParams {
    pub gravity: f32,
    pub air_resistence: f32,
    pub point_size: f32,
    pub spring_coeff: f32,
    pub damping: f32,
    pub collision_force: f32,
    pub push_from_sides_force: f32,
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

    pub fn calculate_velocity(from: Vec2, to: Vec2, delta: f32) -> Vec2 {
        (to - from) / delta
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
                point.velocity *= self.params.air_resistence.powf(delta);
                point.location += point.velocity * delta;
            }
        }
    }

    fn fit_in_screen(&self, physics_system: &mut PhysicsSystem) {
        for point in physics_system.get_points_mut() {
            if point.location.x + self.params.point_size > self.bounding_box.max_x {
                point.location.x = self.bounding_box.max_x - self.params.point_size;
                point.velocity.x = -self.params.push_from_sides_force;
            }

            if point.location.y + self.params.point_size > self.bounding_box.max_y {
                point.location.y = self.bounding_box.max_y - self.params.point_size;
                point.velocity.y = 0.0;
            }

            if point.location.x - self.params.point_size < self.bounding_box.min_x {
                point.location.x = self.bounding_box.min_x + self.params.point_size;
                point.velocity.x = self.params.push_from_sides_force;
            }

            if point.location.y - self.params.point_size < self.bounding_box.min_y {
                point.location.y = self.bounding_box.min_y + self.params.point_size;
                point.velocity.y = 0.0;
            }
        }
    }

    fn apply_point_changes(physics_system: &mut PhysicsSystem, point_changes: &[(u64, u64, Vec2)]) {
        for (id1, id2, delta_v) in point_changes {
            let point1 = physics_system.get_point_mut(*id1).expect("Invalid id");
            point1.velocity += *delta_v;

            let point2 = physics_system.get_point_mut(*id2).expect("Invalid id");
            point2.velocity -= *delta_v;
        }
    }

    fn apply_constraints(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        let point_changes: Vec<_> = physics_system
            .get_constraints()
            .par_iter()
            .map(|c| {
                let point1 = physics_system
                    .get_point(c.get_point1())
                    .expect("Invalid constraint: point should be some");
                let point2 = physics_system
                    .get_point(c.get_point2())
                    .expect("Invalid constraint: point should be some");

                let direction = point2.location - point1.location;
                let distance_between_points = direction.length();
                let direction_norm = direction.normalize_or_zero();
                let displacement = distance_between_points - c.get_distance();
                let force = self.params.spring_coeff * displacement;
                let relative_velocity = point2.velocity - point1.velocity;
                let damping = self.params.damping * relative_velocity.dot(direction_norm);
                let delta_v = (force + damping) * direction_norm * delta;

                let required_change = (c.get_point1(), c.get_point2(), delta_v);
                required_change
            })
            .collect();

        Self::apply_point_changes(physics_system, &point_changes);
    }

    fn calculate_collision(&self, point1: &Point, point2: &Point, delta: f32) -> Vec2 {
        let direction = point1.location - point2.location;
        let distance = direction.length();
        if distance >= self.params.point_size * 2.0 {
            return vec2(0.0, 0.0);
        }

        let force = (self.params.point_size * 2.0 - distance) * self.params.collision_force * delta;

        direction.normalize_or_zero() * force
    }

    fn apply_collision_velocity_changes(
        physics_system: &mut PhysicsSystem,
        point_changes: &[(u64, Vec2)],
    ) {
        for (id, delta_v) in point_changes {
            let point = physics_system
                .get_point_mut(*id)
                .expect("Point should be found");
            point.velocity += *delta_v;
        }
    }

    fn apply_collisions(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        let point_changes: Vec<_> = physics_system
            .get_points_ids()
            .par_iter()
            .map(|(id, point)| {
                let mut change = vec2(0.0, 0.0);
                for (other_id, other_point) in physics_system.get_points_ids() {
                    if id == other_id {
                        continue;
                    }
                    change += self.calculate_collision(point, other_point, delta);
                }

                (*id, change)
            })
            .filter(|(_id, v)| v.x != 0.0 || v.y != 0.0)
            .collect();

        Self::apply_collision_velocity_changes(physics_system, &point_changes);
    }

    pub fn next_step(&self, physics_system: &mut PhysicsSystem, delta: f32) {
        self.apply_gravity(physics_system, delta);
        self.apply_constraints(physics_system, delta);
        self.apply_collisions(physics_system, delta);
        self.apply_velocity(physics_system, delta);
        self.fit_in_screen(physics_system);
    }
}
