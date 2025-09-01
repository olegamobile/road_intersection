use rand::Rng;
use road_intersection::{Direction, Turn, World, WINDOW_WIDTH, WINDOW_HEIGHT, ROAD_WIDTH, ROAD_X, ROAD_Y, INTERSECTION_X_START, INTERSECTION_Y_START, INTERSECTION_X_END, INTERSECTION_Y_END, SOUTHBOUND_LANE_X, NORTHBOUND_LANE_X, WESTBOUND_LANE_Y, EASTBOUND_LANE_Y};


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use road_intersection::vehicle::Vehicle;

const SPAWN_TIMEOUT: Duration = Duration::from_millis(250);

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window("Road Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // Create a texture for the static background
    let mut static_background = texture_creator
        .create_texture_target(None, WINDOW_WIDTH, WINDOW_HEIGHT)
        .map_err(|e| e.to_string())?;

    // Draw the static elements to the new texture
    canvas.with_texture_canvas(&mut static_background, |texture_canvas| {
        // Clear the texture
        texture_canvas.set_draw_color(Color::RGB(200, 200, 200));
        texture_canvas.clear();

        // Draw all the static parts
        draw_roads(texture_canvas).unwrap();
        draw_lanes(texture_canvas).unwrap();
        draw_lane_dividers(texture_canvas).unwrap();
        draw_intersection_elements(texture_canvas).unwrap();
    }).map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;
    let mut world = World::new();
    let mut last_spawn_time = Instant::now();
    let mut random_generation_on = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Up) => handle_spawn_key(&mut world, &mut last_spawn_time, Direction::South),
                    Some(Keycode::Down) => handle_spawn_key(&mut world, &mut last_spawn_time, Direction::North),
                    Some(Keycode::Left) => handle_spawn_key(&mut world, &mut last_spawn_time, Direction::East),
                    Some(Keycode::Right) => handle_spawn_key(&mut world, &mut last_spawn_time, Direction::West),
                    Some(Keycode::R) => {
                        let mut rng = rand::thread_rng();
                        let random_dir = match rng.gen_range(0..4) {
                            0 => Direction::North,
                            1 => Direction::South,
                            2 => Direction::East,
                            _ => Direction::West,
                        };
                        handle_spawn_key(&mut world, &mut last_spawn_time, random_dir);
                    }
                    Some(Keycode::G) => random_generation_on = !random_generation_on,
                    _ => {}
                },
                _ => {}
            }
        }

        if random_generation_on && last_spawn_time.elapsed() >= SPAWN_TIMEOUT {
            let mut rng = rand::thread_rng();
            let random_dir = match rng.gen_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            };
            handle_spawn_key(&mut world, &mut last_spawn_time, random_dir);
        }

        // Update simulation
        world.update();

        // Copy the pre-rendered background
        canvas.copy(&static_background, None, None)?;

        // Draw dynamic elements
        draw_traffic_lights(&mut canvas, &world.controller.current)?;
        draw_vehicles(&mut canvas, &world.vehicles)?;

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn handle_spawn_key(world: &mut World, last_spawn_time: &mut Instant, direction: Direction) {
    if last_spawn_time.elapsed() >= SPAWN_TIMEOUT {
        world.spawn_vehicle(direction);
        *last_spawn_time = Instant::now();
    }
}

fn draw_roads(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.fill_rect(Rect::new(ROAD_X as i32, 0, ROAD_WIDTH, WINDOW_HEIGHT))?;
    canvas.fill_rect(Rect::new(0, ROAD_Y as i32, WINDOW_WIDTH, ROAD_WIDTH))?;
    Ok(())
}

