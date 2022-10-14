use std::time::{Duration, Instant};

use sdl2::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::config::CONFIG;
use crate::raycaster::Raycaster;

pub trait Renderer {
    fn update(&self, events: Vec<Event>) -> bool;
    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String>;
}

// TODO(mlesniak) Add Trait implementation to parameters
pub fn run(mut event_pump: &mut EventPump, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut raycaster: Raycaster = Raycaster::new(Color::GREEN);

    loop {
        let now = Instant::now();
        let events = collect_events(&mut event_pump);
        if !raycaster.update(events) {
           break;
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        raycaster.draw(canvas)?;
        canvas.present();

        wait(now)
    }

    Ok(())
}

/// Adaptive waiting based on frame rate.
fn wait(now: Instant) {
    let diff_ms = Instant::now().duration_since(now).as_millis();
    let delta = 1_000 / CONFIG.fps - diff_ms as i32;
    if delta > 0 {
        std::thread::sleep(Duration::new(0, delta as u32 * 1_000 * 1_000));
    } else {
        // TODO(mlesniak) Wait until program actually started.
        println!("Unable to achieve FPS: missed by {}ms", delta.abs());
    }
}

fn collect_events(event_pump: &mut EventPump) -> Vec<Event> {
    let mut events = vec![];

    for event in event_pump.poll_iter() {
        events.push(event);
        //
        // match event {
        //     Event::Quit { .. }
        //     | Event::KeyDown {
        //         keycode: Some(Keycode::Escape),
        //         ..
        //     } => return events,
        //     _ => {}
        // }
    }

    events
}
