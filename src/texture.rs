use sdl2::image;
use sdl2::surface::Surface;
use std::collections::HashMap;

// A basic RGB color struct so we're independent of SDL2
// for our textures.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub map: Vec<Vec<i32>>,
    pub colors: Vec<Color>,
}

impl Texture {
    fn new(map: Vec<Vec<i32>>, colors: Vec<Color>) -> Texture {
        Texture {
            width: map[0].len() as i32,
            height: map.len() as i32,
            map,
            colors,
        }
    }

    pub fn load(filename: &str) -> Result<Texture, String> {
        let mut map: Vec<Vec<i32>> = vec![];
        let mut color_to_maptile: HashMap<Color, i32> = HashMap::new();

        let mut color_index = 0;
        let mut row: Vec<i32> = vec![];

        let surface: Surface = image::LoadSurface::from_file(filename)?;
        surface.with_lock(|pixels| {
            for i in (0..pixels.len()).step_by(3) {
                // Create new row if necessary.
                if i != 0 && i as u32 % (surface.width() * 3) == 0 {
                    map.push(row.clone());
                    row = vec![];
                }
                // Add pixel as indexed value into the color map to the
                // tile map. Add the color of the pixel to color indices
                // if it's not there yet.
                let color = Color(pixels[i], pixels[i + 1], pixels[i + 2]);
                match color_to_maptile.get(&color) {
                    None => {
                        row.push(color_index);
                        color_to_maptile.insert(color, color_index);
                        color_index += 1;
                    }
                    Some(idx) => {
                        row.push(*idx);
                    }
                }
            }
        });
        map.push(row);

        // Reverse map and use its interger values as arrays for a new
        // color vector for faster access.
        let mut indexed_colors: Vec<Color> = vec![Color(0, 0, 0); color_to_maptile.len()];
        for color in color_to_maptile.keys() {
            let idx = color_to_maptile[color] as usize;
            indexed_colors[idx] = *color;
        }

        Ok(Texture::new(map, indexed_colors))
    }
}
