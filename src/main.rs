#![windows_subsystem = "windows"]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Instant;

mod sand;
use sand::{sand_properties::SandSimulationProperties, Sand, SandGrid};

struct SandSimClock {
    frame: u32,
    timer: f64,
    dt: f64,
    paused: bool,
    quit: bool,
}

//If scroll direction < 0, decrease radius size,
//if > 0, increase radius size
fn change_brush_size(
    brush_radius: u32,
    min_radius: u32,
    max_radius: u32,
    scroll_direction: i32,
) -> u32 {
    if scroll_direction < 0 && brush_radius > min_radius {
        return brush_radius - 1;
    }

    if scroll_direction > 0 && brush_radius < max_radius {
        return brush_radius + 1;
    }

    return brush_radius;
}

fn display_sand_select(
    canvas: &mut Canvas<Window>,
    sand_menu: &[Sand],
    selected_ind: usize,
) -> Result<(), String> {
    //Display the menu
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas
        .fill_rect(Rect::new(0, 0, 800, 16))
        .map_err(|e| e.to_string())?;
    for i in 0..sand_menu.len() {
        canvas.set_draw_color(sand::sand_color(sand_menu[i]));
        if i == selected_ind {
            canvas
                .fill_rect(Rect::new(i as i32 * 16 + 2, 2, 12, 12))
                .map_err(|e| e.to_string())?;
        } else {
            canvas
                .fill_rect(Rect::new(i as i32 * 16, 0, 16, 16))
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn update_sand(
    sand_grid: &mut SandGrid,
    sand_sim_properties: &SandSimulationProperties,
    sim_clock: &mut SandSimClock,
) {
    if sim_clock.timer > 1.0 / 60.0 && !sim_clock.paused {
        let start_sand_update = Instant::now();

        sand_grid.update_sand(&sand_sim_properties, sim_clock.frame);

        let time_passed = start_sand_update.elapsed().as_millis();
        println!("{time_passed} ms to update sand");

        sim_clock.timer = 0.0;
        sim_clock.frame += 1;
    }

    if !sim_clock.paused {
        sim_clock.timer += sim_clock.dt;
    }
}

fn display_sand_grid(pixels: &mut [u8], sand_grid: &SandGrid) {
    for y in 0..sand_grid.height {
        for x in 0..sand_grid.width {
            let color = sand::sand_color(sand_grid.get_sand(x, y));
            let pixel_pos = sand_grid.width * 4 * y + x * 4;
            pixels[pixel_pos + 1] = color.r;
            pixels[pixel_pos + 2] = color.g;
            pixels[pixel_pos + 3] = color.b;	
		}
    }
}

fn display_brush(
    pixels: &mut [u8],
    mousex: isize,
    mousey: isize,
    radius: u32,
    sand_grid: &SandGrid,
) {
    for y in (mousey - radius as isize)..(mousey as isize + radius as isize) {
        for x in (mousex - radius as isize)..(mousex as isize + radius as isize) {
            if sand_grid.out_of_bounds(x, y) {
                continue;
            }

            if !sand::inside_circle(
                mousex as i32,
                mousey as i32,
                radius as i32,
                x as i32,
                y as i32,
            ) {
                continue;
            }

            let pixel_pos = sand_grid.width * 4 * y as usize + x as usize * 4;
            pixels[pixel_pos + 1] /= 4;
            pixels[pixel_pos + 2] /= 4;
            pixels[pixel_pos + 3] /= 4;
            pixels[pixel_pos + 1] *= 3;
            pixels[pixel_pos + 2] *= 3;
            pixels[pixel_pos + 3] *= 3;
        }
    }
}

fn mouse_place_sand(
    event_pump: &EventPump,
    sand_grid: &mut SandGrid,
    sand_menu: &[Sand],
    selected_ind: usize,
    radius: u32,
) {
    let mouse_state = event_pump.mouse_state();

    //Do not place sand if we hovering over the menu
    let mousex = mouse_state.x() as usize / 16;
    let mousey = mouse_state.y();
    if mousey < 16 && mousex < sand_menu.len() {
        return;
    }

    //Place sand
    if mouse_state.left() {
        let scalex = 800 / sand_grid.width;
        let scaley = 600 / sand_grid.height;
        let mousex = mouse_state.x() / scalex as i32;
        let mousey = mouse_state.y() / scaley as i32;
        sand_grid.place_sand(sand_menu[selected_ind], mousex, mousey, radius);
    }
}

fn mouse_select_menu(event_pump: &EventPump, sand_menu: &[Sand], selected_ind: usize) -> usize {
    let mouse_state = event_pump.mouse_state();

    let mousex = mouse_state.x() as usize / 16;
    let mousey = mouse_state.y();

    if mouse_state.left() && mousey < 16 && mousex < sand_menu.len() {
        return mousex;
    }

    selected_ind
}

fn main() -> Result<(), String> {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    let ctx = sdl2::init().map_err(|e| e.to_string())?;
    let vid_subsystem = ctx.video().map_err(|e| e.to_string())?;
    let window = vid_subsystem
        .window("Sandpaint", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = ctx.event_pump().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut sand_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::BGRA8888, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())?;

    let mut sand_grid = SandGrid::new(WIDTH, HEIGHT);

    let mut selected_sand_ind = 0;
    let mut radius = 4;

    let mut sim_clock = SandSimClock {
        frame: 0,
        timer: 0.0,
        dt: 0.0,
        paused: false,
        quit: false,
    };

    let sand_sim_properties = SandSimulationProperties::simulation_sand_properties();

    let sand_menu = [
        Sand::Sand,
        Sand::Water,
        Sand::Wall,
        Sand::Wood,
        Sand::Fire,
        Sand::Oil,
        Sand::Acid,
        Sand::Lava,
        Sand::Stone,
        Sand::Explosive,
        Sand::Air,
    ];

    while !sim_clock.quit {
        let start = Instant::now();

        //Handle mouse events
        let mouse_state = event_pump.mouse_state();
        selected_sand_ind = mouse_select_menu(&event_pump, &sand_menu, selected_sand_ind);
        mouse_place_sand(
            &event_pump,
            &mut sand_grid,
            &sand_menu,
            selected_sand_ind,
            radius,
        );

        //Update sand simulation
        update_sand(&mut sand_grid, &sand_sim_properties, &mut sim_clock);

        //Display sand grid
        canvas.clear();
        sand_texture
            .with_lock(None, |pixels: &mut [u8], _pitch: usize| {
                display_sand_grid(pixels, &sand_grid);
                let scalex = (800 / WIDTH) as isize;
                let scaley = (600 / HEIGHT) as isize;
                let mousex = mouse_state.x() as isize / scalex;
                let mousey = mouse_state.y() as isize / scaley;
                display_brush(pixels, mousex, mousey, radius, &sand_grid);
            })
            .map_err(|e| e.to_string())?;
        canvas
            .copy(&sand_texture, None, None)
            .map_err(|e| e.to_string())?;
        //Display Menu
        display_sand_select(&mut canvas, &sand_menu, selected_sand_ind)
            .map_err(|e| e.to_string())?;
        canvas.present();

        event_pump.poll_iter().for_each(|event| match event {
            Event::Quit { .. } => sim_clock.quit = true,
            Event::MouseWheel { y, .. } => radius = change_brush_size(radius, 1, 64, y),
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                repeat: false,
                ..
            } => sim_clock.paused = !sim_clock.paused,
            _ => {}
        });

        sim_clock.dt = start.elapsed().as_secs_f64();
    }

    Ok(())
}
