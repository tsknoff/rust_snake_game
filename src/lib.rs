mod snake;

use std::cmp::PartialEq;
use nannou::color::{Rgb};
use nannou::rand;
use nannou::rand::Rng;
use snake::{Snake, Block};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            _ => false,
        }
    }
}

pub struct SnakeGame {
    pub block_size: u32,
    pub snake: Snake,
    pub direction: Direction,
    pub width: u32,
    pub height: u32,
    pub food: Block,
}

impl SnakeGame {
    pub fn new(
        width: u32,
        height: u32,
        block_size: u32,
    ) -> Result<SnakeGame, &'static str> {

        if width % block_size != 0 || height % block_size != 0 {
            return Err("Width and height must be divisible by block size");
        }

        let new_food = Block {
            x: 0.0,
            y: 0.0,
            size: 30.0,
            color: Rgb::new(1.0, 0.0, 0.0) };

        Ok(SnakeGame {
            block_size,
            snake: Snake::new(),
            direction: Direction::Right,
            width,
            height,
            food: new_food,
        })
    }

    pub fn set_direction(&mut self, new_direction: Direction) {

        if (self.direction == Direction::Up && new_direction == Direction::Down) ||
            (self.direction == Direction::Down && new_direction == Direction::Up) ||
            (self.direction == Direction::Left && new_direction == Direction::Right) ||
            (self.direction == Direction::Right && new_direction == Direction::Left) {
            return;
        }
        self.direction = new_direction;
    }

    pub fn move_snake(&mut self) {
        let current_head = &self.snake.body.last().unwrap();
        let mut new_head = Block { x: current_head.x, y: current_head.y, size: 30.0, color: Rgb::new(0.0, 1.0, 0.0) };

        match self.direction {
            Direction::Up => {
                // use block_size
                new_head.y += 30.0;
            }
            Direction::Down => {
                new_head.y -= 30.0;
            }
            Direction::Left => {
                new_head.x -= 30.0;
            }
            Direction::Right => {
                new_head.x += 30.0;
            }
        }

        if new_head.x < 0.0 || new_head.x >= self.width as f32 || new_head.y < 0.0 || new_head.y >= self.height as f32 {
            return;
            // TODO game over
        }

        if (self.food.x == new_head.x && self.food.y == new_head.y) {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.width / self.block_size) as f32 * 30.0;
            let y = rng.gen_range(0..self.height / self.block_size) as f32 * 30.0;

            self.food.x = x;
            self.food.y = y;
        } else {
            self.snake.body.remove(0);
        }

        self.snake.body.push(new_head);

        // // удаляем хвост змеи если не съели еду
        // if self.snake.body.last().unwrap().x != self.food.x || self.snake.body.last().unwrap().y != self.food.y {
        //     self.snake.body.remove(0);
        // } else {
        //     self.food.x = 10.0 * 30.0;
        //     self.food.y = 10.0 * 30.0;
        //
        //     println!("Food x: {}, y: {}", self.food.x, self.food.y)
        // }
    }
}