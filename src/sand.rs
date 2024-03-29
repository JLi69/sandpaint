use sdl2::pixels::Color;

mod sand_physics;
pub mod sand_properties;
mod update_sand;

use sand_properties::{SandProperties, SandSimulationProperties};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::enum_variant_names)]
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
    OutOfBounds,
}

#[derive(PartialEq, Clone)]
struct SandParticle {
    sand_type: Sand,
    updated: bool,
    can_update: bool,
}

pub struct SandGrid {
    grid: Vec<SandParticle>,
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
        Sand::Explosion => Color::RED,
        _ => Color::WHITE,
    }
}

pub fn inside_circle(circle_x: i32, circle_y: i32, radius: i32, x: i32, y: i32) -> bool {
    (circle_y - y) * (circle_y - y) + (circle_x - x) * (circle_x - x) < (radius * radius)
}

impl SandGrid {
    pub fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize
    }

    pub fn new(w: usize, h: usize) -> Self {
        SandGrid {
            grid: vec![
                SandParticle {
                    sand_type: Sand::Air,
                    updated: false,
                    can_update: false
                };
                w * h
            ],
            width: w,
            height: h,
        }
    }

    fn set_adjacent_can_update(&mut self, x: usize, y: usize) {
        const ADJ_X: [isize; 8] = [0, 0, -1, 1, -1, -1, 1, 1];
        const ADJ_Y: [isize; 8] = [-1, 1, 0, 0, -1, 1, -1, 1];

        for i in 0..8 {
            let posx = x as isize + ADJ_X[i];
            let posy = y as isize + ADJ_Y[i];
            if self.out_of_bounds(posx, posy) {
                continue;
            }
            self.set_can_update(posx as usize, posy as usize);
        }
    }

    fn check_space_nearby(&self, x: usize, y: usize, sand_property: &SandProperties) -> bool {
        const ADJ_X: [isize; 8] = [0, 0, -1, 1, -1, -1, 1, 1];
        const ADJ_Y: [isize; 8] = [-1, 1, 0, 0, -1, 1, -1, 1];

        for i in 0..8 {
            let posx = (x as isize + ADJ_X[i]) as usize;
            let posy = (y as isize + ADJ_Y[i]) as usize;
            if self.space_available(posx, posy, sand_property)
                || sand_property
                    .can_sink_in
                    .contains(&self.get_sand(posx, posy))
            {
                return true;
            }
        }

        false
    }

    //Place sand in a circle centered at posx and posy
    pub fn place_sand(&mut self, sand: Sand, posx: i32, posy: i32, radius: u32) {
        for y in (posy - radius as i32)..(posy + radius as i32) {
            for x in (posx - radius as i32)..(posx + radius as i32) {
                if self.out_of_bounds(x as isize, y as isize) {
                    continue;
                }

                if inside_circle(posx, posy, radius as i32, x, y) {
                    self.set_sand(x as usize, y as usize, sand);
                    self.grid[y as usize * self.width + x as usize].can_update = true;
                    self.grid[y as usize * self.width + x as usize].updated = false;
                    self.set_adjacent_can_update(x as usize, y as usize);
                }
            }
        }
    }

    pub fn get_sand(&self, x: usize, y: usize) -> Sand {
        if self.out_of_bounds(x as isize, y as isize) {
            return Sand::OutOfBounds;
        }

        self.grid[y * self.width + x].sand_type
    }

    pub fn set_sand(&mut self, x: usize, y: usize, sand: Sand) {
        if self.out_of_bounds(x as isize, y as isize) {
            return;
        }

        self.grid[y * self.width + x].sand_type = sand;
    }

    pub fn set_updated(&mut self, x: usize, y: usize) {
        if self.out_of_bounds(x as isize, y as isize) {
            return;
        }

        self.grid[y * self.width + x].updated = true;
    }

    pub fn set_can_update(&mut self, x: usize, y: usize) {
        if self.out_of_bounds(x as isize, y as isize) {
            return;
        }

        self.grid[y * self.width + x].can_update = true;
    }

    pub fn get_updated(&self, x: usize, y: usize) -> bool {
        if self.out_of_bounds(x as isize, y as isize) {
            return false;
        }

        self.grid[y * self.width + x].updated
    }

    pub fn space_available(&self, x: usize, y: usize, properties: &SandProperties) -> bool {
        if self.out_of_bounds(x as isize, y as isize) {
            return false;
        }

        properties.can_replace.contains(&self.get_sand(x, y))
            && !self.grid[y * self.width + x].updated
    }

    fn invert_x_on_even(&self, x: usize, frame: u32) -> usize {
        if frame % 2 == 0 {
            return self.width - 1 - x;
        }

        x
    }

    pub fn update_sand(&mut self, sand_sim_properties: &SandSimulationProperties, frame: u32) {
        //Update the sand grid
        for y in 0..self.height {
            for xval in 0..self.width {
                let x = self.invert_x_on_even(xval, frame);
                self.update_pixel(x, y, sand_sim_properties);
            }
        }

        for i in 0..self.grid.len() {
            let (x, y) = (i % self.width, i / self.width);

            if self.grid[i].updated || self.grid[i].sand_type == Sand::Fire {
                self.set_can_update(x, y);
                self.set_adjacent_can_update(x, y);
            }

            self.grid[i].updated = false;
        }
    }

    fn update_pixel(&mut self, x: usize, y: usize, sand_sim_properties: &SandSimulationProperties) {
        if !self.grid[self.width * y + x].can_update {
            return;
        }

        if self.get_updated(x, y) {
            return;
        }

        if self.get_sand(x, y) == Sand::Air {
            return;
        }

        let sand_property_op = sand_sim_properties.get_sand_property(self.get_sand(x, y));
        let sand_property = match sand_property_op {
            Some(sand_prop) => sand_prop,
            _ => return,
        };

        let sand = self.get_sand(x, y);

        match sand {
            Sand::Sand => {
                update_sand::update_particle(x, y, self, sand_property);
            }
            Sand::Water => {
                update_sand::update_liquid(x, y, self, sand_property);
            }
            Sand::Oil => {
                update_sand::transform_from_neighbors(
                    x,
                    y,
                    Sand::Lava,
                    Sand::Fire,
                    self,
                    1,
                    4,
                    0.2,
                );
                update_sand::update_liquid(x, y, self, sand_property);
            }
            Sand::Lava => {
                update_sand::update_liquid(x, y, self, sand_property);
            }
            Sand::Acid => {
                update_sand::update_liquid(x, y, self, sand_property);
            }
            Sand::Fire => {
                update_sand::update_fire(x, y, self, sand_property);
            }
            Sand::Stone => {
                sand_physics::fall_down(x, y, self, sand_property);
            }
            Sand::Wood => {
                update_sand::transform_from_neighbors(
                    x,
                    y,
                    Sand::Lava,
                    Sand::Fire,
                    self,
                    1,
                    4,
                    0.2,
                );
            }
            Sand::Explosive => {
                update_sand::update_explosive(x, y, self, sand_property, sand_sim_properties);
            }
            _ => {}
        }

        if self.get_sand(x, y) != sand {
            self.update_pixel(x, y, sand_sim_properties);
        }

        if !self.check_space_nearby(x, y, sand_property) && !self.get_updated(x, y) {
            self.grid[self.width * y + x].can_update = false;
        }
    }
}
