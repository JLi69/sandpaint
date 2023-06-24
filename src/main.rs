use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;
use std::time::Instant;

mod sand;
use sand::Sand;

fn main() {
    let ctx = sdl2::init().unwrap();
    let vid_subsystem = ctx.video().unwrap();

    let window = vid_subsystem
        .window("Sandpaint", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;

    let texture_creator = canvas.texture_creator();
    let mut sand = texture_creator
        .create_texture_streaming(PixelFormatEnum::BGRA8888, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut event_pump = ctx.event_pump().unwrap();

    let mut sand_grid = [Sand::Air; WIDTH * HEIGHT];
    let mut sand_grid_future = sand_grid.clone();

    let mut place_sand = Sand::Sand;

    let mut timer = 0.0;
    let mut frame = 0u32;

    'running: loop {
        let start = Instant::now();

        canvas.clear();

        let keyboard_state = event_pump.keyboard_state();
        let mouse_state = event_pump.mouse_state();

        if keyboard_state.is_scancode_pressed(Scancode::Num1) {
            place_sand = Sand::Sand;
        } else if keyboard_state.is_scancode_pressed(Scancode::Num2) {
            place_sand = Sand::Water;
        } else if keyboard_state.is_scancode_pressed(Scancode::Num3) {
            place_sand = Sand::Wall;
        }

        //Place sand
        if mouse_state.left() {
            let scalex = 800 / WIDTH;
            let scaley = 600 / HEIGHT;

            let mousex = mouse_state.x() / scalex as i32;
            let mousey = mouse_state.y() / scaley as i32;

            for y in (mousey - 16)..(mousey + 16) {
                for x in (mousex - 16)..(mousex + 16) {
                    if y < 0 || y >= HEIGHT as i32 || x < 0 || x >= WIDTH as i32 {
                        continue;
                    }

                    if (y - mousey) * (y - mousey) + (x - mousex) * (x - mousex) < 16 {
                        sand_grid[y as usize * WIDTH + x as usize] = place_sand;
                    }
                }
            }
        }

        if timer > 1.0 / 60.0 {
            sand::update_sand(&sand_grid, &mut sand_grid_future, frame, WIDTH, HEIGHT);

            //Copy the future array in the sand buffer
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    sand_grid[y * WIDTH + x] = sand_grid_future[y * WIDTH + x];
                }
            }

            timer = 0.0;
            frame += 1;
        }

        //Display the sand grid
        sand.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let color = sand::sand_color(sand_grid[y * WIDTH + x]);
                    let pixel_pos = WIDTH * 4 * y + x * 4;
                    pixels[pixel_pos + 1] = color.r;
                    pixels[pixel_pos + 2] = color.g;
                    pixels[pixel_pos + 3] = color.b;
                }
            }
        })
        .unwrap();

        canvas.copy(&sand, None, None).unwrap();

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.present();

        let time_passed = start.elapsed().as_secs_f64();
        timer += time_passed;
    }
}
