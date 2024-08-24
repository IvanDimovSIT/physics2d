use macroquad::{math::vec2, miniquad::window::screen_size};

use crate::{
    constraint::Constraint, input::Operation, physics_system::PhysicsSystem, point::Point,
    renderer::Renderer, simulator::Simulator, ui_renderer::UiRenderer,
};

struct SimulationSpeed {
    speeds: Vec<f32>,
    current: usize,
}
impl SimulationSpeed {
    fn new() -> Self {
        Self {
            speeds: vec![0.1, 0.25, 0.5, 0.75, 1.0, 1.25],
            current: 4,
        }
    }

    fn get_speed(&self) -> f32 {
        self.speeds[self.current]
    }

    fn increase_speed(&mut self) {
        if self.current + 1 < self.speeds.len() {
            self.current += 1;
        }
    }

    fn decrease_speed(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }
}

struct ControllerState {
    mouse_pos: (f32, f32),
    is_paused: bool,
    is_debug_mode: bool,
    selected_point: Option<u64>,
    is_draging: bool,
    simualtion_speed: SimulationSpeed,
}
impl Default for ControllerState {
    fn default() -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            is_paused: true,
            selected_point: None,
            is_debug_mode: false,
            is_draging: false,
            simualtion_speed: SimulationSpeed::new(),
        }
    }
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
            state: ControllerState::default(),
        }
    }

    fn handle_pause_unpause(&mut self) {
        self.state.is_paused = !self.state.is_paused;
    }

    fn handle_move(&mut self, x: &f32, y: &f32, delta: f32) {
        let old_mouse_pos = self.state.mouse_pos;
        self.state.mouse_pos.0 = *x;
        self.state.mouse_pos.1 = *y;

        if !self.state.is_draging {
            return;
        }
        if self.state.selected_point.is_none() {
            self.state.is_draging = false;
            return;
        }
        let id = self.state.selected_point.expect("Id should be valid");

        let option_point = self.physics_system.get_point_mut(id);
        if option_point.is_none() {
            self.state.is_draging = false;
            self.state.selected_point = None;
            return;
        }
        let point = option_point.unwrap();

        point.location = vec2(self.state.mouse_pos.0, self.state.mouse_pos.1);
        point.velocity = Simulator::calculate_velocity(
            vec2(old_mouse_pos.0, old_mouse_pos.1),
            point.location,
            delta,
        );
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
        self.state.is_draging = false;
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
        self.state.is_draging = false;
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
        self.state.is_draging = false;
        self.state.selected_point = None;
        let id = self.find_point_id_for_location(*x, *y);
        if id.is_none() {
            return;
        }

        self.physics_system.remove_point(id.unwrap());
    }

    fn handle_toggle_debug(&mut self) {
        self.state.is_debug_mode = !self.state.is_debug_mode;
    }

    fn handle_drag_start(&mut self, x: &f32, y: &f32) {
        self.state.selected_point = None;
        let id = self.find_point_id_for_location(*x, *y);
        if id.is_none() {
            return;
        }
        self.state.selected_point = id;
        self.state.is_draging = true;
    }

    fn handle_drag_end(&mut self) {
        self.state.is_draging = false;
        self.state.selected_point = None;
    }

    fn handle_increase_simulation_speed(&mut self) {
        self.state.simualtion_speed.increase_speed();
    }

    fn handle_decrease_simulation_speed(&mut self) {
        self.state.simualtion_speed.decrease_speed();
    }

    pub fn handle_input(&mut self, input: &[Operation], delta: f32) {
        for operation in input {
            match operation {
                Operation::PauseUnpause => self.handle_pause_unpause(),
                Operation::MousePosition { x, y } => {
                    self.handle_move(x, y, delta * self.state.simualtion_speed.get_speed())
                }
                Operation::MouseDown { x, y } => self.handle_mouse_down(x, y),
                Operation::MouseUp { x, y } => self.handle_mouse_up(x, y),
                Operation::Remove { x, y } => self.handle_right_click(x, y),
                Operation::ToggleDebug => self.handle_toggle_debug(),
                Operation::DragStart { x, y } => self.handle_drag_start(x, y),
                Operation::DragEnd => self.handle_drag_end(),
                Operation::IncreaseSimulationSpeed => self.handle_increase_simulation_speed(),
                Operation::DecreaseSimulationSpeed => self.handle_decrease_simulation_speed(),
            }
        }
    }

    pub fn next_step(&mut self, delta: f32) {
        if !self.state.is_paused {
            self.simulator.next_step(
                &mut self.physics_system,
                delta * self.state.simualtion_speed.get_speed(),
            );
        }
    }

    fn draw_ui_constraint_line(&self, screen_size: (f32, f32)) {
        if self.state.selected_point.is_none() || self.state.is_draging {
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

    fn draw_debug_window(&self, screen_size: (f32, f32)) {
        let option_id =
            self.find_point_id_for_location(self.state.mouse_pos.0, self.state.mouse_pos.1);
        if option_id.is_none() {
            return;
        }
        let id = option_id.unwrap();
        let option_point = self.physics_system.get_point(id);
        if option_point.is_none() {
            return;
        }
        let point = option_point.unwrap();

        let draw_at = (self.state.mouse_pos.0, self.state.mouse_pos.1 + 0.02);
        self.ui_renderer
            .draw_point_info(screen_size, id, point, draw_at);
    }

    pub fn draw_frame(&self) {
        self.renderer.draw(&self.physics_system);
        let screen_size = screen_size();

        self.draw_ui_constraint_line(screen_size);
        self.ui_renderer
            .draw_simulation_speed(screen_size, self.state.simualtion_speed.get_speed());

        if self.state.is_debug_mode {
            self.ui_renderer.draw_debug_text(
                screen_size,
                self.state.mouse_pos,
                self.physics_system.get_points_ids().len(),
                self.physics_system.get_constraints().len(),
            );
            self.draw_debug_window(screen_size);
        }

        if self.state.is_paused {
            self.ui_renderer.draw_paused_text(screen_size);
        }
    }
}
