use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::rand;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::*;

struct Dimension {
    width: u32,
    height: u32,
}

#[cfg(debug_assertions)]
const DIMENSIONS: Dimension = Dimension {
    width: 640,
    height: 480,
};

#[cfg(not(debug_assertions))]
const DIMENSIONS: (u32, u32) = (1920, 1080);

fn main() -> Result<(), String> {
    // Gather all relevant objects from the SDL context.
    let sdl_context = init().expect("General SDL error");
    let video_subsystem = sdl_context.video().expect("Video subsystem error");
    let window = video_subsystem
        .window("pixel demo", DIMENSIONS.width, DIMENSIONS.height)
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

    system_loop(&mut event_pump, &mut canvas)?;

    Ok(())
}

fn system_loop(
    mut event_pump: &mut EventPump,
    mut canvas: &mut WindowCanvas,
) -> Result<(), String> {
    loop {
        if event_handling(&mut event_pump) {
            break;
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::YELLOW);
        let p1 = Point::new(rand::random::<i32>() % 600, rand::random::<i32>() % 600);
        let p2 = Point::new(rand::random::<i32>() % 600, rand::random::<i32>() % 600);
        canvas.draw_line(p1, p2)?;

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
