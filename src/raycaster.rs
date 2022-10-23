use crate::math::*;
use crate::system_loop::{Canvas, Renderer};
use crate::{utils, CONFIG};
use sdl2::event::Event;
use sdl2::image;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point as RectPoint, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct Raycaster {
    pub map: Vec<Vec<i32>>,
    pub player: Player,

    // Cache loaded assets
    textures: Vec<Texture>,
    background: sdl2::render::Texture,

    // Internal state.
    pressed_keys: HashSet<Keycode>,
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

impl Point {
    fn dist(&self, p: &Point) -> f32 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }

    fn add(&self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    fn sub(&self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }

    fn floor(&self) -> (usize, usize) {
        (self.x.floor() as usize, self.y.floor() as usize)
    }
}

#[derive(Debug)]
pub struct Texture {
    width: i32,
    height: i32,
    map: Vec<Vec<i32>>,
    colors: Vec<Color>,
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
                let color = Color::RGB(pixels[i], pixels[i + 1], pixels[i + 2]);
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

        let mut indexed_colors: Vec<Color> = vec![Color::BLACK; colors.len()];
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

// Until we load it from a file
#[inline]
fn load_textures() -> Vec<Texture> {
    vec![
        Texture {
            width: 8,
            height: 8,
            map: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0, 1, 0, 0],
            ],
            colors: vec![Color::RGB(40, 40, 40), Color::RGB(60, 60, 60)],
        },
        Texture {
            width: 8,
            height: 8,
            map: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
            ],
            colors: vec![Color::RGB(40, 40, 40), Color::RGB(255, 0, 0)],
        },
        Texture::load("texture.png").unwrap(),
        Texture::load("suki.png").unwrap(),
    ]
}

impl Raycaster {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Raycaster {
        // let background = Texture::load("background.png").unwrap();
        // let surface: Surface = image::LoadSurface::from_file("background.png")?;
        let bg = texture_creator.load_texture("background.png").unwrap();

        // println!("{} / {}", background.width, background.height);

        Raycaster {
            map: utils::read_map(),
            player: Player {
                pos: Point { x: 2.0, y: 2.0 },
                angle: 00.0,
            },
            textures: load_textures(),
            background: bg,
            pressed_keys: HashSet::new(),
        }
    }
}

