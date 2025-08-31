use rand::Rng;
use road_intersection::{Direction, Turn, World, WINDOW_WIDTH, WINDOW_HEIGHT, ROAD_WIDTH, ROAD_X, ROAD_Y, INTERSECTION_X_START, INTERSECTION_Y_START, INTERSECTION_X_END, INTERSECTION_Y_END, SOUTHBOUND_LANE_X, NORTHBOUND_LANE_X, WESTBOUND_LANE_Y, EASTBOUND_LANE_Y};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

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

    let mut event_pump = sdl.event_pump()?;
    let mut world = World::new();

    // Load font for overlay
    let font = ttf_context.load_font("assets/fonts/DejaVuSans.ttf", 12)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Up) => world.spawn_vehicle(Direction::South),
                    Some(Keycode::Down) => world.spawn_vehicle(Direction::North),
                    Some(Keycode::Left) => world.spawn_vehicle(Direction::East),
                    Some(Keycode::Right) => world.spawn_vehicle(Direction::West),
                    Some(Keycode::R) => {
                        let mut rng = rand::thread_rng();
                        let random_dir = match rng.gen_range(0..4) {
                            0 => Direction::North,
                            1 => Direction::South,
                            2 => Direction::East,
                            _ => Direction::West,
                        };
                        world.spawn_vehicle(random_dir);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Update simulation
        world.update();

        // Clear background
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();

        // Draw roads
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(ROAD_X as i32, 0, ROAD_WIDTH, WINDOW_HEIGHT))?;
        canvas.fill_rect(Rect::new(0, ROAD_Y as i32, WINDOW_WIDTH, ROAD_WIDTH))?;

        // Draw lane dividers
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

        // Draw intersection borders and stopping lines
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

        // Draw traffic lights
        let green_dir = world.controller.current;
        let all_red = world.controller.all_red_phase;
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        {
            let (x, y) = match dir {
                Direction::North => (SOUTHBOUND_LANE_X - 25, INTERSECTION_Y_START as i32 - 25),
                Direction::South => (NORTHBOUND_LANE_X + 5, INTERSECTION_Y_END as i32 + 5),
                Direction::East => (INTERSECTION_X_END as i32 + 5, WESTBOUND_LANE_Y - 25),
                Direction::West => (INTERSECTION_X_START as i32 - 25, EASTBOUND_LANE_Y + 5),
            };
            if all_red {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
            } else {
                if dir == green_dir {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                }
            }
            canvas.fill_rect(Rect::new(x, y, 20, 20))?;
        }

        // Draw vehicles
        for v in &world.vehicles {
            let color = match v.turn {
                Turn::Left => Color::RGB(255, 255, 0), // Yellow
                Turn::Right => Color::RGB(0, 255, 255), // Cyan
                Turn::Straight => Color::RGB(255, 0, 255), // Magenta
            };
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(v.x, v.y, 20, 20))?;
        }

        // Overlay: show variables
        let textures_creator = canvas.texture_creator();
        let overlay_text = format!(
            "Current green: {:?}, Vehicles: {}",
            green_dir,
            world.vehicles.len()
        );
        let surface = font
            .render(&overlay_text)
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;
        let texture = textures_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Some(Rect::new(10, 10, surface.width(), surface.height())))?;

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
