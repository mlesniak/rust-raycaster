mod config;
mod math;
mod raycaster;
mod system_loop;
mod utils;

use crate::config::CONFIG;
use crate::raycaster::Raycaster;
use sdl2::render::{WindowCanvas};
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

    let mut texture_creator = canvas.texture_creator();
    let mut renderer = Raycaster::new(&texture_creator);
    system_loop::run(&mut renderer, &mut event_pump, &mut canvas)
}
