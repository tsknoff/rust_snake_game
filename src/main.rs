use nannou::prelude::*;

const BLOCK_SIZE: u32 = 20;
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
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .size(CLIENT_WIDTH, CLIENT_HEIGHT)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    Model { _window }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    if app.elapsed_frames() % FRAMES_PER_MILLISECOND != 0 {
        return;
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    if app.elapsed_frames() % FRAMES_PER_MILLISECOND != 0 {
        return;
    }
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}