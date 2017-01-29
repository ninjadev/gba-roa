use glium::glutin::{ElementState, VirtualKeyCode};
use interconnect::{InterconnectRead};

#[derive(Default, Debug)]
pub struct Gamepad {
    a: bool,
    b: bool,
    left: bool,
    down: bool,
    up: bool,
    right: bool,
    left_bumper: bool,
    right_bumper: bool,
    start: bool,
    select: bool,
}

impl Gamepad {
    pub fn update(&mut self, key_state: ElementState, virtual_key_code: VirtualKeyCode) {
        match virtual_key_code {
            VirtualKeyCode::W => self.up = key_state == ElementState::Pressed,
            VirtualKeyCode::A => self.left = key_state == ElementState::Pressed,
            VirtualKeyCode::S => self.down = key_state == ElementState::Pressed,
            VirtualKeyCode::D => self.right = key_state == ElementState::Pressed,
            VirtualKeyCode::J => self.a = key_state == ElementState::Pressed,
            VirtualKeyCode::K => self.b = key_state == ElementState::Pressed,
            VirtualKeyCode::U => self.left_bumper = key_state == ElementState::Pressed,
            VirtualKeyCode::I => self.right_bumper = key_state == ElementState::Pressed,
            VirtualKeyCode::Return => self.start = key_state == ElementState::Pressed,
            VirtualKeyCode::P => self.select = key_state == ElementState::Pressed,
            _ => (),
        }
    }
}

impl InterconnectRead for Gamepad {
    fn read(&self, address: u32) -> u32 {
        0
    }
}
