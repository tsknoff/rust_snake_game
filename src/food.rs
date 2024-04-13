use nannou::color::Rgb;
use nannou::rand;
use nannou::rand::Rng;
use crate::Block;

pub struct Food {
    pub block: Block,
}

impl Food {
    pub fn new(width: u32, height: u32, block_size: u32) -> Food {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width / block_size) as f32 * block_size as f32;
        let y = rng.gen_range(0..height / block_size) as f32 * block_size as f32;

        Food {
            block: Block {
                x,
                y,
                size: block_size as f32,
                color: Rgb::new(1.0, 0.0, 0.0),
            }
        }
    }
}