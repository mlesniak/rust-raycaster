use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::*;
use std::time::Duration;

// SNES resolution, scaled up.
// Opinionated.
const WIDTH: i32 = 265;
const HEIGHT: i32 = 224;
const FPS: u32 = 30;
const SCALE: i32 = 3;

trait Graphics {
    fn pixel(&mut self, x: i32, y: i32, c: Color);
    fn clear(&mut self, c: Color);
    fn show(&mut self);
}

struct SDL2Renderer<'a> {
    canvas: &'a mut WindowCanvas,
}

impl SDL2Renderer<'_> {
    fn new(canvas: &mut WindowCanvas) -> SDL2Renderer {
        SDL2Renderer { canvas }
    }
}

impl Graphics for SDL2Renderer<'_> {
    fn pixel(&mut self, x: i32, y: i32, c: Color) {
        self.canvas.set_draw_color(c);
        self.canvas.draw_point(Point::new(x, y)).unwrap();
    }

    fn clear(&mut self, c: Color) {
        self.canvas.set_draw_color(c);
        self.canvas.clear();
    }

    fn show(&mut self) {
        self.canvas.present();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("pixel demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.present();

    let mut graphics = SDL2Renderer::new(&mut canvas);

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        graphics.clear(Color::BLACK);
        for i in 100..200 {
            graphics.pixel(i, 100, Color::RED);
        }
        graphics.show();

        // Non-adaptive FPS loop.
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
