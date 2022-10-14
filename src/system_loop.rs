use crate::config::CONFIG;
use crate::raycaster::Raycaster;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::{rand, sleep};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::*;
use std::time::{Duration, Instant};

pub fn run(mut event_pump: &mut EventPump, mut canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut raycaster: Raycaster = Raycaster::new(Color::GREEN);

    loop {
        let now = Instant::now();
        if event_handling(&mut event_pump) {
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
    let delta = (1_000 / CONFIG.fps - diff_ms as i32);
    if delta > 0 {
        std::thread::sleep(Duration::new(0, delta as u32 * 1_000 * 1_000));
    } else {
        // TODO(mlesniak) Wait until program actually started.
        println!("Unable to achieve FPS: missed by {}ms", delta.abs());
    }
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