impl Renderer for Raycaster {
    fn update(&mut self, events: Vec<Event>) -> bool {
        let player_speed = 0.1;
        let player_rotation = 3.0;
        let player_radius = 1.0;

        for event in events.iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    self.pressed_keys.insert(*key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    self.pressed_keys.remove(key);
                }

                _ => {}
            }
        }

        for keycode in self.pressed_keys.clone().into_iter() {
            match keycode {
                Keycode::A => {
                    self.player.angle -= player_rotation;
                }
                Keycode::D => {
                    self.player.angle += player_rotation;
                }
                Keycode::W => {
                    let dx = deg_to_rad(self.player.angle).cos() * player_speed;
                    let dy = deg_to_rad(self.player.angle).sin() * player_speed;
                    let np = self.player.pos.add(Point { x: dx, y: dy });
                    let cx = (np.x + dx * player_radius).floor() as usize;
                    let cy = (np.y + dy * player_radius).floor() as usize;
                    let (x, y) = np.floor();
                    if self.map[cy][x] == 0 {
                        self.player.pos.y = np.y;
                    }
                    if self.map[y][cx] == 0 {
                        self.player.pos.x = np.x;
                    }
                }
                Keycode::S => {
                    let dx = deg_to_rad(self.player.angle).cos() * player_speed;
                    let dy = deg_to_rad(self.player.angle).sin() * player_speed;
                    let np = self.player.pos.sub(Point { x: dx, y: dy });
                    let cx = (np.x - dx * player_radius).floor() as usize;
                    let cy = (np.y - dy * player_radius).floor() as usize;
                    let (x, y) = np.floor();
                    if self.map[cy][x] == 0 {
                        self.player.pos.y = np.y;
                    }
                    if self.map[y][cx] == 0 {
                        self.player.pos.x = np.x;
                    }
                }

                _ => {}
            }
        }

        true
    }

    fn draw(&mut self, canvas: &mut Canvas) -> Result<(), String> {
        // fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        // canvas.copy(&self.background, None, None).unwrap();
        // Ok(())

        let half_height = canvas.height as f32 / 2.0;
        let incr_angle = CONFIG.fov / canvas.width as f32;
        let mut ray_angle = self.player.angle - CONFIG.fov / 2.0;
        let query = self.background.query();

        for x in 0..CONFIG.width {
            let mut ray = Ray::new(self.player.pos.x, self.player.pos.y, ray_angle);

            let mut ray_content = 0;
            while ray_content == 0 {
                ray.advance();
                let (x, y) = ray.pos.floor();
                ray_content = self.map[y][x];
            }

            let hypothenuse = self.player.pos.dist(&ray.pos);
            let dist = hypothenuse * deg_to_rad(ray_angle - self.player.angle).cos();
            let wall_height = (half_height / dist);
            // let wall_height = (half_height / dist).min(half_height);

            // In-memory texture mapping
            let tx = &self.textures[(ray_content - 1) as usize];
            let tx_posx = ((tx.width as f32 * (ray.pos.x + ray.pos.y)).floor() as i32) % tx.width;

            canvas.draw_vertical_line(x, 0, (half_height - wall_height) as i32, 0, 0, 255);

            // canvas.set_draw_color(Color::RGB(30, 30, 30));
            // canvas.draw_line(
            //     RectPoint::new(x, 0),
            //     RectPoint::new(x, (half_height - wall_height) as i32),
            // )?;
            // self.draw_background_strip(canvas, x, 0, (half_height - wall_height) as i32);
            // canvas
            //     .copy(
            //         &self.background,
            //         Rect::new(
            //             (x + self.player.angle as i32).abs() % query.width as i32,
            //             0,
            //             1,
            //             (half_height - wall_height) as u32,
            //         ),
            //         Rect::new(
            //             x.abs() % query.width as i32,
            //             0,
            //             1,
            //             (half_height - wall_height) as u32,
            //         ),
            //     )
            //     .unwrap();
            //

            self.draw_texture_strip(canvas, x, wall_height, tx_posx, tx);

            canvas.draw_vertical_line(
                x,
                (half_height + wall_height) as i32,
                CONFIG.height,
                128,
                128,
                128,
            );

            ray_angle += incr_angle;
        }

        // canvas.draw_vertical_line(300, 0, canvas.height as i32, 255, 0, 0);
        // canvas.set_pixel(320, 240, 255, 255, 255);

        Ok(())
    }
}

impl Raycaster {
    // fn draw_background_strip(&self, canvas: &mut WindowCanvas, x: i32, y1: i32, y2: i32) {
    //     let start = x + self.player.angle as i32;
    //     let tx = (start % self.background.width).abs() as usize;
    //
    //     for y in y1..y2 {
    //         let ty = (y % self.background.height) as usize;
    //         let idx = self.background.map[ty][tx] as usize;
    //         let c = self.background.colors[idx];
    //         canvas.set_draw_color(c);
    //         // TODO(mlesniak) draw_points
    //         canvas.draw_point(sdl2::rect::Point::new(x, y)).unwrap();
    //     }
    // }

    fn draw_texture_strip(
        &self,
        canvas: &mut Canvas,
        x: i32,
        wall_height: f32,
        tx_posx: i32,
        tx: &Texture,
    ) {
        let y_incr = wall_height * 2.0 / tx.height as f32;
        let mut y = CONFIG.height as f32 / 2.0 - wall_height;

        for i in 0..tx.height as usize {
            let tx_val = tx.map[i][tx_posx as usize] as usize;
            canvas.draw_vertical_line(
                x,
                y as i32,
                (y + y_incr) as i32,
                tx.colors[tx_val].r,
                tx.colors[tx_val].g,
                tx.colors[tx_val].b,
            );
            // canvas.set_draw_color(tx.colors[tx_val]);
            // canvas
            //     .draw_line(
            //         sdl2::rect::Point::new(x, y as i32),
            //         sdl2::rect::Point::new(x, (y + y_incr) as i32),
            //     )
            //     .unwrap();

            y += y_incr;
        }
    }
}
