use super::{sand_properties::SandProperties, Sand, SandGrid};

pub fn swap(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
) -> bool {
    if sand_grid.out_of_bounds(x1 as isize, y1 as isize)
        || sand_grid.out_of_bounds(x2 as isize, y2 as isize)
    {
        return false;
    }

    if properties.can_sink_in.contains(&sand_grid.get_sand(x2, y2)) &&
	   rand::random() {
        let sand = sand_grid.get_sand(x2, y2);
        sand_grid.set_sand(x2, y2, sand_grid.get_sand(x1, y1));
        sand_grid.set_sand(x1, y1, sand);
        sand_grid.set_updated(x1, y1);
        sand_grid.set_updated(x2, y2);
        return true;
    }

    false
}

//Returns true if it can move down,
//false otherwise
pub fn fall_down(
    x: usize,
    y: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
) -> bool {
    if sand_grid.get_updated(x, y) {
        return false;
    }

    if y == sand_grid.height - 1 {
        return false;
    }

    if sand_grid.space_available(x, y + 1, properties) {
        sand_grid.set_sand(
            x,
            y + 1,
            properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x, y + 1)),
        );
        sand_grid.set_sand(x, y, Sand::Air);
        sand_grid.set_updated(x, y + 1);
        sand_grid.set_updated(x, y);
        return true;
    }

    if swap(x, y, x, y + 1, sand_grid, properties) {
        return true;
    }

    false
}

pub fn fall_left_right(
    x: usize,
    y: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
) -> bool {
    if sand_grid.get_updated(x, y) {
        return false;
    }

    if y == sand_grid.height - 1 {
        return false;
    }

    if rand::random() {
        if x > 0 && sand_grid.space_available(x - 1, y + 1, properties) {
            sand_grid.set_sand(
                x - 1,
                y + 1,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x - 1, y + 1)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x - 1, y + 1);
            sand_grid.set_updated(x, y);
            return true;
        } else if x < sand_grid.width - 1 && sand_grid.space_available(x + 1, y + 1, properties) {
            sand_grid.set_sand(
                x + 1,
                y + 1,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x + 1, y + 1)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x + 1, y + 1);
            sand_grid.set_updated(x, y);
            return true;
        }

        if x > 0 && swap(x, y, x - 1, y + 1, sand_grid, properties) {
            if sand_grid.get_sand(x - 1, y) == Sand::Air {
                swap(x - 1, y, x, y, sand_grid, properties);
            }
            return true;
        } else if swap(x, y, x + 1, y + 1, sand_grid, properties) {
            if sand_grid.get_sand(x + 1, y) == Sand::Air {
                swap(x + 1, y, x, y, sand_grid, properties);
            }
            return true;
        }
    } else {
        if x < sand_grid.width - 1 && sand_grid.space_available(x + 1, y + 1, properties) {
            sand_grid.set_sand(
                x + 1,
                y + 1,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x + 1, y + 1)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x + 1, y + 1);
            sand_grid.set_updated(x, y);
            return true;
        } else if x > 0 && sand_grid.space_available(x - 1, y + 1, properties) {
            sand_grid.set_sand(
                x - 1,
                y + 1,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x - 1, y + 1)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x - 1, y + 1);
            sand_grid.set_updated(x, y);
            return true;
        }

        if swap(x, y, x + 1, y + 1, sand_grid, properties) {
            if sand_grid.get_sand(x + 1, y) == Sand::Air {
                swap(x + 1, y, x, y, sand_grid, properties);
            }
            return true;
        } else if x > 0 && swap(x, y, x - 1, y + 1, sand_grid, properties) {
            if sand_grid.get_sand(x - 1, y) == Sand::Air {
                swap(x - 1, y, x, y, sand_grid, properties);
            }
            return true;
        }
    }

    false
}

pub fn flow_left_right(
    x: usize,
    y: usize,
    sand_grid: &mut SandGrid,
    properties: &SandProperties,
) -> bool {
    if sand_grid.get_updated(x, y) {
        return false;
    }

    if rand::random() {
        if x > 0 && sand_grid.space_available(x - 1, y, properties) {
            sand_grid.set_sand(
                x - 1,
                y,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x - 1, y)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x - 1, y);
            sand_grid.set_updated(x, y);
            return true;
        } else if x < sand_grid.width - 1 && sand_grid.space_available(x + 1, y, properties) {
            sand_grid.set_sand(
                x + 1,
                y,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x + 1, y)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x + 1, y);
            sand_grid.set_updated(x, y);
            return true;
        }

        if x > 0 && swap(x, y, x - 1, y, sand_grid, properties) {
            return true;
        } else if swap(x, y, x + 1, y, sand_grid, properties) {
            return true;
        }
    } else {
        if x < sand_grid.width - 1 && sand_grid.space_available(x + 1, y, properties) {
            sand_grid.set_sand(
                x + 1,
                y,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x + 1, y)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x + 1, y);
            sand_grid.set_updated(x, y);
            return true;
        } else if x > 0 && sand_grid.space_available(x - 1, y, properties) {
            sand_grid.set_sand(
                x - 1,
                y,
                properties.replace(sand_grid.get_sand(x, y), sand_grid.get_sand(x - 1, y)),
            );
            sand_grid.set_sand(x, y, Sand::Air);
            sand_grid.set_updated(x - 1, y);
            sand_grid.set_updated(x, y);
            return true;
        }

        if swap(x, y, x + 1, y, sand_grid, properties) {
            return true;
        } else if x > 0 && swap(x, y, x - 1, y, sand_grid, properties) {
            return true;
        }
    }

    false
}
