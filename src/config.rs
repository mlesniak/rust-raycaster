pub struct Config {
    pub width: i32,
    pub height: i32,
    pub fps: i32,
    pub fov: f32,
}

#[cfg(debug_assertions)]
pub const CONFIG: Config = Config {
    width: 640,
    height: 480,
    fps: 60,
    fov: 60.0,
};

#[cfg(not(debug_assertions))]
pub const CONFIG: Config = Config {
    // width: 1280,
    // height: 960,
    width: 640,
    height: 480,
    fps: 60,
    fov: 60.0,
};
