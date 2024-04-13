mod snake;
mod food;

use std::cmp::PartialEq;
use nannou::color::{Rgb};
use snake::Snake;
use food::Food;

pub struct SnakeGame {
    pub block_size: u32,
    pub snake: Snake,
    pub direction: Direction,
    pub width: u32,
    pub height: u32,
    pub food: Food,
    pub score: u32,
}

impl SnakeGame {
    pub fn build(
        width: u32,
        height: u32,
        block_size: u32,
    ) -> Result<SnakeGame, &'static str> {

        if width % block_size != 0 || height % block_size != 0 {
            panic!("Width and height must be divisible by block size")
        }

        let new_food = Food::new(width, height, block_size);

        Ok(SnakeGame {
            block_size,
            snake: Snake::new(
                block_size as f32,
            ),
            direction: Direction::Right,
            width,
            height,
            food: new_food,
            score: 0,
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

    pub fn update(&mut self) {
        let current_head = &self.snake.body.last().unwrap();
        let mut new_head = Block {
            x: current_head.x,
            y: current_head.y,
            size: self.block_size as f32,
            color: Rgb::new(0.0, 1.0, 0.0)
        };

        match self.direction {
            Direction::Up => {
                // use block_size
                new_head.y += self.block_size as f32;
            }
            Direction::Down => {
                new_head.y -= self.block_size as f32;
            }
            Direction::Left => {
                new_head.x -= self.block_size as f32;
            }
            Direction::Right => {
                new_head.x += self.block_size as f32;
            }
        }

        if self.check_collision_with_border(&new_head) || self.check_collision_with_self(&new_head) {
            self.game_over();
            return;
        }

        if self.check_collision_with_food(&new_head) {
            self.score += 1;
            self.food = Food::new(self.width, self.height, self.block_size);
        } else {
            self.snake.body.remove(0);
        }

        self.snake.body.push(new_head);
    }

    fn check_collision_with_food(&self, new_head: &Block) -> bool {
        self.food.block.x == new_head.x && self.food.block.y == new_head.y
    }
    fn check_collision_with_border(&self, new_head: &Block) -> bool {
        new_head.x < 0.0 || new_head.x >= self.width as f32 || new_head.y < 0.0 || new_head.y >= self.height as f32
    }

    fn check_collision_with_self(&self, new_head: &Block) -> bool {
        for block in &self.snake.body {
            if block.x == new_head.x && block.y == new_head.y {
                return true;
            }
        }
        false
    }
    fn game_over(&mut self) {
        self.snake = Snake::new(self.block_size as f32);
        self.direction = Direction::Right;
        self.food = Food::new(self.width, self.height, self.block_size);
        self.score = 0;
    }
}

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

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Rgb,
}