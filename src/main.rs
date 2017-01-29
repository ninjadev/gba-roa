#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate enum_primitive;

extern crate num;

#[macro_use]
extern crate glium;

mod cpu;
mod display;
mod game_boy_advance;
mod gamepad;
mod instruction;
mod interconnect;
mod opcode;
mod parser;
mod utils;

mod errors {
    error_chain!{}
}
use errors::*;
quick_main!(run);

use glium::glutin::Event;
use glium::{DisplayBuild, Surface};
use interconnect::InterconnectWrite;

fn run() -> Result<()> {
    let mut gba = game_boy_advance::GameBoyAdvance::new();

    // Program courtesy of https://www.reinterpretcast.com/writing-a-game-boy-advance-game
    // Write into the I/O registers, setting video display parameters.
    let ioram_base = 0x04000000;
    // Use video mode 3 (in BG2, a 16bpp bitmap in VRAM)
    gba.interconnect.write(ioram_base, 0x03);
    // Enable BG2 (BG0 = 1, BG1 = 2, BG2 = 4, ...)
    gba.interconnect.write(ioram_base, 0x03);

    let vram_base = 0x06000000;
    gba.interconnect.write(vram_base + 80 * 240 + 115, 0b11111_00000_00000_0);
    gba.interconnect.write(vram_base + 80 * 240 + 120, 0b00000_11111_00000_0);
    gba.interconnect.write(vram_base + 80 * 240 + 125, 0b00000_00000_11111_0);

    loop {
        for ev in gba.interconnect.display.display.poll_events() {
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
