use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::rand;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::*;

#[cfg(debug_assertions)]
const DIMENSIONS: (u32, u32) = (640, 360);

#[cfg(not(debug_assertions))]
const DIMENSIONS: (u32, u32) = (1920, 1080);

fn main() -> Result<(), String> {
    let sdl_context = init().expect("General SDL error");
    let video_subsystem = sdl_context.video().expect("Video subsystem error");
    let window = video_subsystem
        .window("pixel demo", DIMENSIONS.0, DIMENSIONS.1)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("Window subsystem error");
    let mut canvas: WindowCanvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Canvas subsystem error");
    let mut event_pump = sdl_context.event_pump().expect("Event Pump error");

    graphic_loop(&mut event_pump, &mut canvas)?;

    Ok(())
}

fn graphic_loop(
    mut event_pump: &mut EventPump,
    mut canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let mut tx = texture_creator
        .create_texture_streaming(None, DIMENSIONS.0, DIMENSIONS.1)
        .unwrap();
    let mut pixels: Vec<u8> = vec![0; (DIMENSIONS.0 * DIMENSIONS.1 * 4) as usize];

    let l = pixels.len();
    loop {
        if event_handling(&mut event_pump) {
            break;
        }

        tx.update(
            None,
            pixels.as_slice(),
            (DIMENSIONS.0 as usize * 4) as usize,
        )
        .expect("Copying pixels to GPU texture did not work");

        // Prevent mutable warning for the time being.
        pixels[(rand::random::<usize>()) % l] = 255;

        canvas.copy(&tx, None, None)?;
        canvas.present();
    }

    Ok(())
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
