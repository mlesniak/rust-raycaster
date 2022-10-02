use rand::random;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::*;

// const WIDTH: u32 = 2560;
// const HEIGHT: u31 = 1440;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const LENGTH: usize = (WIDTH * HEIGHT) as usize * 4;
const FPS: u128 = 60;

fn main() -> Result<(), String> {
    let sdl_context = init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("pixel demo", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: &mut WindowCanvas =
        &mut window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();
    let mut tx = texture_creator
        .create_texture_streaming(None, WIDTH, HEIGHT)
        .unwrap();
    let mut pixels: Vec<u8> = Vec::new();
    for _ in 0..(WIDTH * HEIGHT * 4) {
        pixels.push(0);
    }

    let mut c: u8 = 0;
    let mut dir: i32 = 1;
    loop {
        let d1 = SystemTime::now();
        if event_handling(&mut event_pump) {
            break;
        }
        background(canvas);

        for i in (0..(WIDTH * HEIGHT * 4) as usize).step_by(4) {
            pixels[i] = c;
            pixels[i + 1] = c / 2;
            pixels[i + 2] = c / 3;
        }
        c = (c as i32 + dir) as u8;
        if c == 255 || c == 0 {
            dir *= -1;
        }
        tx.update(None, pixels.as_slice(), (WIDTH as usize * 4) as usize)
            .unwrap();

        canvas.copy(&tx, None, None)?;
        canvas.present();

        // let d2 = SystemTime::now().duration_since(d1).unwrap().as_millis();
        // if d2 < FPS {
        //     std::thread::sleep(Duration::new(0, (FPS - d2) as u32 * 1_000));
        // }
        let d3 = SystemTime::now().duration_since(d1).unwrap().as_millis();
        // if d3 > 20 {
        // if c % 10 == 0 {
        println!("{}", d3);
        // }
        // }
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

fn render_random_points(canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut map: HashMap<Color, Vec<Point>> = HashMap::new();

    for x in 0..100 {
        for y in 0..100 {
            let color = Color::RGB(
                rand::random::<u8>() % 10 + 150,
                rand::random::<u8>() % 10 + 50,
                rand::random::<u8>() % 10 + 150,
            );
            if !map.contains_key(&color) {
                map.insert(color, Vec::new());
            }

            let mut v = map.get_mut(&color).unwrap();
            v.push(Point::new(x, y));
        }
    }

    println!("{}", map.len());

    for (c, ps) in map.iter() {
        canvas.set_draw_color(*c);
        canvas.draw_points(ps.as_slice())?;
    }

    Ok(())
}

fn _render_sinus(canvas: &mut WindowCanvas) -> Result<(), String> {
    let scale = 50.0;
    let stretch = 40.0;

    canvas.set_draw_color(Color::RED);
    for x in 0..WIDTH {
        let xf = x as f64 / stretch;
        let xi = x as i32;
        let yi: i32 = (HEIGHT / 2) as i32 + (xf.sin() * scale) as i32;
        canvas.draw_point(Point::new(xi, yi))?;
    }

    Ok(())
}
