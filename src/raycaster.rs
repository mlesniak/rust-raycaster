use crate::CONFIG;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub struct Raycaster {
    pub color: Color,
}

impl Raycaster {
    pub fn new(color: Color) -> Raycaster {
        Raycaster { color }
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
