use sdl2::pixels::Color;
use std::collections::{HashSet, HashMap};

mod move_sand;
mod update_sand;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sand {
    Air,
    Sand,
    Water,
    Wall,
	Wood,
	Fire,
	Oil,
	Acid,
	Lava,
	Stone,
	Explosive,
	Delete
}

pub struct SandProperties {
    pub can_replace: HashSet<Sand>,
	pub replace_with: HashMap<Sand, Sand>
}

pub struct SandGrid {	
    pub grid: Vec<Sand>,
	pub width: usize,
	pub height: usize
}

pub fn sand_color(sand: Sand) -> Color {
    match sand {
        Sand::Air => Color::WHITE,
        Sand::Sand => Color::RGB(255, 200, 0),
        Sand::Water => Color::BLUE,
        Sand::Wall => Color::GRAY,
		Sand::Wood => Color::RGB(128, 64, 0),
		Sand::Fire => Color::RED,
		Sand::Oil => Color::BLACK,
		Sand::Acid => Color::GREEN,
		Sand::Lava => Color::RGB(255, 128, 0),
		Sand::Stone => Color::RGB(180, 180, 180),
		Sand::Explosive => Color::RGB(255, 64, 0),
		_ => Color::BLACK	
	}
}

fn update_pixel(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
) {
    match sand_grid.grid[y * sand_grid.width + x] {
        Sand::Air => {}
        Sand::Sand => {
            let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);
            sand_property.can_replace.insert(Sand::Water);
            sand_property.can_replace.insert(Sand::Fire);
            sand_property.can_replace.insert(Sand::Oil);
            sand_property.can_replace.insert(Sand::Acid);

            sand_property.replace_with.insert(Sand::Acid, Sand::Acid);

            update_sand::update_particle(x, y, sand_grid, future_sand, &sand_property);
		}
        Sand::Water => {
            let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(),
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);
            sand_property.can_replace.insert(Sand::Fire);
            sand_property.can_replace.insert(Sand::Lava);

            sand_property.replace_with.insert(Sand::Lava, Sand::Stone);

            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);
        }
		Sand::Oil => {
			if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1 &&
			   rand::random::<f64>() < 0.2 {	
				future_sand[sand_grid.width * y + x] = Sand::Fire;
				return	
			}

			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);

            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);	
		}
		Sand::Lava => { 
			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);
            sand_property.can_replace.insert(Sand::Water);
            sand_property.can_replace.insert(Sand::Fire);

            sand_property.replace_with.insert(Sand::Water, Sand::Stone);

			update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);			
		}
		Sand::Acid => {
			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);
            sand_property.can_replace.insert(Sand::Wood);
            sand_property.can_replace.insert(Sand::Sand);
            sand_property.can_replace.insert(Sand::Fire);
            sand_property.can_replace.insert(Sand::Stone);

			sand_property.replace_with.insert(Sand::Wood, Sand::Delete);
            sand_property.replace_with.insert(Sand::Sand, Sand::Delete);
            sand_property.replace_with.insert(Sand::Fire, Sand::Delete);
            sand_property.replace_with.insert(Sand::Stone, Sand::Delete);

            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);
		}
		Sand::Fire => {
			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);	
            sand_property.can_replace.insert(Sand::Oil);
            sand_property.can_replace.insert(Sand::Wood);

			update_sand::update_fire(x, y, sand_grid, future_sand, &sand_property);	
		}
		Sand::Stone => {
			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);	
            sand_property.can_replace.insert(Sand::Oil);
            sand_property.can_replace.insert(Sand::Water);
			sand_property.can_replace.insert(Sand::Acid);

            sand_property.replace_with.insert(Sand::Acid, Sand::Acid);

			if move_sand::fall_down(x, y, sand_grid, future_sand, &sand_property) {
				return	
			}

			if future_sand[sand_grid.width * y + x] == Sand::Air {
				future_sand[sand_grid.width * y + x] = Sand::Stone;
			}
		}
		Sand::Wood => {
			if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1 &&
			   rand::random::<f64>() < 0.2 {	
				future_sand[sand_grid.width * y + x] = Sand::Fire;
				return	
			}

			if future_sand[sand_grid.width * y + x] == Sand::Air {
				future_sand[sand_grid.width * y + x] = Sand::Wood;
			}
		}
		Sand::Explosive => {
			let mut sand_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            sand_property.can_replace.insert(Sand::Air);
            sand_property.can_replace.insert(Sand::Water);	
            sand_property.can_replace.insert(Sand::Oil);	

			let mut explosion_property = SandProperties {
                can_replace: HashSet::<Sand>::new(), 
                replace_with: HashMap::<Sand, Sand>::new()
            };
            explosion_property.can_replace.insert(Sand::Air);	
            explosion_property.can_replace.insert(Sand::Oil);
			explosion_property.can_replace.insert(Sand::Wood);
			explosion_property.can_replace.insert(Sand::Fire);
			explosion_property.can_replace.insert(Sand::Sand);
			explosion_property.can_replace.insert(Sand::Explosive);
			explosion_property.can_replace.insert(Sand::Lava);

			if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1 ||
				move_sand::count_neighbors(x, y, sand_grid, Sand::Fire) >= 1 {
				update_sand::explode(x, y, sand_grid, future_sand, &explosion_property, 64);	
				return;
			}

			update_sand::update_particle(x, y, sand_grid, future_sand, &sand_property);
		}
        _ => {
            future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
        }
    }
}

pub fn update_sand(
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    frame: u32,
) {
    //Clear the future buffer
    for y in 0..sand_grid.height {
        for x in 0..sand_grid.width {
            future_sand[y * sand_grid.width + x] = Sand::Air;
        }
    }

    //Update the sand grid
    for y in 0..sand_grid.height {
        for xval in 0..sand_grid.width {
            let mut x = xval;
            if frame % 2 == 0 {
                x = sand_grid.width - 1 - xval;
            }

            update_pixel(x, y, sand_grid, future_sand);
        }
    }
}

pub fn place_sand(sand_grid: &mut SandGrid, sand: Sand, posx: i32, posy: i32, radius: u32) {
	for y in (posy - radius as i32)..(posy + radius as i32) {
        for x in (posx - radius as i32)..(posx + radius as i32) {
            if y < 0 || y >= sand_grid.height as i32 || x < 0 || x >= sand_grid.width as i32 {
                continue;
            }

            if (y - posy) * (y - posy) + (x - posx) * (x - posx) < (radius * radius) as i32 {
                sand_grid.grid[y as usize * sand_grid.width + x as usize] = sand;
            }
        }
    }
}
