use crate::math::*;
use crate::system_loop::Renderer;
use crate::CONFIG;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as RectPoint;
use sdl2::render::WindowCanvas;

pub struct Raycaster {
    pub map: Vec<Vec<i32>>,
    pub player: Player,
}

impl Ray {
    fn new(x: f32, y: f32, angle: f32) -> Ray {
        let precision = 64.0;
        Ray {
            pos: Point { x, y },
            dx: deg_to_rad(angle).cos() / precision,
            dy: deg_to_rad(angle).sin() / precision,
        }
    }

    fn advance(&mut self) {
        self.pos.x += self.dx;
        self.pos.y += self.dy;
    }

    fn floor(&self) -> (usize, usize) {
        (self.pos.x.floor() as usize, self.pos.y.floor() as usize)
    }
}

pub struct Player {
    pos: Point,
    angle: f32,
}

struct Ray {
    pos: Point,
    dx: f32,
    dy: f32,
}

struct Point {
    x: f32,
    y: f32,
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
                pos: Point { x: 2.0, y: 2.0 },
                angle: 90.0,
            },
        }
    }
}

impl Renderer for Raycaster {
    fn update(&self, events: Vec<Event>) -> bool {
        for event in events.iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }

        return true;
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let incr_angle = CONFIG.fov / CONFIG.width as f32;
        let mut ray_angle = self.player.angle - CONFIG.fov / 2.0;

        for x in 0..CONFIG.width {
            // TODO(mlesniak) Move into data structure
            let mut ray = Ray::new(self.player.pos.x, self.player.pos.y, ray_angle);

            let mut wall = 0;
            while wall == 0 {
                ray.advance();
                let (x, y) = ray.floor();
                wall = self.map[y][x];
            }

            // TODO(mlesniak) can be computed in ray with original player
            let dist = ((self.player.pos.x - ray.pos.x).powi(2) + (self.player.pos.y - ray.pos.y).powi(2)).sqrt();
            let half_height = CONFIG.height as f32 / 2.0;
            let wall_height = half_height / dist;

            canvas.set_draw_color(Color::RGB(30, 30, 30));
            canvas.draw_line(
                RectPoint::new(x, 0),
                RectPoint::new(x, (half_height - wall_height) as i32),
            )?;

            canvas.set_draw_color(Color::RGB(100, 100, 100));
            canvas.draw_line(
                RectPoint::new(x, (half_height - wall_height) as i32),
                RectPoint::new(x, (half_height + wall_height) as i32),
            )?;

            canvas.set_draw_color(Color::GRAY);
            canvas.draw_line(
                RectPoint::new(x, (half_height + wall_height) as i32),
                RectPoint::new(x, CONFIG.height),
            )?;

            ray_angle += incr_angle;
        }

        self.player.angle += 2.5;

        Ok(())
    }
}
