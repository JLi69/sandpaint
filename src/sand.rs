use sdl2::pixels::Color;

mod move_sand;
mod update_sand;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sand {
    Air,
    Sand,
    Water,
    Wall,
}

pub fn sand_color(sand: Sand) -> Color {
    match sand {
        Sand::Air => Color::WHITE,
        Sand::Sand => Color::YELLOW,
        Sand::Water => Color::BLUE,
        Sand::Wall => Color::GRAY,
    }
}

fn update_pixel(
    x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
    height: usize,
) {
    match sand_grid[y * width + x] {
        Sand::Air => {}
		Sand::Sand => {
            update_sand::update_particle(x, y, sand_grid, future_sand, width, height);
        }
        Sand::Water => {
            update_sand::update_liquid(x, y, sand_grid, future_sand, width, height);
        }
        _ => {
            future_sand[y * width + x] = sand_grid[y * width + x];
        }
    }
}

pub fn update_sand(
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    frame: u32,
    width: usize,
    height: usize,
) {
    //Clear the future buffer
    for y in 0..height {
        for x in 0..width {
            future_sand[y * width + x] = Sand::Air;
        }
    }

    //Update the sand grid
    for y in 0..height {
        for xval in 0..width {
            let mut x = xval;
            if frame % 2 == 0 {
                x = width - 1 - xval;
            }

            update_pixel(x, y, sand_grid, future_sand, width, height);
        }
    }
}
