use sdl3::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormat,
    render::{Canvas, FRect, Texture},
    sys::pixels::SDL_PixelFormat,
    video::Window,
};

use crate::image::color::{Color, as_u8};

pub struct Display {
    width: u32,
    height: u32,
    canvas: Canvas<Window>,
    texture: Texture,
    event_pump: EventPump,
    events: Vec<Event>,
    pub is_running: bool,
}

impl Display {
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
        let texture = texture_creator
            .create_texture(
                unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) },
                sdl3::render::TextureAccess::Streaming,
                width,
                height,
            )
            .map_err(|e| e.to_string())
            .expect("failed to create texture");

        let event_pump = sdl_context.event_pump().expect("failed to get event pump");

        Self {
            canvas,
            width,
            height,
            texture,
            event_pump,
            events: vec![],
            is_running: true,
        }
    }

    pub fn update(&mut self, data: &Vec<Vec<Color>>) {
        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..data.len() {
                    for x in 0..data[0].len() {
                        let color = data[y][x];
                        let offset = pitch * y + x * 3;
                        buffer[offset] = as_u8(color.x);
                        buffer[offset + 1] = as_u8(color.y);
                        buffer[offset + 2] = as_u8(color.z);
                    }
                }
            })
            .expect("error filling in texture with data");

        // Clear the screen
        self.canvas
            .set_draw_color(sdl3::pixels::Color::RGB(66, 66, 66));
        self.canvas.clear();
        self.canvas
            .copy(
                &self.texture,
                None,
                Some(FRect::new(0.0, 0.0, self.width as f32, self.height as f32)),
            )
            .expect("failed to copy texture to canvas");

        self.canvas.present();
    }

    /// check if a quit event was sent
    pub fn handle_quit(&mut self) {
        for e in self.event_pump.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            };
        }
    }

    /// block until a quit event is sent
    pub fn wait_until_quit(&mut self) {
        while self.is_running {
            self.handle_quit();
        }
    }
}
