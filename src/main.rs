mod config;
mod system_loop;

use crate::config::CONFIG;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::rand;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::*;

fn main() -> Result<(), String> {
    // Gather all relevant objects from the SDL context.
    let sdl_context = init().expect("General SDL error");
    let video_subsystem = sdl_context.video().expect("Video subsystem error");
    let window = video_subsystem
        .window("pixel demo", CONFIG.width as u32, CONFIG.height as u32)
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

    system_loop::run(&mut event_pump, &mut canvas)?;

    Ok(())
}
