use interconnect::InterconnectWrite;
use utils::B5G5R5_to_B8G8R8;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Renderer;

pub struct Display {
    pub buf: [u16; 240 * 160],
}

impl Display {
    pub fn new() -> Self {
        Display {
            buf: [0b0000_0000_0000_0000u16; 240 * 160],
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear();
        for x in 0..240 {
            for y in 0..160 {
                renderer.pixel(
                    x as i16,
                    y as i16,
                    B5G5R5_to_B8G8R8(self.buf[y * 160 + x] as u16));
            }
        }
        renderer.present();
    }
}

impl InterconnectWrite for Display {
    fn write(&mut self, address: u32, word: u32) {
        self.buf[address as usize] = word as u16;
    }
}
