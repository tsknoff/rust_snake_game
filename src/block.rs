use nannou::color::Rgb;

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Rgb,
}

impl Block {
    pub fn new(x: f32, y: f32, size: f32, color: Rgb) -> Block {
        Block { x, y, size, color }
    }
}