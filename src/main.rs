use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("pixel demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.present();

    // TODO(mlesniak) Have a map with flags?
    let mut change_color = false;
    let mut time_passed = 0;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::MouseButtonDown { .. } => change_color = true,
                Event::MouseButtonUp { .. } => change_color = false,
                _ => {}
            }
        }

        canvas.clear();

        if change_color && time_passed > 10 {
            time_passed = 0;
            canvas.set_draw_color(Color::RGB(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            ));
        }
        time_passed += 1;

        canvas.present();

        // Non-adaptive FPS duration.
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
