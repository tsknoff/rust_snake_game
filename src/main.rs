use nannou::prelude::*;
use rust_snake_game::{Direction, SnakeGame, Block};


//---------------Constants---------------//
const BLOCK_SIZE: u32 = 25; // Size of each block in pixels.
const GRID_SIZE: u32 = 10; // Number of blocks in the grid.
const CLIENT_WIDTH: u32 = BLOCK_SIZE * GRID_SIZE; // Width of the window.
const CLIENT_HEIGHT: u32 = BLOCK_SIZE * GRID_SIZE; // Height of the window.
const FRAMES_COUNT: u64 = 300; // Number of frames to update the snake game.
//--------------------------------------//


fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .run();
}

// Model of the application.
struct Model {
    snake_game: SnakeGame,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(CLIENT_WIDTH, CLIENT_HEIGHT)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    Model {
        // Initialize the snake game and return the model.
        snake_game: SnakeGame::build(CLIENT_WIDTH, CLIENT_HEIGHT, BLOCK_SIZE).unwrap(),
    }
}

// Update the snake game every FRAMES_COUNT frames.
fn update(app: &App, _model: &mut Model, _update: Update) {
    if app.elapsed_frames() % FRAMES_COUNT != 0 { return; }

    // Update the snake game.
    _model.snake_game.update();
}

// Handle the KeyPressed event to change the direction of the snake.
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
    if app.elapsed_frames() % FRAMES_COUNT != 0 { return; }

    // Begin drawing.
    let draw = app.draw();

    // Draw the snake blocks.
    for block in &_model.snake_game.snake.body {
        draw_block(&draw, block);
    }

    // Draw the food block.
    draw_block(&draw, &_model.snake_game.food.block);

    // Clear the background to white.
    draw.background().color(WHITE);

    // Draw to the frame.
    draw.to_frame(app, &frame).unwrap();
}

// Helper function to draw a block.
fn draw_block(draw: &Draw, block: &Block) {
    draw.rect()
        .stroke(BLACK)
        .stroke_weight(1.0)
        .x_y(
            block.x + block.size / 2.0 - CLIENT_WIDTH as f32 / 2.0,
            block.y + block.size / 2.0 - CLIENT_HEIGHT as f32 / 2.0,
        )
        .w_h(block.size, block.size)
        .color(block.color);
}