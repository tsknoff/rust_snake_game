use nannou::prelude::*;
use rust_snake_game::{Direction, SnakeGame};

const BLOCK_SIZE: u32 = 30;
const CLIENT_WIDTH: u32 = BLOCK_SIZE * 20;
const CLIENT_HEIGHT: u32 = BLOCK_SIZE * 20;
const FRAMES_PER_MILLISECOND: u64 = 10;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    snake_game: SnakeGame,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .size(CLIENT_WIDTH, CLIENT_HEIGHT)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window ,
        snake_game: SnakeGame::new(CLIENT_WIDTH, CLIENT_HEIGHT, BLOCK_SIZE).unwrap(),
    }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    if app.elapsed_frames() % FRAMES_PER_MILLISECOND != 0 {
        return;
    }

    _model.snake_game.move_snake();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _event {
        Event::WindowEvent { simple: Some(event), .. } => {
            match event {
                KeyPressed(key) => {
                    match key {
                        Key::Up => _model.snake_game.set_direction(Direction::Up),
                        Key::Down => _model.snake_game.set_direction(Direction::Down),
                        Key::Left => _model.snake_game.set_direction(Direction::Left),
                        Key::Right => _model.snake_game.set_direction(Direction::Right),
                        Key::Q => std::process::exit(0),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    if app.elapsed_frames() % FRAMES_PER_MILLISECOND != 0 {
        return;
    }
    let draw = app.draw();

    for block in &_model.snake_game.snake.body {
        draw.rect()
            .stroke(BLACK)
            .stroke_weight(2.0)
            .x_y(
                block.x + block.size / 2.0 - CLIENT_WIDTH as f32 / 2.0,
                block.y + block.size / 2.0 - CLIENT_HEIGHT as f32 / 2.0,
            )
            .w_h(block.size , block.size )
            .color(block.color);
    }

    draw.rect()
        .stroke(BLACK)
        .stroke_weight(2.0)
        .x_y(
            _model.snake_game.food.x + _model.snake_game.food.size / 2.0 - CLIENT_WIDTH as f32 / 2.0,
            _model.snake_game.food.y + _model.snake_game.food.size / 2.0 - CLIENT_HEIGHT as f32 / 2.0,
        )
        .w_h(_model.snake_game.food.size, _model.snake_game.food.size)
        .color(_model.snake_game.food.color);

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}