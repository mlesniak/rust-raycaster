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

    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        for x in 0..CONFIG.width {
            let p1 = Point::new(x, rand::random::<i32>() % CONFIG.height);
            let p2 = Point::new(x, rand::random::<i32>() % CONFIG.height);
            canvas.draw_line(p1, p2)?;
        }

        Ok(())
    }
}
