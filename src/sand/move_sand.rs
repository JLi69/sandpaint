use crate::sand::{Sand, SandProperties, SandGrid};

pub fn replace(sand: Sand,
		   sand_to_replace: Sand,
		   properties: &SandProperties) -> Sand {
	match properties.replace_with.get(&sand_to_replace) {
		Some(s) => *s,
		_ => sand
	}
}

//Returns true if it can move down,
//false otherwise
pub fn fall_down(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) -> bool {
    if y == sand_grid.height - 1 {
        return false;
    }

    if properties
        .can_replace
        .contains(&sand_grid.grid[(y + 1) * sand_grid.width + x])
    {
        future_sand[(y + 1) * sand_grid.width + x] = 
			replace(sand_grid.grid[y * sand_grid.width + x],
					sand_grid.grid[(y + 1) * sand_grid.width + x], properties);
        return true;
    }

    false
}

pub fn fall_left_right(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) -> bool {
    if y == sand_grid.height - 1 {
        return false;
    }

    if rand::random() {
        if x > 0
            && properties
                .can_replace
                .contains(&sand_grid.grid[(y + 1) * sand_grid.width + (x - 1)])
            && properties
                .can_replace
                .contains(&future_sand[(y + 1) * sand_grid.width + (x - 1)])
        {
            future_sand[(y + 1) * sand_grid.width + (x - 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[(y + 1) * sand_grid.width + (x - 1)],
						properties);
            return true;
        } else if x < sand_grid.width - 1
            && properties
                .can_replace
                .contains(&sand_grid.grid[(y + 1) * sand_grid.width + (x + 1)])
            && properties
                .can_replace
                .contains(&future_sand[(y + 1) * sand_grid.width + (x + 1)])
        {
            future_sand[(y + 1) * sand_grid.width + (x + 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[(y + 1) * sand_grid.width + (x + 1)],
						properties);
            return true;
        }
    } else {
        if x < sand_grid.width - 1
            && properties
                .can_replace
                .contains(&sand_grid.grid[(y + 1) * sand_grid.width + (x + 1)])
            && properties
                .can_replace
                .contains(&future_sand[(y + 1) * sand_grid.width + (x + 1)])
        {
            future_sand[(y + 1) * sand_grid.width + (x + 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[(y + 1) * sand_grid.width + (x + 1)],
						properties);
            return true;
        } else if x > 0
            && properties
                .can_replace
                .contains(&sand_grid.grid[(y + 1) * sand_grid.width + (x - 1)])
            && properties
                .can_replace
                .contains(&future_sand[(y + 1) * sand_grid.width + (x - 1)])
        {
            future_sand[(y + 1) * sand_grid.width + (x - 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[(y + 1) * sand_grid.width + (x - 1)],
						properties);
            return true;
        }
    }

    false
}

pub fn flow_left_right(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    properties: &SandProperties,
) -> bool {
    if rand::random() {
        if x > 0
            && properties
                .can_replace
                .contains(&sand_grid.grid[y * sand_grid.width + (x - 1)])
            && properties
                .can_replace
                .contains(&future_sand[y * sand_grid.width + (x - 1)])
        {
            future_sand[y * sand_grid.width + (x - 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[y * sand_grid.width + (x - 1)],
						properties
						);
            return true;
        } else if x < sand_grid.width - 1
            && properties
                .can_replace
                .contains(&sand_grid.grid[y * sand_grid.width + (x + 1)])
            && properties
                .can_replace
                .contains(&future_sand[y * sand_grid.width + (x + 1)])
        {
            future_sand[y * sand_grid.width + (x + 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[y * sand_grid.width + (x + 1)],
						&properties);
            return true;
        }
    } else {
        if x < sand_grid.width - 1
            && properties
                .can_replace
                .contains(&sand_grid.grid[y * sand_grid.width + (x + 1)])
            && properties
                .can_replace
                .contains(&future_sand[y * sand_grid.width + (x + 1)])
        {
            future_sand[y * sand_grid.width + (x + 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[y * sand_grid.width + (x + 1)],
						&properties);
            return true;
        } else if x > 0
            && properties
                .can_replace
                .contains(&sand_grid.grid[y * sand_grid.width + (x - 1)])
            && properties
                .can_replace
                .contains(&future_sand[y * sand_grid.width + (x - 1)])
        {
            future_sand[y * sand_grid.width + (x - 1)] = 
				replace(sand_grid.grid[y * sand_grid.width + x],
						sand_grid.grid[y * sand_grid.width + (x - 1)],
						&properties);
            return true;
        }
    }

    false
}

pub fn count_neighbors(
	x: usize,
    y: usize,
    sand_grid: &SandGrid,
	sand: Sand
) -> u32 {
	const NEIGHBOR_X: [isize; 4] = [ -1, 1, 0, 0 ];
	const NEIGHBOR_Y: [isize; 4] = [ 0, 0, -1, 1 ];

	let mut count = 0;

	for i in 0..4 {
		let nx = x as isize + NEIGHBOR_X[i];
		let ny = y as isize + NEIGHBOR_Y[i];
		
		if nx < 0 || ny < 0 || nx >= sand_grid.width as isize || ny >= sand_grid.height as isize {
			continue;	
		}

		if sand_grid.grid[(sand_grid.width as isize * ny + nx) as usize] == sand {
			count += 1;	
		}
	}

	count
}
