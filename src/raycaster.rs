use crate::canvas::Canvas;
use crate::math::*;
use crate::texture::{Color, Texture};
use crate::{utils, Renderer, CONFIG};
use sdl2::event::Event;
use sdl2::image;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
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

    // Contains the keys which are currently pressed. Most
    // libraries (libgdx and sdl2) do not have proper key
    // pressed support, so we have to simulate it: when a
    // key is pressed, it's added to this set, when it's
    // released, it's removed. In the meantime we can look
    // at this set and simulate that the key is pressed
    // all the time.
    pressed_keys: HashSet<Keycode>,
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

impl Ray {
    fn new(x: f32, y: f32, angle: f32) -> Ray {
        // Number of steps from player's position in the
        // angle direction to check for collisions. Smaller
        // values are faster but also more imprecise.
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

impl Raycaster {
    pub fn new() -> Raycaster {
        Raycaster {
            map: utils::read_map(),
            player: Player {
                pos: Point { x: 2.0, y: 2.0 },
                angle: 00.0,
            },
            textures: Raycaster::load_textures(),
            pressed_keys: HashSet::new(),
        }
    }

    // In our implementation we allow both in-memory textures
    // as well as external images. External images are converted
    // to our internal format as well; since we index by color,
    // images should not have too many colors.
    //
    // In our map file textures are referred by their position
    // in the returned vector plus one since 0 means empty
    // space in the map.
    fn load_textures() -> Vec<Texture> {
        vec![
            // Background texture.
            Texture::load("images/background.png").unwrap(),
            // Basic brick texture.
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
                colors: vec![Color(0, 0, 0), Color(255, 255, 255)],
            },
            // Flat segment.
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
                colors: vec![Color(40, 40, 40), Color(255, 0, 0)],
            },
            // Image-based brick texture.
            Texture::load("images/texture.png").unwrap(),
            // 🐕 (Suki)
            Texture::load("images/dog.png").unwrap(),
        ]
    }

    fn handle_pressed_keys(&mut self) {
        // Could be moved to CONFIG, but currently I don't see
        // the necessity and am note sure if CONFIG is the best
        // place for these ("single used here" - values).
        let player_speed = 0.1;
        let player_rotation = 3.0;
        let player_radius = 5.0;

        // Handle all currently pressed keys.
        for keycode in self.pressed_keys.clone().into_iter() {
            match keycode {
                Keycode::A => {
                    self.player.angle -= player_rotation;
                }
                Keycode::D => {
                    self.player.angle += player_rotation;
                }
                Keycode::W => {
                    // Can this be simplified? Probably by adding some
                    // directional parameters +1 or -1 to a helper function.
                    // Would this make the following lines more readable?
                    // Probably not...
                    let dx = deg_to_rad(self.player.angle).cos() * player_speed;
                    let dy = deg_to_rad(self.player.angle).sin() * player_speed;
                    let np = self.player.pos.add(Point { x: dx, y: dy });
                    let cx = (np.x + dx * player_radius).floor() as usize;
                    let cy = (np.y + dy * player_radius).floor() as usize;
                    self.update_player_position(np, cx, cy)
                }
                Keycode::S => {
                    let dx = deg_to_rad(self.player.angle).cos() * player_speed;
                    let dy = deg_to_rad(self.player.angle).sin() * player_speed;
                    let np = self.player.pos.sub(Point { x: dx, y: dy });
                    let cx = (np.x - dx * player_radius).floor() as usize;
                    let cy = (np.y - dy * player_radius).floor() as usize;
                    self.update_player_position(np, cx, cy)
                }

                _ => {}
            }
        }
    }

    // If the player can move to the new position, move them.
    // Otherwise ignore the movement. We do this indepedently
    // for both x and y values to allow the player to glide
    // at walls.
    fn update_player_position(&mut self, np: Point, cx: usize, cy: usize) {
        let (x, y) = np.floor();
        if self.map[cy][x] == 0 {
            self.player.pos.y = np.y;
        }
        if self.map[y][cx] == 0 {
            self.player.pos.x = np.x;
        }
    }

    fn draw_background_strip(
        &self,
        canvas: &mut Canvas,
        background_texture: usize,
        x: i32,
        y1: i32,
        y2: i32,
    ) {
        let texture = &self.textures[background_texture];
        let start = x + self.player.angle as i32;
        let tx = (start % texture.width).abs() as usize;

        for y in y1..y2 {
            let ty = (y % texture.height) as usize;
            let idx = texture.map[ty][tx] as usize;
            let c = texture.colors[idx];
            canvas.set_pixel(x, y, c.0, c.1, c.2);
        }
    }

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
                tx.colors[tx_val].0,
                tx.colors[tx_val].1,
                tx.colors[tx_val].2,
            );
            y += y_incr;
        }
    }
}

impl Renderer for Raycaster {
    fn update(&mut self, events: Vec<Event>) -> bool {
        // Quit on escape and update hashset of pressed keys.
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

        self.handle_pressed_keys();
        true
    }

    fn draw(&mut self, canvas: &mut Canvas) -> Result<(), String> {
        let half_height = canvas.height as f32 / 2.0;
        let incr_angle = CONFIG.fov / canvas.width as f32;
        let mut ray_angle = self.player.angle - CONFIG.fov / 2.0;

        for x in 0..CONFIG.width {
            let mut ray = Ray::new(self.player.pos.x, self.player.pos.y, ray_angle);

            let mut ray_collision = 0;
            while ray_collision == 0 {
                ray.advance();
                let (x, y) = ray.pos.floor();
                ray_collision = self.map[y][x];
            }

            let hypothenuse = self.player.pos.dist(&ray.pos);
            let dist = hypothenuse * deg_to_rad(ray_angle - self.player.angle).cos();
            let wall_height = (half_height / dist);

            // Use the collision values from the map as the index
            // into our list of textures.
            let tx = &self.textures[(ray_collision - 1) as usize];
            // Since we only have orthogonal angles we needs to scan textures
            // either in x or y direction and use the addition of x and y of
            // the ray. Given it's orthogonal, one of these values will always
            // be constant.
            let tx_posx = ((tx.width as f32 * (ray.pos.x + ray.pos.y)).floor() as i32) % tx.width;

            // Draw pixel a) above the collision wall, b) the collions wall and
            // c) the floor, using the wall_height as the base reference.
            self.draw_background_strip(canvas, 0, x, 0, (half_height - wall_height) as i32);
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

        Ok(())
    }
}
