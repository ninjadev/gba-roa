#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate enum_primitive;

extern crate num;

#[macro_use]
extern crate glium;

mod game_boy_advance;
mod cpu;
mod instruction;
mod opcode;
mod parser;
mod gamepad;
mod interconnect;

mod errors {
    error_chain!{}
}
use errors::*;
quick_main!(run);

use glium::{DisplayBuild, Surface};
use glium::glutin::Event;

fn run() -> Result<()> {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut gba = game_boy_advance::GameBoyAdvance::new();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::KeyboardInput(key_state, _, Some(virtual_key_code))
                    => gba.interconnect.gamepad.update(key_state, virtual_key_code),
                Event::Closed => return Ok(()),
                _ => (),
            }

            println!("Keyboard state: {:?}", gba.interconnect.gamepad);
        }
    }
}
