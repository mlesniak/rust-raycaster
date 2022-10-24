mod canvas;
mod config;
mod math;
mod raycaster;
mod system_loop;
mod utils;
mod texture;

use crate::canvas::Canvas;
use crate::config::CONFIG;
use crate::raycaster::Raycaster;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::*;

// Trait every render logic has to implement.
pub trait Renderer {
    fn update(&mut self, events: Vec<Event>) -> bool;
    fn draw(&mut self, canvas: &mut Canvas) -> Result<(), String>;
}

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

    // Actual logic is defined in an implementation of Renderer and is
    // independent of any graphic library. Instead we solely need an
    // implementation of the Canvas trait to draw vertial lines and
    // single pixels.
    let mut renderer = Raycaster::new();

    system_loop::run(&mut renderer, &mut event_pump, &mut canvas)
}
