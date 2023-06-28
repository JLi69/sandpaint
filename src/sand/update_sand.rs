use super::{
    sand_physics,
    sand_properties::{SandProperties, SandSimulationProperties},
    Sand, SandGrid,
};

fn count_neighbors(x: usize, y: usize, sand_grid: &SandGrid, sand: Sand) -> u32 {
    const NEIGHBOR_X: [isize; 4] = [-1, 1, 0, 0];
    const NEIGHBOR_Y: [isize; 4] = [0, 0, -1, 1];

    let mut count = 0;

    for i in 0..4 {
        let nx = x as isize + NEIGHBOR_X[i];
        let ny = y as isize + NEIGHBOR_Y[i];

        if sand_grid.out_of_bounds(nx, ny) {
            continue;
        }

        if sand_grid.get_sand(nx as usize, ny as usize) == sand {
            count += 1;
        }
    }

    count
}

pub fn update_particle(x: usize, y: usize, sand_grid: &mut SandGrid, properties: &SandProperties) {
    if y == sand_grid.height - 1 {
        return;
    }

    if sand_physics::fall_down(x, y, sand_grid, properties) {
        return;
    }

    if sand_physics::fall_left_right(x, y, sand_grid, properties) {
        return;
    }
}

pub fn update_liquid(x: usize, y: usize, sand_grid: &mut SandGrid, properties: &SandProperties) {
    if y == sand_grid.height - 1 {
        sand_physics::flow_left_right(x, y, sand_grid, properties);
        return;
    }

    if sand_physics::fall_down(x, y, sand_grid, properties) {
        return;
    }

    if rand::random() {
        if sand_physics::fall_left_right(x, y, sand_grid, properties) {
            return;
        }

        if sand_physics::flow_left_right(x, y, sand_grid, properties) {
            return;
        }
    } else {
        if sand_physics::flow_left_right(x, y, sand_grid, properties) {
            return;
        }

        if sand_physics::fall_left_right(x, y, sand_grid, properties) {
            return;
        }
    }
}

pub fn explode(
    x: usize,
    y: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
    radius: isize,
) {
    sand_grid.set_sand(x, y, Sand::Fire);

    let mut angle = 0.0f64;
    while angle < 3.14159 * 2.0 {
        let mut posx = 0.0f64;
        let mut posy = 0.0f64;
        while (posx * posx + posy * posy).sqrt() < radius as f64 {
            posx += angle.cos() * 2.0;
            posy += angle.sin() * 2.0;
            let trans_x = posx + x as f64;
            let trans_y = posy + y as f64;

            if trans_x < 0.0
                || trans_y < 0.0
                || trans_x >= sand_grid.width as f64
                || trans_y >= sand_grid.height as f64
            {
                break;
            }

            let trans_x = trans_x.floor() as usize;
            let trans_y = trans_y.floor() as usize;

            if properties
                .can_replace
                .contains(&sand_grid.get_sand(trans_x, trans_y))
            {
                sand_grid.set_sand(trans_x, trans_y, Sand::Fire);
                sand_grid.set_updated(trans_x, trans_y);
            } else {
                break;
            }
        }
        angle += 0.05;
    }
}

pub fn update_explosive(
    x: usize,
    y: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
    sand_sim_properties: &SandSimulationProperties,
) {
    let explosion_property;
    match sand_sim_properties.get_sand_property(Sand::Explosion) {
        Some(sand_prop) => explosion_property = sand_prop,
        _ => return,
    }

    if count_neighbors(x, y, &sand_grid, Sand::Lava) >= 1
        || count_neighbors(x, y, sand_grid, Sand::Fire) >= 1
    {
        sand_grid.set_sand(x, y, Sand::Fire);
        explode(x, y, sand_grid, &explosion_property, 64);
        return;
    }

    transform_from_neighbors(x, y, Sand::Lava, Sand::Fire, sand_grid, 1, 4, 1.0);
    transform_from_neighbors(x, y, Sand::Fire, Sand::Fire, sand_grid, 1, 4, 1.0);
    update_particle(x, y, sand_grid, properties);
}

pub fn update_fire(x: usize, y: usize, sand_grid: &mut SandGrid, properties: &SandProperties) {
    let mut flammable_count = 0;

    for yoff in -2isize..2isize {
        for xoff in -2isize..2isize {
            if sand_grid.out_of_bounds(xoff + x as isize, yoff + y as isize) {
                continue;
            }

            if xoff * xoff + yoff * yoff > 2 * 2 {
                continue;
            }

            let posx = (xoff + x as isize) as usize;
            let posy = (yoff + y as isize) as usize;

            if properties
                .can_replace
                .contains(&sand_grid.get_sand(posx, posy))
                && sand_grid.get_sand(posx, posy) != Sand::Air
            {
                if rand::random::<f64>().fract() < 0.01 {
                    sand_grid.set_sand(posx, posy, Sand::Fire);
                    sand_grid.set_updated(posx, posy);
                }
                flammable_count += 1;
            } else if sand_grid.get_sand(posx, posy) == Sand::Air && rand::random::<f64>() < 0.065 {
                sand_grid.set_sand(posx, posy, Sand::Fire);
                sand_grid.set_updated(posx, posy);
            }
        }
    }

    if !((flammable_count >= 1)
        || (count_neighbors(x, y, sand_grid, Sand::Fire) >= 2 && rand::random::<f64>() < 0.8))
    {
        sand_grid.set_sand(x, y, Sand::Air);
        sand_grid.set_updated(x, y);
    }
}

//Transforms the cell at the position based on the number
//of neighboring cells of a certain type
pub fn transform_from_neighbors(
    x: usize,
    y: usize,
    neighbor: Sand,
    turn_into: Sand,
    sand_grid: &mut SandGrid,
    min_count: u32,
    max_count: u32,
    probability: f64,
) {
    if sand_grid.get_updated(x, y) {
        return;
    }

    if count_neighbors(x, y, sand_grid, neighbor) >= min_count
        && count_neighbors(x, y, sand_grid, neighbor) <= max_count
        && rand::random::<f64>() < probability
    {
        sand_grid.set_sand(x, y, turn_into);
        sand_grid.set_updated(x, y);
    }
}
