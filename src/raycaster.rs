use crate::math::*;
use crate::CONFIG;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::system_loop::Renderer;

// TODO(mlesniak) Generator function
pub struct Raycaster {
    pub map: Vec<Vec<i32>>,
    pub player: Player,
}

impl Raycaster {
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
}

// TODO(mlesniak) Trait
impl Renderer for Raycaster {
    fn update(&self, events: Vec<Event>) -> bool {
        for event in events.iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => { return false }
                _ => {}
            }
        }

        return true
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let incr_angle = CONFIG.fov / CONFIG.width as f32;
        let mut ray_angle = self.player.angle - CONFIG.fov / 2.0;
        let precision = 64.0;

        for x in 0..CONFIG.width {
            // TODO(mlesniak) Move into data structure
            let mut ray_x = self.player.x;
            let mut ray_y = self.player.y;
            let ray_dx = deg_to_rad(ray_angle).cos() / precision;
            let ray_dy = deg_to_rad(ray_angle).sin() / precision;

            let mut wall = 0;
            while wall == 0 {
                ray_x += ray_dx;
                ray_y += ray_dy;
                wall = self.map[ray_y.floor() as usize][ray_x.floor() as usize];
            }

            let dist = ((self.player.x - ray_x).powi(2) + (self.player.y - ray_y).powi(2)).sqrt();
            let half_height = CONFIG.height as f32 / 2.0;
            let wall_height = half_height / dist;

            canvas.set_draw_color(Color::RED);
            canvas.draw_line(
                Point::new(x, 0),
                Point::new(x, (half_height - wall_height) as i32),
            )?;

            canvas.set_draw_color(Color::GREEN);
            canvas.draw_line(
                Point::new(x, (half_height - wall_height) as i32),
                Point::new(x, (half_height + wall_height) as i32),
            )?;

            canvas.set_draw_color(Color::BLUE);
            canvas.draw_line(
                Point::new(x, (half_height + wall_height) as i32),
                Point::new(x, CONFIG.height),
            )?;

            ray_angle += incr_angle;
        }

        self.player.angle += 2.5;

        Ok(())
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

// struct Ray {
//     x: f32,
//     y: f32,
//     angle: f32,
// }

impl Raycaster {

}
