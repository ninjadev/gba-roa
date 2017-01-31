#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate enum_primitive;

extern crate sdl2;

extern crate num;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

use interconnect::InterconnectWrite;

fn run() -> Result<()> {
    let mut gba = game_boy_advance::GameBoyAdvance::new();

    // Program courtesy of https://www.reinterpretcast.com/writing-a-game-boy-advance-game
    // Write into the I/O registers, setting video display parameters.
    let ioram_base = 0x04000000;
    // Use video mode 3 (in BG2, a 16bpp bitmap in VRAM)
    gba.interconnect.write(ioram_base, 0x03);
    // Enable BG2 (BG0 = 1, BG1 = 2, BG2 = 4, ...)
    gba.interconnect.write(ioram_base + 1, 0x03);

    let vram_base = 0x06000000;
    gba.interconnect.write(vram_base + 80 * 240 + 115, 0b0_00000_00000_11111);
    gba.interconnect.write(vram_base + 80 * 240 + 120, 0b0_00000_11111_00000);
    gba.interconnect.write(vram_base + 80 * 240 + 125, 0b0_11111_00000_00000);

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window(
        "Game Boy Advance: Rusty Oxidation Action - The ROA Emulator", 240, 160)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break,
                Event::KeyUp {keycode: Some(keycode), ..} => {
                    gba.interconnect.gamepad.update(true, keycode)
                }
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    gba.interconnect.gamepad.update(false, keycode)
                }
                _ => (),
            }
            gba.interconnect.display.draw(&mut renderer);
        }
    }
}
