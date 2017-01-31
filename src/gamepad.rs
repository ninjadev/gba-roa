use interconnect::InterconnectRead;
use sdl2::keyboard::Keycode;

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
    pub fn update(&mut self, pressed: bool, keycode: Keycode) {
        match keycode {
            Keycode::W => self.up = pressed,
            Keycode::A => self.left = pressed,
            Keycode::S => self.down = pressed,
            Keycode::D => self.right = pressed,
            Keycode::J => self.a = pressed,
            Keycode::K => self.b = pressed,
            Keycode::U => self.left_bumper = pressed,
            Keycode::I => self.right_bumper = pressed,
            Keycode::Return => self.start = pressed,
            Keycode::P => self.select = pressed,
            _ => (),
        }
    }
}

impl InterconnectRead for Gamepad {
    fn read(&self, address: u32) -> u32 {
        0
    }
}
