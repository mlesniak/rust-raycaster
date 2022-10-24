use sdl2::image;
use sdl2::surface::Surface;
use std::collections::HashMap;

#[derive(Debug,Clone, Eq, PartialEq, Hash, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub map: Vec<Vec<i32>>,
    pub colors: Vec<Color>,
}

impl Texture {
    pub fn load(filename: &str) -> Result<Texture, String> {
        // TODO(mlesniak) Refactor this
        let surface: Surface = image::LoadSurface::from_file(filename)?;

        let mut counter = 0;
        let mut colors: HashMap<Color, i32> = HashMap::new();

        let mut map: Vec<Vec<i32>> = vec![];
        let mut row: Vec<i32> = vec![];

        surface.with_lock(|pixels| {
            for i in (0..pixels.len()).step_by(3) {
                if i != 0 && i as u32 % (surface.width() * 3) == 0 {
                    map.push(row.clone());
                    row = vec![];
                }
                let color = Color(pixels[i], pixels[i + 1], pixels[i + 2]);
                match colors.get(&color) {
                    None => {
                        row.push(counter);
                        colors.insert(color, counter);
                        counter += 1;
                    }
                    Some(idx) => {
                        row.push(*idx);
                    }
                }
            }
        });
        map.push(row);

        let mut indexed_colors: Vec<Color> = vec![Color(0,0,0); colors.len()];
        for color in colors.keys() {
            let idx = colors[color] as usize;
            indexed_colors[idx] = *color;
        }

        Ok(Texture {
            width: surface.width() as i32,
            height: surface.height() as i32,
            map,
            colors: indexed_colors,
        })
    }
}
