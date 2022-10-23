use std::time::{Duration, Instant};

use crate::canvas::Canvas;
use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
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

    let mut texture_creator = canvas.texture_creator();
    let mut pixel_surface = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            CONFIG.width as u32,
            CONFIG.height as u32,
        )
        .unwrap();

    let mut c = Canvas::new(CONFIG.width as u32, CONFIG.height as u32);
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