fn draw_lanes(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(120, 120, 120)); // Slightly lighter gray

    // Northbound Lane (South to North)
    let rect_northbound = Rect::new(NORTHBOUND_LANE_X - (ROAD_WIDTH / 4) as i32, 0, ROAD_WIDTH / 2, WINDOW_HEIGHT);
    canvas.fill_rect(rect_northbound)?;
    
    // Southbound Lane (North to South)
    let rect_southbound = Rect::new(SOUTHBOUND_LANE_X - (ROAD_WIDTH / 4) as i32, 0, ROAD_WIDTH / 2, WINDOW_HEIGHT);
    canvas.fill_rect(rect_southbound)?;
    
    // Eastbound Lane (West to East)
    let rect_eastbound = Rect::new(0, EASTBOUND_LANE_Y - (ROAD_WIDTH / 4) as i32, WINDOW_WIDTH, ROAD_WIDTH / 2);
    canvas.fill_rect(rect_eastbound)?;
    
    // Westbound Lane (East to West)
    let rect_westbound = Rect::new(0, WESTBOUND_LANE_Y - (ROAD_WIDTH / 4) as i32, WINDOW_WIDTH, ROAD_WIDTH / 2);
    canvas.fill_rect(rect_westbound)?;
    Ok(())
}

fn draw_lane_dividers(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in 0..15 {
        // Horizontal road
        if i * 60 + 30 < ROAD_X as i32 || i * 60 > (ROAD_X + ROAD_WIDTH) as i32 {
            canvas.fill_rect(Rect::new(i * 60, (ROAD_Y + ROAD_WIDTH / 2 - 2) as i32, 30, 4))?;
        }
        // Vertical road
        if i * 40 + 20 < ROAD_Y as i32 || i * 40 > (ROAD_Y + ROAD_WIDTH) as i32 {
            canvas.fill_rect(Rect::new((ROAD_X + ROAD_WIDTH / 2 - 2) as i32, i * 40, 4, 20))?;
        }
    }
    Ok(())
}

fn draw_intersection_elements(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(200, 200, 200)); // Light gray for intersection outline
    canvas.draw_rect(Rect::new(INTERSECTION_X_START as i32, INTERSECTION_Y_START as i32, ROAD_WIDTH, ROAD_WIDTH))?;

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // White for stopping lines
    // North
    canvas.fill_rect(Rect::new(SOUTHBOUND_LANE_X - 25, INTERSECTION_Y_START as i32 - 5, 50, 5))?;
    // South
    canvas.fill_rect(Rect::new(NORTHBOUND_LANE_X - 25, INTERSECTION_Y_END as i32, 50, 5))?;
    // East
    canvas.fill_rect(Rect::new(INTERSECTION_X_END as i32, WESTBOUND_LANE_Y - 25, 5, 50))?;
    // West
    canvas.fill_rect(Rect::new(INTERSECTION_X_START as i32 - 5, EASTBOUND_LANE_Y - 25, 5, 50))?;
    Ok(())
}

fn draw_traffic_lights(canvas: &mut Canvas<Window>, current_green_dir: &Direction) -> Result<(), String> {
    let all_red = *current_green_dir == Direction::AllRed;
    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    {
        let (x, y) = match dir {
            Direction::North => (SOUTHBOUND_LANE_X - 50, INTERSECTION_Y_START as i32 - 25),
            Direction::South => (NORTHBOUND_LANE_X + 30, INTERSECTION_Y_END as i32 + 5),
            Direction::East => (INTERSECTION_X_END as i32 + 5, WESTBOUND_LANE_Y - 50),
            Direction::West => (INTERSECTION_X_START as i32 - 25, EASTBOUND_LANE_Y + 30),
            Direction::AllRed => (0, 0), // Placeholder, will be handled by all_red color below
        };
        if all_red {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        } else {
            if dir == *current_green_dir {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
            } else {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
            }
        }
        canvas.fill_rect(Rect::new(x, y, 20, 20))?;
    }
    Ok(())
}

fn draw_vehicles(canvas: &mut Canvas<Window>, vehicles: &Vec<Vehicle>) -> Result<(), String> {
    for v in vehicles {
        let color = match v.turn {
            Turn::Left => Color::RGB(255, 255, 0), // Yellow
            Turn::Right => Color::RGB(0, 255, 255), // Cyan
            Turn::Straight => Color::RGB(255, 0, 255), // Magenta
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(v.x, v.y, 20, 20))?;
    }
    Ok(())
}
