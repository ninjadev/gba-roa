use interconnect::InterconnectWrite;
use glium::{self, DisplayBuild, Frame, Surface};
use glium::texture::RawImage2d;
use glium::texture::ClientFormat;
use utils::RGB;

pub struct Display {
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub buf: [u16; 240 * 160],
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Display {
    pub fn new() -> Self {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(240 * 2, 160 * 2)
            .with_title("Game Boy Advance: Rusty Oxidation Action - The ROA Emulator")
            .build_glium()
            .unwrap();

        let width = 160;
        let height = 240;

        let vertex_shader_src = r#"
            in vec2 position;
            out vec2 vuv;

            void main() {
                vuv = position;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            in vec2 vuv;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, vuv / 2.0 + 0.5);
            }
        "#;

        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        Display {
            display: display,
            program: program,
            buf: [0b0000_0000_0000_0000u16; 240 * 160],
        }
    }

    fn vsync(&self) {
        let shape = vec![Vertex { position: [-1.0, -1.0] },
                         Vertex { position: [1.0, -1.0] },
                         Vertex { position: [-1.0, 1.0] },
                         Vertex { position: [-1.0, 1.0] },
                         Vertex { position: [1.0, -1.0] },
                         Vertex { position: [1.0, 1.0] }];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        use std::borrow::Cow;
        let image = glium::texture::RawImage2d {
            data: Cow::from(&self.buf[..]),
            width: 240,
            height: 160,
            format: ClientFormat::U5U5U5U1,
        };

        let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();
        let sampler = texture.sampled()
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
        let uniforms = uniform! {
            tex: sampler,
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer,
                  &indices,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

    }
}

impl InterconnectWrite for Display {
    fn write(&mut self, address: u32, word: u32) {
        self.buf[address as usize] = word as u16;

        self.vsync();
    }
}
