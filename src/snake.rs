use nannou::color::Rgb;
use crate::Block;

pub struct Snake {
    pub body: Vec<Block>,
}

impl Snake {
    pub fn new(
        block_size: f32,
    ) -> Snake {
        Snake {
            body: vec![
                Block { x: 0.0, y: 0.0, size: block_size, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: block_size, y: 0.0, size: block_size, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: block_size * 2.0, y: 0.0, size: block_size, color: Rgb::new(0.0, 1.0, 0.0) },
                Block { x: block_size * 3.0, y: 0.0, size: block_size, color: Rgb::new(0.0, 1.0, 0.0) },
            ],
        }
    }
}