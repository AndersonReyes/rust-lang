use sdl3::{
    pixels::PixelFormat,
    render::{Canvas, Texture},
    sys::{
        pixels::SDL_PixelFormat,
        render::{SDL_CreateRenderer, SDL_Renderer},
    },
    video::Window,
};

pub struct Display<'a> {
    width: u32,
    height: u32,
    canvas: Canvas<Window>,
    texture: Texture<'a>,
}

impl<'a> Display<'a> {
    pub fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window: Window = video_subsystem
            .window("rust-sdl3 demo", width, height)
            .position_centered()
            .build()
            .expect("failed to create sdl window");

        let canvas: Canvas<Window> = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        let texture = texture_creator.create_texture_streaming(
            unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) },
            width,
            height,
        );

        Self {
            canvas,
            width,
            height,
            texture,
        }
    }

    pub fn render(&mut self, start: usize, data: &[u8]) {
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            data.iter().enumerate().for_each(|(idx, color)| {
                let offset = start + idx;
                buffer[offset] = color;
            });
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        })?;
    }
}
