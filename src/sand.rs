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

impl SandProperties {
	pub fn empty() -> Self {
		SandProperties { 
			can_replace: HashSet::<Sand>::new(), 
			replace_with: HashMap::<Sand, Sand>::new() 
		}
	}

	pub fn from_vecs(can_replace: Option<Vec<Sand>>,
					 replace_with: Option<Vec<(Sand, Sand)>>) -> Self {
		let mut properties = SandProperties::empty();

		properties.add_replaceable(Sand::Air);

		match can_replace {
			Some(can_replace) => { 
				can_replace
					.into_iter()
					.for_each(|sand| properties.add_replaceable(sand)); 
			}
			_ => {}
		}

		match replace_with {
			Some(replace_with) => {
				replace_with
					.into_iter()
					.for_each(|(sand1, sand2)| properties.add_replace_with(sand1, sand2));	
			}
			_ => {}
		}
	
		properties
	}

	pub fn add_replaceable(&mut self, sand: Sand) {
		self.can_replace.insert(sand);
	}

	pub fn add_replace_with(&mut self, can_replace: Sand, replace_with: Sand) {
		self.replace_with.insert(can_replace, replace_with);
	}

	pub fn replace(&self, sand: Sand, sand_to_replace: Sand) -> Sand {
		match self.replace_with.get(&sand_to_replace) {
			Some(s) => *s,
			_ => sand
		}
	}
}

impl SandGrid {
	fn out_of_bounds(&self, x: isize, y: isize) -> bool {
		x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize
	}

	pub fn new(w: usize, h: usize) -> Self {
		SandGrid {
			grid: vec![Sand::Air; w * h],
			width: w,
			height: h
		}
	}

	//Place sand in a circle centered at posx and posy
	pub fn place_sand(&mut self, sand: Sand, posx: i32, posy: i32, radius: u32) {
		for y in (posy - radius as i32)..(posy + radius as i32) {
	        for x in (posx - radius as i32)..(posx + radius as i32) {
	            if self.out_of_bounds(x as isize, y as isize) {
	                continue;
	            }
	
	            if (y - posy) * (y - posy) + (x - posx) * (x - posx) < (radius * radius) as i32 {
	                self.grid[y as usize * self.width + x as usize] = sand;
	            }
	        }
	    }
	}

	pub fn get_sand(&self, x: usize, y: usize) -> Sand {
		if self.out_of_bounds(x as isize, y as isize) {
			return Sand::Air	
		}

		self.grid[y * self.width + x]
	}

	pub fn set_sand(&mut self, x: usize, y: usize, sand: Sand) {
		if self.out_of_bounds(x as isize, y as isize) {
			return	
		}

		self.grid[y * self.width + x] = sand;	
	}
}

fn update_pixel(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
) {
    match sand_grid.get_sand(x, y) {
        Sand::Air => {}
        Sand::Sand => {
			let can_replace = vec![
				Sand::Water,
				Sand::Fire,
				Sand::Oil,
				Sand::Acid
			];

			let replace_with = vec![
				(Sand::Acid, Sand::Acid)
			];

            let sand_property = SandProperties::from_vecs(
				Some(can_replace), 
				Some(replace_with)
			);

            update_sand::update_particle(x, y, sand_grid, future_sand, &sand_property);
		}
        Sand::Water => {
			let can_replace = vec![
				Sand::Fire,
				Sand::Lava
			];

			let replace_with = vec![
				(Sand::Lava, Sand::Stone)	
			];

            let sand_property = SandProperties::from_vecs(
				Some(can_replace),
				Some(replace_with)
			);

            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);
        }
		Sand::Oil => {
			if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1 &&
			   rand::random::<f64>() < 0.2 {	
				future_sand[sand_grid.width * y + x] = Sand::Fire;
				return	
			}

			let sand_property = SandProperties::from_vecs(None, None);

            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);	
		}
		Sand::Lava => { 
			let mut sand_property = SandProperties::empty();
            sand_property.add_replaceable(Sand::Air);
            sand_property.add_replaceable(Sand::Water);
            sand_property.add_replaceable(Sand::Fire);

            sand_property.add_replace_with(Sand::Water, Sand::Stone);

			update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);			
		}
		Sand::Acid => {
			let can_replace = vec![
				Sand::Wood,
            	Sand::Sand,
            	Sand::Fire,
            	Sand::Stone
			];
			
			let replace_with = vec![
				(Sand::Wood, Sand::Delete),
				(Sand::Sand, Sand::Delete),
				(Sand::Fire, Sand::Delete),
				(Sand::Stone, Sand::Delete)
			];

			let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
            
            update_sand::update_liquid(x, y, sand_grid, future_sand, &sand_property);
		}
		Sand::Fire => {
			let can_replace = vec! [
				Sand::Oil,
				Sand::Wood
			];
			let sand_property = SandProperties::from_vecs(Some(can_replace), None);

			update_sand::update_fire(x, y, sand_grid, future_sand, &sand_property);	
		}
		Sand::Stone => {
			let can_replace = vec![
				Sand::Oil,
				Sand::Water,
				Sand::Acid
			];

			let replace_with = vec![
				(Sand::Acid, Sand::Acid)
			];

			let sand_property = SandProperties::from_vecs(
				Some(can_replace),
				Some(replace_with)
			);

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
			let can_replace = vec![
				Sand::Water,
				Sand::Oil
			];

			let sand_property = SandProperties::from_vecs(Some(can_replace), None);	

			let can_replace = vec![
				Sand::Water,
				Sand::Wood,
				Sand::Fire,
				Sand::Sand,
				Sand::Explosive,
				Sand::Lava
			];

			let explosion_property = SandProperties::from_vecs(Some(can_replace), None); 

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
