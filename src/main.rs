#![windows_subsystem = "windows"]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::time::Instant;

mod sand;
use sand::{Sand, SandGrid};

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

    let mut sand_grid = SandGrid::new(WIDTH, HEIGHT);
	let mut sand_grid_future = sand_grid.grid.clone();

    let mut selected_sand_ind = 0;
	let mut radius = 4;
	let mut paused = false;

    let mut timer = 0.0;
    let mut frame = 0u32;

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
		Sand::Air
	];

    'running: loop {
        let start = Instant::now();

        canvas.clear();

        let mouse_state = event_pump.mouse_state();

        //Place sand
        if mouse_state.left() {
            let mousex = mouse_state.x() as usize / 16;
            let mousey = mouse_state.y();

			if mousey < 16 && mousex < sand_menu.len() {
				selected_sand_ind = mousex;	
			} else {
				let scalex = 800 / WIDTH;
            	let scaley = 600 / HEIGHT;
            	let mousex = mouse_state.x() / scalex as i32;
            	let mousey = mouse_state.y() / scaley as i32;
				sand_grid.place_sand(sand_menu[selected_sand_ind], mousex, mousey, radius);	
			}
        }

        if timer > 1.0 / 60.0 && !paused {
			let start_sand_update = Instant::now();

            sand::update_sand(&sand_grid, &mut sand_grid_future, frame);

            //Copy the future array in the sand buffer
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    sand_grid.grid[y * WIDTH + x] = sand_grid_future[y * WIDTH + x];
					if sand_grid.grid[y * WIDTH + x] == Sand::Delete {
						sand_grid.grid[y * WIDTH + x] = Sand::Air;
					}
                }
            }

			let time_passed = start_sand_update.elapsed().as_millis();
			println!("{time_passed} ms to update sand");

            timer = 0.0;
            frame += 1;	
        }

		//Put the sand grid into the grid
       	sand.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
       	    for y in 0..HEIGHT {
       	        for x in 0..WIDTH {
       	            let color = sand::sand_color(sand_grid.grid[y * WIDTH + x]);
       	            let pixel_pos = WIDTH * 4 * y + x * 4;
       	            pixels[pixel_pos + 1] = color.r;
       	            pixels[pixel_pos + 2] = color.g;
       	            pixels[pixel_pos + 3] = color.b;
       	        }
       	    }
			
			let scalex = (800 / WIDTH) as isize;
           	let scaley = (600 / HEIGHT) as isize;
			let mousex = mouse_state.x() as isize / scalex;
            let mousey = mouse_state.y() as isize / scaley;

			for y in (mousey - radius as isize)..(mousey as isize + radius as isize) {
				for x in (mousex - radius as isize)..(mousex as isize + radius as isize) {
					if y < 0 || x < 0 || x as usize >= WIDTH || y as usize >= HEIGHT {
						continue;	
					}

					if (y - mousey) * (y - mousey) + (x - mousex) * (x - mousex) > (radius * radius) as isize {
						continue;	
					}

					let pixel_pos = WIDTH * 4 * y as usize + x as usize * 4;
       	            pixels[pixel_pos + 1] /= 4;
       	            pixels[pixel_pos + 2] /= 4;
       	            pixels[pixel_pos + 3] /= 4;
					pixels[pixel_pos + 1] *= 3;
       	            pixels[pixel_pos + 2] *= 3;
       	            pixels[pixel_pos + 3] *= 3;
				}
			}
       	})
       	.unwrap();

        canvas.copy(&sand, None, None).unwrap();

		//Display the menu
		canvas.set_draw_color(Color::RGB(64, 64, 64));
		canvas.fill_rect(Rect::new(0, 0, 800, 16)).unwrap();
		for i in 0..sand_menu.len() {
			canvas.set_draw_color(sand::sand_color(sand_menu[i]));
			if i == selected_sand_ind {
				canvas.fill_rect(Rect::new(i as i32 * 16 + 2, 2, 12, 12)).unwrap();
			} else {
				canvas.fill_rect(Rect::new(i as i32 * 16, 0, 16, 16)).unwrap();
			}
		}

        for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => { break 'running; }
				Event::MouseWheel { y, .. } => {
					//Increase draw radius
					if y > 0 {
						radius += 1;
						if radius > 64 {
							radius = 64;	
						} 
					} else if y < 0 {
						radius -= 1;	
						if radius < 2 {
							radius = 2;
						}	
					}
				}
				Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
					paused = !paused;
				}
				_ => {}	
			}
        }
		
        canvas.present();

        let time_passed = start.elapsed().as_secs_f64();
		if !paused {
			timer += time_passed;
		}
    }
}
