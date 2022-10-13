use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::rand;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::*;

pub fn run(mut event_pump: &mut EventPump, mut canvas: &mut WindowCanvas) -> Result<(), String> {
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
