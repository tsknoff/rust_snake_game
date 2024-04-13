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
                Block::new(0.0, 0.0, block_size, Rgb::new(0.0, 1.0, 0.0)),
                Block::new(block_size, 0.0, block_size, Rgb::new(0.0, 1.0, 0.0)),
                Block::new(block_size * 2.0, 0.0, block_size, Rgb::new(0.0, 1.0, 0.0)),
            ],
        }
    }
}