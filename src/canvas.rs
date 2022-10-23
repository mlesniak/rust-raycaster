// Primitive canvas structure allowing to set single pixels as well
// as providing some helper functions such as drawing vertical lines.
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![0; (width * height * 3) as usize],
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, c1: u8, c2: u8, c3: u8) {
        self.pixels[((self.width as i32 * y + x) * 3) as usize] = c1;
        self.pixels[((self.width as i32 * y + x) * 3 + 1) as usize] = c2;
        self.pixels[((self.width as i32 * y + x) * 3 + 2) as usize] = c3;
    }

    pub fn draw_vertical_line(&mut self, x: i32, y1: i32, y2: i32, c1: u8, c2: u8, c3: u8) {
        let y1_clamped = y1.max(0);
        let y2_clamped = y2.min(self.height as i32);

        for y in y1_clamped..y2_clamped {
            self.set_pixel(x, y, c1, c2, c3)
        }
    }
}
