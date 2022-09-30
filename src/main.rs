use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const FPS: u32 = 30;

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

    loop {
        if event_handling(&mut event_pump) {
            break;
        }
        background(&mut canvas);
        render(&mut canvas)?;

        canvas.present();

        // Non-adaptive FPS loop.
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}

fn background(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
}

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

    return false;
}

fn render(canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.set_draw_color(Color::RED);
    for x in 0..WIDTH {
        canvas.draw_point(Point::new(x, HEIGHT / 2))?;
    }

    Ok(())
}
