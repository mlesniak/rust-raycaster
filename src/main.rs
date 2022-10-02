use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::*;

use tinyrand::{Rand, StdRand};

#[cfg(debug_assertions)]
const DIMENSIONS: (u32, u32) = (640, 360);

#[cfg(not(debug_assertions))]
const DIMENSIONS: (u32, u32) = (1920, 1080);

fn main() -> Result<(), String> {
    let sdl_context = init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("pixel demo", DIMENSIONS.0, DIMENSIONS.1)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas: &mut WindowCanvas = &mut window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();
    let mut tx = texture_creator
        .create_texture_streaming(None, DIMENSIONS.0, DIMENSIONS.1)
        .unwrap();
    let mut pixels: Vec<u8> = Vec::new();
    pixels.resize((DIMENSIONS.0 * DIMENSIONS.1 * 4) as usize, 0);

    let mut rng = StdRand::default();

    let mut c: u8 = 0;
    let mut dir: i32 = 1;
    loop {
        if event_handling(&mut event_pump) {
            break;
        }
        background(canvas);

        for i in (0..(DIMENSIONS.0 * DIMENSIONS.1 * 4) as usize).step_by(4) {
            pixels[i] = (rng.next_u64() % 256) as u8;
            pixels[i + 1] = (rng.next_u64() % 256) as u8;
            pixels[i + 2] = (rng.next_u64() % 256) as u8;
        }
        c = (c as i32 + dir) as u8;
        if c == 255 || c == 0 {
            dir *= -1;
        }
        tx.update(
            None,
            pixels.as_slice(),
            (DIMENSIONS.0 as usize * 4) as usize,
        )
        .unwrap();

        canvas.copy(&tx, None, None)?;
        canvas.present();
    }

    Ok(())
}

fn background(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
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
