use crate::sand::Sand;

//Returns true if it can move down,
//false otherwise
pub fn fall_down(
	x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
    height: usize,
) -> bool {
	if y == height - 1 {
        return false
    }

	if sand_grid[(y + 1) * width + x] == Sand::Air {
        future_sand[(y + 1) * width + x] = sand_grid[y * width + x];
        return true
    }

	false
}

pub fn fall_left_right(
	x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
    height: usize,
) -> bool {
	if y == height - 1 {
		return false	
	}

	if rand::random() {
        if x > 0 
			&& sand_grid[(y + 1) * width + (x - 1)] == Sand::Air
			&& future_sand[(y + 1) * width + (x - 1)] == Sand::Air {
            future_sand[(y + 1) * width + (x - 1)] = sand_grid[y * width + x];
            return true
        } else if x < width - 1 
			&& sand_grid[(y + 1) * width + (x + 1)] == Sand::Air
			&& future_sand[(y + 1) * width + (x + 1)] == Sand::Air {
            future_sand[(y + 1) * width + (x + 1)] = sand_grid[y * width + x];
            return true
        }
    } else {
        if x < width - 1 
			&& sand_grid[(y + 1) * width + (x + 1)] == Sand::Air
			&& future_sand[(y + 1) * width + (x + 1)] == Sand::Air {
            future_sand[(y + 1) * width + (x + 1)] = sand_grid[y * width + x];
            return true
        } else if x > 0 
			&& sand_grid[(y + 1) * width + (x - 1)] == Sand::Air
			&& future_sand[(y + 1) * width + (x - 1)] == Sand::Air {
            future_sand[(y + 1) * width + (x - 1)] = sand_grid[y * width + x];
            return true
        }
    }

	false
}

pub fn flow_left_right(
	x: usize,
    y: usize,
    sand_grid: &[Sand],
    future_sand: &mut [Sand],
    width: usize,
) -> bool {
	if rand::random() {
        if x > 0
            && sand_grid[y * width + (x - 1)] == Sand::Air
            && future_sand[y * width + (x - 1)] == Sand::Air
        {
            future_sand[y * width + (x - 1)] = sand_grid[y * width + x];
            return true
        } else if x < width - 1
            && sand_grid[y * width + (x + 1)] == Sand::Air
            && future_sand[y * width + (x + 1)] == Sand::Air
        {
            future_sand[y * width + (x + 1)] = sand_grid[y * width + x];
            return true
        }
    } else {
        if x < width - 1
            && sand_grid[y * width + (x + 1)] == Sand::Air
            && future_sand[y * width + (x + 1)] == Sand::Air
        {
            future_sand[y * width + (x + 1)] = sand_grid[y * width + x];
            return true
        } else if x > 0
            && sand_grid[y * width + (x - 1)] == Sand::Air
            && future_sand[y * width + (x - 1)] == Sand::Air
        {
            future_sand[y * width + (x - 1)] = sand_grid[y * width + x];
            return true
        }
    }

	false
}
