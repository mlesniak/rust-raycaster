mod config;
mod math;
mod raycaster;
mod system_loop;

use crate::config::CONFIG;
use sdl2::render::WindowCanvas;
use sdl2::*;
use crate::raycaster::{Player, Raycaster};

fn main() -> Result<(), String> {
    let mut renderer = new();

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

    system_loop::run(&mut renderer, &mut event_pump, &mut canvas)
}

pub fn new() -> Raycaster {
    Raycaster {
        map: vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ],
        player: Player {
            x: 2.0,
            y: 2.0,
            angle: 90.0,
        },
    }
}
