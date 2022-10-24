use std::time::{Duration, Instant};

use crate::canvas::Canvas;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;
use sdl2::*;

use crate::config::CONFIG;
use crate::Renderer;

pub fn run(
    renderer: &mut dyn Renderer,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let mut tick = 0;

    let mut pixel_surface = canvas
        .texture_creator()
        .create_texture_streaming(
            // Fun fact: PixelFormatEnum::RGB888 is actually four bytes
            // large and just hides the fourth byte. Since we're not
            // interested in alpha-blending, three bytes for RGB are
            // sufficient.
            PixelFormatEnum::RGB24,
            CONFIG.width as u32,
            CONFIG.height as u32,
        )
        .expect("Unable to create inmemory pixel structure");
    let mut pixel_canvas = Canvas::new(CONFIG.width as u32, CONFIG.height as u32);

    // Note that pixels from previous frames are not cleared. While it might be
    // inconvenient from time to time it allows also to optimize rendering since
    // we can decide which parts of the next frame we want to touch and which
    // we leave as it is.
    loop {
        let now = Instant::now();
        let events = event_pump.poll_iter().collect();
        if !renderer.update(events) {
            break;
        }

        renderer
            .draw(&mut pixel_canvas)
            .expect("Unable to render into pixel surface");
        pixel_surface
            .update(
                None,
                &pixel_canvas.pixels,
                (pixel_canvas.width * 3) as usize,
            )
            .expect("Unable to update surface with new values from pixel array");
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

// Adaptive waiting based on frame rate.
fn wait(now: Instant) {
    let diff_ms = Instant::now().duration_since(now).as_millis();
    let delta = 1_000 / CONFIG.fps - diff_ms as i32;
    if delta >= 0 {
        std::thread::sleep(Duration::new(0, delta as u32 * 1_000 * 1_000));
    } else {
        log_missed_fps(delta);
    }
}

#[cfg(feature = "debug")]
fn log_missed_fps(missed_ms: i32) {
    println!("Unable to achieve FPS: missed by {}ms", missed_ms.abs());
}

#[cfg(not(feature = "debug"))]
fn log_missed_fps(_missed_ms: i32) {
    // Intentionally empty.
}