use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::*;

use crate::config::CONFIG;

pub trait Renderer {
    fn update(&mut self, events: Vec<Event>) -> bool;
    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String>;
}

pub fn run(
    renderer: &mut dyn Renderer,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let mut tick = 0;
    loop {
        let now = Instant::now();
        let events = event_pump.poll_iter().collect();
        if !renderer.update(events) {
            break;
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        renderer.draw(canvas)?;
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
