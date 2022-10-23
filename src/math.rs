use std::f32;

// Basic 2D point structure and primitive operations on points.
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn dist(&self, p: &Point) -> f32 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }

    pub fn add(&self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    pub fn sub(&self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }

    pub fn floor(&self) -> (usize, usize) {
        (self.x.floor() as usize, self.y.floor() as usize)
    }
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * f32::consts::PI / 180.0
}

