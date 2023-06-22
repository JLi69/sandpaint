use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::{PixelFormatEnum, Color};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sand {
	Air,
	Sand,
	Water,
	Wall
}

fn sand_color(sand: Sand) -> Color {
	match sand {
		Sand::Air => { Color::WHITE }
		Sand::Sand => { Color::YELLOW }
		Sand::Water => { Color::BLUE }
		Sand::Wall => { Color::GRAY }
	}
}

fn update_sand(
	sand_grid: &[Sand],
	future_sand: &mut [Sand],
	frame: u32,
	width: usize, 
	height: usize) 
{	
	//Clear the future buffer	
	for y in 0..height {
		for x in 0..width {
			future_sand[y * width + x] = Sand::Air;	
		}
	}

	//Update the sand grid
	for y in 0..height {
		for xval in 0..width {
			
			let mut x = xval;

			if frame % 2 == 0 {
				x = width - 1 - xval;
			}

			match sand_grid[y * width + x] {	
				Sand::Sand => {
					if y == height - 1 {
						future_sand[y * width + x] = Sand::Sand;
						continue;	
					}

					if sand_grid[(y + 1) * width + x] == Sand::Air {
						future_sand[(y + 1) * width + x] = Sand::Sand;
						continue;	
					}

					if rand::random() {
						if x > 0 && sand_grid[(y + 1) * width + (x - 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x - 1)] = Sand::Sand;
							continue;
						} else if x < width - 1 && sand_grid[(y + 1) * width + (x + 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x + 1)] = Sand::Sand;	
							continue;
						}
					} else {
						if x < width - 1 && sand_grid[(y + 1) * width + (x + 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x + 1)] = Sand::Sand;
							continue;
						} else if x > 0 && sand_grid[(y + 1) * width + (x - 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x - 1)] = Sand::Sand;	
							continue;
						}
					}

					future_sand[y * width + x] = Sand::Sand;
				}
				Sand::Water => {	
					if y == height - 1 {
						if rand::random() {
							if x > 0 && sand_grid[y * width + (x - 1)] == Sand::Air &&
							   future_sand[y * width + (x - 1)] == Sand::Air {
								future_sand[y * width + (x - 1)] = Sand::Water;
								continue;
							} else if x < width - 1 && sand_grid[y * width + (x + 1)] == Sand::Air &&
							  future_sand[y * width + (x + 1)] == Sand::Air {
								future_sand[y * width + (x + 1)] = Sand::Water;	
								continue;
							}
						} else {
							if x < width - 1 && sand_grid[y * width + (x + 1)] == Sand::Air &&
							  future_sand[y * width + (x + 1)] == Sand::Air {
								future_sand[y * width + (x + 1)] = Sand::Water;	
								continue;
							} else if x > 0 && sand_grid[y * width + (x - 1)] == Sand::Air &&
							   future_sand[y * width + (x - 1)] == Sand::Air {
								future_sand[y * width + (x - 1)] = Sand::Water;
								continue;
							}
						}

						future_sand[y * width + x] = Sand::Water;
						continue;	
					}

					if sand_grid[(y + 1) * width + x] == Sand::Air {
						future_sand[(y + 1) * width + x] = Sand::Water;
						continue;	
					}

					if rand::random() {
						if x > 0 && sand_grid[(y + 1) * width + (x - 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x - 1)] = Sand::Water;
							continue;
						} else if x < width - 1 && sand_grid[(y + 1) * width + (x + 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x + 1)] = Sand::Water;	
							continue;
						}
					} else {	
						if x < width - 1 && sand_grid[(y + 1) * width + (x + 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x + 1)] = Sand::Water;
							continue;
						} else if x > 0 && sand_grid[(y + 1) * width + (x - 1)] == Sand::Air {
							future_sand[(y + 1) * width + (x - 1)] = Sand::Water;	
							continue;
						}
					}

					if rand::random() {
						if x > 0 && sand_grid[y * width + (x - 1)] == Sand::Air &&
						   future_sand[y * width + (x - 1)] == Sand::Air {
							future_sand[y * width + (x - 1)] = Sand::Water;
							continue;
						} else if x < width - 1 && sand_grid[y * width + (x + 1)] == Sand::Air &&
						  future_sand[y * width + (x + 1)] == Sand::Air {
							future_sand[y * width + (x + 1)] = Sand::Water;	
							continue;
						}
					} else {
						if x < width - 1 && sand_grid[y * width + (x + 1)] == Sand::Air &&
						  future_sand[y * width + (x + 1)] == Sand::Air {
							future_sand[y * width + (x + 1)] = Sand::Water;	
							continue;
						} else if x > 0 && sand_grid[y * width + (x - 1)] == Sand::Air &&
						   future_sand[y * width + (x - 1)] == Sand::Air {
							future_sand[y * width + (x - 1)] = Sand::Water;
							continue;
						}
					}

					future_sand[y * width + x] = Sand::Water;	
				}
				Sand::Wall => {	
					future_sand[y * width + x] = Sand::Wall;	
				}
				_ => {}
			}
		}
	}	
}

fn main() {
	let ctx = sdl2::init().unwrap();
	let vid_subsystem = ctx.video().unwrap();

	let window = vid_subsystem.window("Sandpaint", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas =
		window
			.into_canvas()
			.present_vsync()
			.build()
			.unwrap();

	const WIDTH: usize = 200;
	const HEIGHT: usize = 150;

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
			let mousex = mouse_state.x() / 4;
			let mousey = mouse_state.y() / 4;

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
			update_sand(&sand_grid, &mut sand_grid_future, frame, WIDTH, HEIGHT);		
			
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
					let color = sand_color(sand_grid[y * WIDTH + x]);
					let pixel_pos = WIDTH * 4 * y + x * 4;
					pixels[pixel_pos + 1] = color.r;
					pixels[pixel_pos + 2] = color.g;
					pixels[pixel_pos + 3] = color.b;
				}
			}
		}).unwrap();
		
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
