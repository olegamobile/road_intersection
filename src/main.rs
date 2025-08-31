use rand::Rng;
use road_intersection::{Direction, Turn, World};
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
        .window("Road Intersection", 800, 600)
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

        // Draw grid
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        for i in 0..16 {
            canvas.draw_line((i * 50, 0), (i * 50, 600))?;
        }
        for i in 0..12 {
            canvas.draw_line((0, i * 50), (800, i * 50))?;
        }

        // Draw roads
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        let road_width = 100;
        canvas.fill_rect(Rect::new(350, 0, road_width, 600))?;
        canvas.fill_rect(Rect::new(0, 250, 800, road_width))?;

        // Draw lane dividers
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..15 {
            // Horizontal road
            if i * 60 + 30 < 350 || i * 60 > 450 {
                canvas.fill_rect(Rect::new(i * 60, 298, 30, 4))?;
            }
            // Vertical road
            if i * 40 + 20 < 250 || i * 40 > 350 {
                canvas.fill_rect(Rect::new(398, i * 40, 4, 20))?;
            }
        }

        // Draw intersection borders and stopping lines
        canvas.set_draw_color(Color::RGB(200, 200, 200)); // Light gray for intersection outline
        canvas.draw_rect(Rect::new(350, 250, 100, 100))?;

        canvas.set_draw_color(Color::RGB(255, 255, 255)); // White for stopping lines
        // North
        canvas.fill_rect(Rect::new(350, 245, 50, 5))?;
        // South
        canvas.fill_rect(Rect::new(400, 350, 50, 5))?;
        // East
        canvas.fill_rect(Rect::new(450, 250, 5, 50))?;
        // West
        canvas.fill_rect(Rect::new(345, 300, 5, 50))?;

        // Draw traffic lights
        let green_dir = world.controller.current;
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        {
            let (x, y) = match dir {
                Direction::North => (325, 225),
                Direction::South => (455, 355),
                Direction::East => (455, 225),
                Direction::West => (325, 355),
            };
                if dir == green_dir {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
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

        // Draw grid labels and turning points
        let textures_creator = canvas.texture_creator();
        for i in 1..8 {
            let x = i * 100;
            let label_text = format!("{}", x);
            let surface = font.render(&label_text).blended(Color::RGB(0, 0, 0)).map_err(|e| e.to_string())?;
            let texture = textures_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
            canvas.copy(&texture, None, Some(Rect::new(x, 0, surface.width(), surface.height())))?;
        }
        for i in 1..6 {
            let y = i * 100;
            let label_text = format!("{}", y);
            let surface = font.render(&label_text).blended(Color::RGB(0, 0, 0)).map_err(|e| e.to_string())?;
            let texture = textures_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
            canvas.copy(&texture, None, Some(Rect::new(0, y, surface.width(), surface.height())))?;
        }

        for v in &world.vehicles {
            if v.turn != Turn::Straight {
                if v.path.len() > 2 {
                    let turning_point = v.path[2];
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.fill_rect(Rect::new(turning_point.0 - 2, turning_point.1 - 2, 4, 4))?;
                    let label_text = format!("({}, {})", turning_point.0, turning_point.1);
                    let surface = font.render(&label_text).blended(Color::RGB(255, 0, 0)).map_err(|e| e.to_string())?;
                    let texture = textures_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
                    canvas.copy(&texture, None, Some(Rect::new(turning_point.0 + 5, turning_point.1, surface.width(), surface.height())))?;
                }
            }
        }

        // Overlay: show variables
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