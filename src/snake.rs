use nannou::color::Rgb;
const BLOCK_SIZE: f32 = 30.0;

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Rgb,
}

pub struct Snake {
    pub body: Vec<Block>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![
                Block { x: 0.0, y: 0.0, size: BLOCK_SIZE, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: BLOCK_SIZE, y: 0.0, size: BLOCK_SIZE, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: BLOCK_SIZE * 2.0, y: 0.0, size: BLOCK_SIZE, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: BLOCK_SIZE * 3.0, y: 0.0, size: BLOCK_SIZE, color: Rgb::new(0.0, 1.0, 0.0) },
            ],
        }
    }
}