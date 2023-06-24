use crate::sand::{Sand, move_sand};

pub fn update_particle(
	x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
    height: usize,
) {
	if y == height - 1 {
        future_sand[y * width + x] = sand_grid[y * width + x];
        return
    }

    if move_sand::fall_down(x, y, sand_grid, future_sand, width, height) {
		return	
	}

    if move_sand::fall_left_right(x, y, sand_grid, future_sand, width, height) {
		return	
	}

    future_sand[y * width + x] = sand_grid[y * width + x];
}

pub fn update_liquid(
	x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
    height: usize,
) {
	if y == height - 1 {
        if move_sand::flow_left_right(x, y, sand_grid, future_sand, width) {
			return	
		}
        future_sand[y * width + x] = sand_grid[y * width + x];
        return
    }

    if move_sand::fall_down(x, y, sand_grid, future_sand, width, height) {
		return	
	}

    if move_sand::fall_left_right(x, y, sand_grid, future_sand, width, height) {
		return	
	}

    if move_sand::flow_left_right(x, y, sand_grid, future_sand, width) {
		return	
	}

    future_sand[y * width + x] = sand_grid[y * width + x];
}
