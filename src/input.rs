use macroquad::input::{
    get_char_pressed, is_key_released, is_mouse_button_pressed, is_mouse_button_released,
    mouse_position,
};

#[derive(Debug)]
pub enum Operation {
    PauseUnpause,
    MousePosition { x: f32, y: f32 },
    MouseDown { x: f32, y: f32 },
    MouseUp { x: f32, y: f32 },
    Remove { x: f32, y: f32 },
    DragStart { x: f32, y: f32 },
    DragEnd,
    ToggleDebug,
}

pub fn get_input(screen_size: (f32, f32)) -> Vec<Operation> {
    let mut operations = vec![];

    let char_pressed = get_char_pressed();
    if char_pressed.is_some() && char_pressed.unwrap() == ' ' {
        operations.push(Operation::PauseUnpause);
    }

    let mouse_pos = mouse_position();
    let normalised_position = (mouse_pos.0 / screen_size.0, mouse_pos.1 / screen_size.1);
    operations.push(Operation::MousePosition {
        x: normalised_position.0,
        y: normalised_position.1,
    });

    if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
        operations.push(Operation::MouseDown {
            x: normalised_position.0,
            y: normalised_position.1,
        });
    }

    if is_mouse_button_released(macroquad::input::MouseButton::Left) {
        operations.push(Operation::MouseUp {
            x: normalised_position.0,
            y: normalised_position.1,
        });
    }

    if is_mouse_button_pressed(macroquad::input::MouseButton::Right) {
        operations.push(Operation::DragStart {
            x: normalised_position.0,
            y: normalised_position.1,
        });
    }

    if is_mouse_button_released(macroquad::input::MouseButton::Right) {
        operations.push(Operation::DragEnd);
    }

    if is_mouse_button_released(macroquad::input::MouseButton::Middle) {
        operations.push(Operation::Remove {
            x: normalised_position.0,
            y: normalised_position.1,
        });
    }

    if is_key_released(macroquad::input::KeyCode::D) {
        operations.push(Operation::ToggleDebug);
    }

    operations
}
