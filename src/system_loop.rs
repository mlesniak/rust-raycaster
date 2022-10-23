use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::*;

use crate::config::CONFIG;

pub trait Renderer {
    fn update(&mut self, events: Vec<Event>) -> bool;
    fn draw(&mut self, canvas: &mut Canvas) -> Result<(), String>;
}

pub struct Canvas {
    // TODO(mlesniak) right type?
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![0; width * height * 3],
        }
    }

    pub fn draw_vertical_line(&mut self, x: i32, y1: i32, y2: i32, c1: u8, c2: u8, c3: u8) {
        for y in y1..y2 {
            self.set_pixel(x, y, c1, c2, c3)
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, c1: u8, c2: u8, c3: u8) {
        // self.pixels[(y * self.width as i32 * 3 + x) as usize] = c1;
        // self.pixels[(y * self.width as i32 * 3 + x + 1) as usize] = c2;
        // self.pixels[(y * self.width as i32 * 3 + x + 2) as usize] = c3;
        // self.pixels[461760] = 255;
        self.pixels[((self.width as i32 * y + x) * 3) as usize] = c1;
        self.pixels[((self.width as i32 * y + x) * 3 + 1) as usize] = c2;
        self.pixels[((self.width as i32 * y + x) * 3 + 2) as usize] = c3;
    }
}

pub fn run(
    renderer: &mut dyn Renderer,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let mut tick = 0;

    let mut texture_creator = canvas.texture_creator();
    let mut pixel_surface = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            CONFIG.width as u32,
            CONFIG.height as u32,
        )
        .unwrap();

    let mut c = Canvas::new(CONFIG.width as usize, CONFIG.height as usize);
    loop {
        let now = Instant::now();
        let events = event_pump.poll_iter().collect();
        if !renderer.update(events) {
            break;
        }

        // canvas.set_draw_color(Color::BLACK);
        // canvas.clear();
        // renderer.draw(canvas)?;
        renderer.draw(&mut c);
        pixel_surface.update(None, &c.pixels, (c.width * 3) as usize);
        canvas.copy(&pixel_surface, None, None)?;
        canvas.present();

        // Wait until window is actually displayed
        // before activating FPS toggling.
        if tick > CONFIG.fps {
            wait(now)
        }
        tick += 1;
    }

    Ok(())
}

/// Adaptive waiting based on frame rate.
fn wait(now: Instant) {
    let diff_ms = Instant::now().duration_since(now).as_millis();
    let delta = 1_000 / CONFIG.fps - diff_ms as i32;
    if delta >= 0 {
        std::thread::sleep(Duration::new(0, delta as u32 * 1_000 * 1_000));
    } else {
        println!("Unable to achieve FPS: missed by {}ms", delta.abs());
    }
}
