use crate::sand::{move_sand, Sand, SandProperties, SandGrid};

pub fn update_particle(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) {
    if y == sand_grid.height - 1 {
        if properties.can_replace.contains(&future_sand[y * sand_grid.width + x]) {
			future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
        }
        return;
    }

    if move_sand::fall_down(x, y, sand_grid, future_sand, properties) {
        return;
    }

    if move_sand::fall_left_right(x, y, sand_grid, future_sand, properties) {
        return;
    }

    if properties.can_replace.contains(&future_sand[y * sand_grid.width + x]) {
        future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
    }
}

pub fn update_liquid(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) {
    if y == sand_grid.height - 1 {
        if move_sand::flow_left_right(x, y, sand_grid, future_sand, properties) {
            return;
        }

        if properties.can_replace.contains(&future_sand[y * sand_grid.width + x]) {
            future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
        }
        return;
    }

    if move_sand::fall_down(x, y, sand_grid, future_sand, properties) {
        return;
    }

    if move_sand::fall_left_right(x, y, sand_grid, future_sand, properties) {
        return;
    }

    if move_sand::flow_left_right(x, y, sand_grid, future_sand, properties) {
        return;
    }

    if properties.can_replace.contains(&future_sand[y * sand_grid.width + x]) {
        future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
    }
}

pub fn explode(
	x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
	radius: isize
) {
	let mut angle = 0.0f64;
	while angle < 3.14159 * 2.0 {
		let mut posx = 0.0f64;
		let mut posy = 0.0f64;
		while (posx * posx + posy * posy).sqrt() < radius as f64 {
			posx += angle.cos();
			posy += angle.sin();
			let trans_x = posx + x as f64;
			let trans_y = posy + y as f64;	

			if trans_x < 0.0 || trans_y < 0.0 ||
				trans_x >= sand_grid.width as f64 ||
				trans_y >= sand_grid.height as f64 {
				break;	
			}

			let trans_x = trans_x.floor() as usize;
			let trans_y = trans_y.floor() as usize;

			if properties.can_replace.contains(&sand_grid.grid[trans_y * sand_grid.width + trans_x]) {
				future_sand[trans_y * sand_grid.width + trans_x] = Sand::Fire;
			} else {
				break;
			}
		}
		angle += 0.02;
	}
}

pub fn update_fire(
	x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) {	
	let mut flammable_count = 0;

	for yoff in -2isize..2isize {
		for xoff in -2isize..2isize {
			if xoff + (x as isize) < 0 ||
			   yoff + (y as isize) < 0 ||
			   xoff + (x as isize) >= sand_grid.width as isize ||
			   yoff + (y as isize) >= sand_grid.height as isize {
				continue;	
			}

			if xoff * xoff + yoff * yoff > 2 * 2 {
				continue;	
			}

			let posx = (xoff + x as isize) as usize;
			let posy = (yoff + y as isize) as usize;

			if properties.can_replace.contains(&sand_grid.grid[posy * sand_grid.width + posx])
			   && sand_grid.grid[posy * sand_grid.width + posx] != Sand::Air {
				if rand::random::<f64>().fract() < 0.04 {
					future_sand[posy * sand_grid.width + posx] = Sand::Fire;
				}
				flammable_count += 1;
			} else if sand_grid.grid[posy * sand_grid.width + posx] == Sand::Air &&
			  rand::random::<f64>() < 0.1 {
				future_sand[posy * sand_grid.width + posx] = Sand::Fire;
			} 
		}
	}

	if properties.can_replace.contains(&future_sand[y * sand_grid.width + x]) &&
	   (flammable_count >= 1 || 
		(move_sand::count_neighbors(x, y, sand_grid, Sand::Fire) >= 2 && rand::random::<f64>() < 0.8)) {		
		future_sand[y * sand_grid.width + x] = Sand::Fire;
	}
}
