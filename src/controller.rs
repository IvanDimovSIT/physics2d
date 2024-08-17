use macroquad::{math::vec2, miniquad::window::screen_size};

use crate::{
    constraint::Constraint,
    input::Operation,
    physics_system::{self, PhysicsSystem},
    point::{self, Point},
    renderer::Renderer,
    simulator::Simulator,
    ui_renderer::UiRenderer,
};

struct ControllerState {
    mouse_pos: (f32, f32),
    is_paused: bool,
    selected_point: Option<u64>,
}

pub struct Controller {
    physics_system: PhysicsSystem,
    simulator: Simulator,
    renderer: Renderer,
    ui_renderer: UiRenderer,
    state: ControllerState,
}
impl Controller {
    pub fn new(
        physics_system: PhysicsSystem,
        simulator: Simulator,
        renderer: Renderer,
        ui_renderer: UiRenderer,
    ) -> Self {
        Self {
            physics_system,
            simulator,
            renderer,
            ui_renderer,
            state: ControllerState {
                mouse_pos: (0.0, 0.0),
                is_paused: true,
                selected_point: None,
            },
        }
    }

    fn handle_pause_unpause(&mut self) {
        self.state.is_paused = !self.state.is_paused;
    }

    fn handle_move(&mut self, x: &f32, y: &f32) {
        self.state.mouse_pos.0 = *x;
        self.state.mouse_pos.1 = *y;
    }

    fn find_point_id_for_location(&self, x: f32, y: f32) -> Option<u64> {
        let point_size = self.renderer.get_draw_params().point_size;
        self.physics_system
            .get_points_ids()
            .iter()
            .find(|(_id, point)| point.location.distance(vec2(x, y)) <= point_size)
            .map(|(id, _point)| *id)
    }

    fn handle_mouse_down(&mut self, x: &f32, y: &f32) {
        let point_id = self.find_point_id_for_location(*x, *y);
        if point_id.is_some() {
            self.state.selected_point = point_id;
            return;
        }

        self.physics_system
            .add_point(Point::new(vec2(*x, *y), vec2(0.0, 0.0), false));
    }

    fn toggle_static(&mut self, id: u64) {
        let option_point = self.physics_system.get_point_mut(id);
        if option_point.is_some() {
            let point = option_point.unwrap();
            point.is_static = !point.is_static;
        }
    }

    fn handle_mouse_up(&mut self, x: &f32, y: &f32) {
        let point_id = self.find_point_id_for_location(*x, *y);
        if point_id.is_none() || self.state.selected_point.is_none() {
            self.state.selected_point = None;
            return;
        }

        let id1 = point_id.unwrap();
        let id2 = self.state.selected_point.unwrap();
        self.state.selected_point = None;

        if id1 == id2 {
            self.toggle_static(id1);
            return;
        }

        let point1 = self.physics_system.get_point(id1);
        let point2 = self.physics_system.get_point(id2);

        if point1.is_none() || point2.is_none() {
            return;
        }

        let distance = point1.unwrap().location.distance(point2.unwrap().location);
        let constraint = Constraint::new(id1, id2, distance);

        self.physics_system.add_constraint(constraint);
    }

    fn handle_right_click(&mut self, x: &f32, y: &f32) {
        self.state.selected_point = None;
        let id = self.find_point_id_for_location(*x, *y);
        if id.is_none() {
            return;
        }

        self.physics_system.remove_point(id.unwrap());
    }

    pub fn handle_input(&mut self, input: &[Operation]) {
        for operation in input {
            match operation {
                Operation::PauseUnpause => self.handle_pause_unpause(),
                Operation::MousePosition { x, y } => self.handle_move(x, y),
                Operation::MouseDown { x, y } => self.handle_mouse_down(x, y),
                Operation::MouseUp { x, y } => self.handle_mouse_up(x, y),
                Operation::RightClick { x, y } => self.handle_right_click(x, y),
                //_ => println!("Unhandled input operation: {:?}", operation)
            }
        }
    }

    pub fn next_step(&mut self, delta: f32) {
        if !self.state.is_paused {
            self.simulator.next_step(&mut self.physics_system, delta);
        }
    }

    fn draw_ui_constraint_line(&self, screen_size: (f32, f32)) {
        if self.state.selected_point.is_none() {
            return;
        }

        let point = self
            .physics_system
            .get_point(self.state.selected_point.unwrap());
        if point.is_none() {
            return;
        }

        self.ui_renderer.draw_line(
            vec2(self.state.mouse_pos.0, self.state.mouse_pos.1),
            point.unwrap().location,
            screen_size,
        );
    }

    pub fn draw_frame(&self) {
        self.renderer.draw(&self.physics_system);
        let screen_size = screen_size();

        self.draw_ui_constraint_line(screen_size);
        if self.state.is_paused {
            self.ui_renderer.draw_paused_text(screen_size);
        }
    }
}
