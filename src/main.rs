use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use road_intersection::{World, Direction};

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video.window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().present_vsync().build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;
    let mut world = World::new();

    // Load font for overlay
    let font = ttf_context.load_font("assets/fonts/DejaVuSans.ttf", 16)?;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => world.spawn_vehicle(Direction::South),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => world.spawn_vehicle(Direction::North),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => world.spawn_vehicle(Direction::East),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => world.spawn_vehicle(Direction::West),
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
        let road_width = 100;
        canvas.fill_rect(Rect::new(350, 0, road_width, 600))?;
        canvas.fill_rect(Rect::new(0, 250, 800, road_width))?;

        // Draw traffic lights
        let green_dir = world.controller.current;
        for dir in [Direction::North, Direction::South, Direction::East, Direction::West] {
            let (x, y) = match dir {
                Direction::North => (400, 240),
                Direction::South => (400, 360),
                Direction::East => (460, 300),
                Direction::West => (340, 300),
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
            let (x, y) = match v.dir {
                Direction::North => (380, 100),
                Direction::South => (420, 500),
                Direction::East => (700, 280),
                Direction::West => (100, 320),
            };
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.fill_rect(Rect::new(x, y, 20, 20))?;
        }

        // Overlay: show variables
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let textures_creator = canvas.texture_creator();
        let overlay_text = format!("Current green: {:?}, Vehicles: {}", green_dir, world.vehicles.len());
        let surface = font.render(&overlay_text).blended(Color::RGB(0, 0, 0)).map_err(|e| e.to_string())?;
        let texture = textures_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Some(Rect::new(10, 10, surface.width(), surface.height())))?;

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
