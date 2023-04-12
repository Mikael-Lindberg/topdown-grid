use raylib::prelude::*;
use std::vec::Vec;

struct Grid {
    width: i32,
    height: i32,
}

struct Tile {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

fn main() {

    const grid: Grid = Grid { width: 24, height: 20 };
    const grid_size: i32 = 32;

    let mut coords: Vec<(i32, i32)> = Vec::new();

    for width in 0 .. grid.width {
        for height in 0 .. grid.height {
            coords.push((width, height));
        }
    }

    let (mut rl, thread) = raylib::init()
        .size(grid.width * grid_size, grid.height * grid_size)
        .title("Grid Manager")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        for (x, y) in &coords {
            let rect_x = x * grid_size;
            let rect_y = y * grid_size;

            let color = if (x + y) % 2 == 0 {
                Color::DARKGRAY
            } else {
                Color::LIGHTGRAY
            };

            d.draw_rectangle(
                rect_x,
                rect_y,
                grid_size,
                grid_size,
                color,
            );
        }

        drop(d);
    }
}

