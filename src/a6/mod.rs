use nannou::prelude::*;
use rand;
mod cell;
mod utils;
use cell::Cell;

pub fn start_a6() {
    nannou::app(model).update(update).run();
}

struct Model {
    duration: f32,
    max: usize,
    grid: Vec<Vec<Cell>>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .title("a6")
        .view(view)
        .build()
        .unwrap();

    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(16);
    for row in 0..16 {
        grid.push(Vec::with_capacity(16));
        for _column in 0..16 {
            let mut quarter = Cell::new();
            quarter.draw(rand::random::<u32>());
            grid[row].push(quarter);
        }
    }
    Model {
        duration: 0.0,
        max: 0,
        grid,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.duration = model.duration + app.duration.since_prev_update.as_secs_f32();
    model.max = (model.duration * 30.0) as usize;
    if model.max > 11 {
        model.max = 11;
    }
    if model.duration > 3.0 {
        for row in 0..16 {
            for column in 0..16 {
                model.grid[row][column].draw(rand::random::<u32>());
            }
        }
        model.duration = 0.0;
        model.max = 0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for row in 0..16 {
        for column in 0..16 {
            if model.grid[row][column].points.len() > 10 {
                let slice = &model.grid[row][column].points[0..model.max];
                let color = model.grid[row][column].color;
                draw.polygon()
                    .points(slice.to_vec())
                    .x_y(
                        (column as i32 - 8) as f32 * 32.0,
                        (row as i32 - 8) as f32 * 32.0,
                    )
                    .color(color);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}