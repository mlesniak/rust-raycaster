use crate::math::*;
use crate::CONFIG;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub struct Raycaster {
    pub color: Color,
    map: Vec<Vec<i32>>,
    player: Player,
}

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

struct Ray {
    x: f32,
    y: f32,
    angle: f32,
}

impl Raycaster {
    pub fn new(color: Color) -> Raycaster {
        Raycaster {
            color,
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

    // TODO(mlesniak) draw is a bad name, maybe split between update and draw
    //                later on...?
    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        let incr_angle = CONFIG.fov / CONFIG.width as f32;
        let mut ray_angle = self.player.angle - CONFIG.fov / 2.0;
        let precision = 64.0;

        for x in 0..CONFIG.width {
            // let p1 = Point::new(x, rand::random::<i32>() % CONFIG.height);
            // let p2 = Point::new(x, rand::random::<i32>() % CONFIG.height);
            // canvas.draw_line(p1, p2)?;

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

        Ok(())
    }
}
