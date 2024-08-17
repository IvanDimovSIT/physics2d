use std::collections::HashMap;

use crate::{
    constraint::Constraint,
    point::Point,
};

const STARTING_CAPACITY: usize = 8;

pub struct PhysicsSystem {
    id_counter: u64,
    points: HashMap<u64, Point>,
    constraints: Vec<Constraint>,
}
impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            points: HashMap::with_capacity(STARTING_CAPACITY),
            constraints: Vec::with_capacity(STARTING_CAPACITY),
        }
    }

    fn new_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;

        id
    }

    pub fn add_point(&mut self, point: Point) -> u64 {
        let id = self.new_id();
        self.points.insert(id, point);

        id
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        let constraint_already_exists = self.constraints.iter().any(|c| {
            (c.get_point1() == constraint.get_point1() && c.get_point2() == constraint.get_point2())
                || (c.get_point1() == constraint.get_point2()
                    && c.get_point2() == constraint.get_point1())
        });

        if constraint_already_exists {
            return;
        }

        self.constraints.push(constraint);
    }

    pub fn get_points_ids(&self) -> &HashMap<u64, Point> {
        &self.points
    }

    pub fn get_point(&self, id: u64) -> Option<&Point> {
        self.points.get(&id)
    }

    pub fn get_point_mut(&mut self, id: u64) -> Option<&mut Point> {
        self.points.get_mut(&id)
    }

    pub fn get_points_mut(&mut self) -> Vec<&mut Point> {
        self.points.iter_mut().map(|(_, point)| point).collect()
    }

    pub fn get_constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    pub fn remove_point(&mut self, point_id: u64) {
        self.constraints.retain(|constraint| {
            constraint.get_point1() != point_id && constraint.get_point2() != point_id
        });

        self.points.remove(&point_id);
    }
}
