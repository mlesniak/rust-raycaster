use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::*;
use tinyrand::{Rand, StdRand};

struct Dimension {
    width: u32,
    height: u32,
}

#[cfg(debug_assertions)]
const DIMENSIONS: Dimension = Dimension {
    width: 640,
    height: 360,
};

#[cfg(not(debug_assertions))]
const DIMENSIONS: Dimension = Dimension(640, 360);

fn main() -> Result<(), String> {
    let (canvas, mut event_pump) = initialize_sdl();

    let mut canvas = canvas;
    let mut rand = StdRand::default();
    let mut prev = Instant::now();

    loop {
        if event_handling(&mut event_pump) {
            break;
        }
        let delta = Instant::now().duration_since(prev).as_millis();

        if delta > 1000 {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.set_draw_color(Color::WHITE);
            for x in (0..DIMENSIONS.width as i32).step_by(4) {
                let y1 = (rand.next_u32() % DIMENSIONS.height) as i32;
                let y2 = (rand.next_u32() % DIMENSIONS.height) as i32;
                canvas.draw_line(Point::new(x, 0), Point::new(x, y1))?;
                canvas.draw_line(Point::new(x, y2), Point::new(x, DIMENSIONS.height as i32))?;
            }
            canvas.present();
            prev = Instant::now();
        }
    }

    Ok(())
}

fn initialize_sdl() -> (WindowCanvas, EventPump) {
    let sdl_context = init().expect("General SDL error");
    let video_subsystem = sdl_context.video().expect("Video subsystem error");
    let window = video_subsystem
        .window("pixel demo", DIMENSIONS.width, DIMENSIONS.height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("Window subsystem error");
    let canvas: WindowCanvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Canvas subsystem error");
    let event_pump = sdl_context.event_pump().expect("Event Pump error");

    (canvas, event_pump)
}

/// Return true if we shall quit.
fn event_handling(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            _ => {}
        }
    }

    false
}
