use sdl2::pixels::Color;
use std::collections::{HashMap, HashSet};

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
    Explosion,
    Delete,
}

pub struct SandProperties {
    pub can_replace: HashSet<Sand>,
    pub replace_with: HashMap<Sand, Sand>,
}

pub struct SandSimulationProperties(HashMap<Sand, SandProperties>);

pub struct SandGrid {
    pub grid: Vec<Sand>,
    pub width: usize,
    pub height: usize,
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
        _ => Color::BLACK,
    }
}

impl SandProperties {
    pub fn empty() -> Self {
        SandProperties {
            can_replace: HashSet::<Sand>::new(),
            replace_with: HashMap::<Sand, Sand>::new(),
        }
    }

    pub fn from_vecs(
        can_replace: Option<Vec<Sand>>,
        replace_with: Option<Vec<(Sand, Sand)>>,
    ) -> Self {
        let mut properties = Self::empty();

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
            _ => sand,
        }
    }
}

impl SandSimulationProperties {
    pub fn new() -> Self {
        Self(HashMap::<Sand, SandProperties>::new())
    }

    pub fn simulation_sand_properties() -> Self {
        let mut sand_sim_properties = Self::new();

        //Sand
        let can_replace = vec![Sand::Water, Sand::Fire, Sand::Oil, Sand::Acid];

        let replace_with = vec![(Sand::Acid, Sand::Acid)];

        let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
        sand_sim_properties.add_sand_property(Sand::Sand, sand_property);

        //Water
        let can_replace = vec![Sand::Fire, Sand::Lava];

        let replace_with = vec![(Sand::Lava, Sand::Stone)];

        let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
        sand_sim_properties.add_sand_property(Sand::Water, sand_property);

        //Wall
        let sand_property = SandProperties::from_vecs(None, None);
        sand_sim_properties.add_sand_property(Sand::Wall, sand_property);

        //Wood
        let sand_property = SandProperties::from_vecs(None, None);
        sand_sim_properties.add_sand_property(Sand::Wood, sand_property);

        //Fire
        let can_replace = vec![Sand::Oil, Sand::Wood];
        let sand_property = SandProperties::from_vecs(Some(can_replace), None);
        sand_sim_properties.add_sand_property(Sand::Fire, sand_property);

        //Oil
        let sand_property = SandProperties::from_vecs(None, None);
        sand_sim_properties.add_sand_property(Sand::Oil, sand_property);

        //Acid
        let can_replace = vec![Sand::Wood, Sand::Sand, Sand::Fire, Sand::Stone];

        let replace_with = vec![
            (Sand::Wood, Sand::Delete),
            (Sand::Sand, Sand::Delete),
            (Sand::Fire, Sand::Delete),
            (Sand::Stone, Sand::Delete),
        ];

        let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
        sand_sim_properties.add_sand_property(Sand::Acid, sand_property);

        //Lava
        let can_replace = vec![Sand::Air, Sand::Water, Sand::Fire];

        let replace_with = vec![(Sand::Water, Sand::Stone)];

        let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
        sand_sim_properties.add_sand_property(Sand::Lava, sand_property);

        //Stone
        let can_replace = vec![Sand::Oil, Sand::Water, Sand::Acid];

        let replace_with = vec![(Sand::Acid, Sand::Acid)];

        let sand_property = SandProperties::from_vecs(Some(can_replace), Some(replace_with));
        sand_sim_properties.add_sand_property(Sand::Stone, sand_property);

        //Explosive
        let can_replace = vec![Sand::Water, Sand::Oil];

        let sand_property = SandProperties::from_vecs(Some(can_replace), None);
        sand_sim_properties.add_sand_property(Sand::Explosive, sand_property);

        //Explosion
        let can_replace = vec![
            Sand::Water,
            Sand::Wood,
            Sand::Fire,
            Sand::Sand,
            Sand::Explosive,
            Sand::Lava,
        ];

        let explosion_property = SandProperties::from_vecs(Some(can_replace), None);
        sand_sim_properties.add_sand_property(Sand::Explosion, explosion_property);

        sand_sim_properties
    }

    pub fn get_sand_property(&self, s: Sand) -> Option<&SandProperties> {
        self.0.get(&s)
    }

    pub fn add_sand_property(&mut self, s: Sand, properties: SandProperties) {
        self.0.insert(s, properties);
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
            height: h,
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
            return Sand::Air;
        }

        self.grid[y * self.width + x]
    }

    pub fn set_sand(&mut self, x: usize, y: usize, sand: Sand) {
        if self.out_of_bounds(x as isize, y as isize) {
            return;
        }

        self.grid[y * self.width + x] = sand;
    }
}

fn update_pixel(
    x: usize,
    y: usize,
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    sand_sim_properties: &SandSimulationProperties,
) {
    if sand_grid.get_sand(x, y) == Sand::Air {
        return;
    }

    let sand_property_op = sand_sim_properties.get_sand_property(sand_grid.get_sand(x, y));

    let sand_property;
    match sand_property_op {
        Some(sand_prop) => {
            sand_property = sand_prop;
        }
        _ => {
            return;
        }
    }

    match sand_grid.get_sand(x, y) {
        Sand::Sand => {
            update_sand::update_particle(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Water => {
            update_sand::update_liquid(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Oil => {
            update_sand::update_liquid(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Lava => {
            update_sand::update_liquid(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Acid => {
            update_sand::update_liquid(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Fire => {
            update_sand::update_fire(x, y, sand_grid, future_sand, sand_property);
        }
        Sand::Stone => {
            if move_sand::fall_down(x, y, sand_grid, future_sand, sand_property) {
                return;
            }

            if future_sand[sand_grid.width * y + x] == Sand::Air {
                future_sand[sand_grid.width * y + x] = Sand::Stone;
            }
        }
        Sand::Wood => {
            if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1
                && rand::random::<f64>() < 0.2
            {
                future_sand[sand_grid.width * y + x] = Sand::Fire;
                return;
            }

            if future_sand[sand_grid.width * y + x] == Sand::Air {
                future_sand[sand_grid.width * y + x] = Sand::Wood;
            }
        }
        Sand::Explosive => {
            let explosion_prop_op = sand_sim_properties.get_sand_property(Sand::Explosion);
            let explosion_property;
            match explosion_prop_op {
                Some(sand_prop) => {
                    explosion_property = sand_prop;
                }
                _ => {
                    return;
                }
            }

            if move_sand::count_neighbors(x, y, sand_grid, Sand::Lava) >= 1
                || move_sand::count_neighbors(x, y, sand_grid, Sand::Fire) >= 1
            {
                update_sand::explode(x, y, sand_grid, future_sand, &explosion_property, 64);
                return;
            }

            update_sand::update_particle(x, y, sand_grid, future_sand, sand_property);
        }
        _ => {
            future_sand[y * sand_grid.width + x] = sand_grid.grid[y * sand_grid.width + x];
        }
    }
}

pub fn update_sand(
    sand_grid: &SandGrid,
    future_sand: &mut [Sand],
    sand_sim_properties: &SandSimulationProperties,
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

            update_pixel(x, y, sand_grid, future_sand, sand_sim_properties);
        }
    }
}
