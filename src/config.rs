pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

#[cfg(debug_assertions)]
pub const DIMENSIONS: Dimension = Dimension {
    width: 640,
    height: 480,
};

#[cfg(not(debug_assertions))]
const DIMENSIONS: Dimension = Dimension {
    width: 1920,
    height: 1080,
};
