# How SDL2 is Used in the `road_intersection` Project

This document explains how the [SDL2](https://www.libsdl.org/) library is used within this project to create the window, handle user input, and render the simulation. All the rendering and event-handling code is located in `src/main.rs`.

## 1. Setup and Dependencies (`Cargo.toml`)

The project includes the `sdl2` crate as a dependency. The `ttf` feature is also enabled, which is crucial for rendering text.

```toml
[dependencies]
sdl2 = { version = "0.38", features = ["ttf"] }
# ... other dependencies
```

- `sdl2`: This is the main crate that provides Rust bindings for the SDL2 library.
- `features = ["ttf"]`: This feature flag enables the `sdl2::ttf` module, which is used for loading fonts and rendering text.

## 2. Initialization (`src/main.rs`)

The `main` function begins by initializing SDL2 and its necessary subsystems.

```rust
fn main() -> Result<(), String> {
    // Initialize the core SDL2 library
    let sdl = sdl2::init()?;

    // Initialize the video subsystem to create a window
    let video = sdl.video()?;

    // Initialize the TTF subsystem for font rendering
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // ... rest of the main function
}
```

- **`sdl2::init()?`**: This is the first step. It initializes SDL2 and returns an `Sdl` context, which is the gateway to all other SDL2 functionality.
- **`sdl.video()?`**: From the `Sdl` context, we get the `VideoSubsystem`. This is required to create windows and manage the display.
- **`sdl2::ttf::init()`**: This initializes the SDL2_ttf library, which allows the program to work with TrueType Fonts (`.ttf` files).

## 3. Creating the Window and Canvas

Next, a window is created and a "canvas" is set up for drawing.

```rust
    let window = video
        .window("Road Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync() // Enable VSync
        .build()
        .map_err(|e| e.to_string())?;
```

- **`.into_canvas()`**: This converts the `Window` into a `Canvas`, the primary tool for all 2D drawing.
- **`.present_vsync()`**: This is an important performance setting that synchronizes the frame rate with the monitor's refresh rate, preventing screen tearing.

## 4. Optimization: Pre-rendering the Static Background

To improve performance, the application avoids redrawing the entire scene every frame. Instead, all the static elements (roads, lanes, dividers) are pre-rendered onto a single texture *once* before the main loop begins. This is much more efficient than drawing hundreds of individual shapes every frame.

```rust
    let texture_creator = canvas.texture_creator();

    // Create a texture that will act as a canvas for the static background
    let mut static_background = texture_creator
        .create_texture_target(None, WINDOW_WIDTH, WINDOW_HEIGHT)
        .map_err(|e| e.to_string())?;

    // Draw the static elements to the new texture
    canvas.with_texture_canvas(&mut static_background, |texture_canvas| {
        texture_canvas.set_draw_color(Color::RGB(200, 200, 200));
        texture_canvas.clear();
        draw_roads(texture_canvas).unwrap();
        draw_lanes(texture_canvas).unwrap();
        draw_lane_dividers(texture_canvas).unwrap();
        draw_intersection_elements(texture_canvas).unwrap();
    }).map_err(|e| e.to_string())?;
```

- **`create_texture_target`**: This creates a special texture that can be drawn onto.
- **`with_texture_canvas`**: This helper method temporarily switches the rendering target to our `static_background` texture. All drawing operations inside this block are applied to the texture instead of the main window.

## 5. The Main Loop and Event Handling

The core of the application is the `'running: loop`. In each iteration, it handles user input, updates the simulation state, and redraws the screen.

```rust
    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => { /* ... */ }
                _ => {}
            }
        }
        // ... update and drawing ...
    }
```

- **`event_pump.poll_iter()`**: This method iterates through all user input events (keyboard, mouse, window closing, etc.) that have occurred since the last check.

## 6. The Optimized Rendering Pipeline

Inside the main loop, the rendering process is now much faster. Instead of redrawing the background elements, it just copies the pre-made `static_background` texture and then draws the dynamic elements over it.

```rust
        // Update simulation
        world.update();

        // 1. Copy the pre-rendered background to the canvas
        canvas.copy(&static_background, None, None)?;

        // 2. Draw dynamic elements on top
        draw_traffic_lights(&mut canvas, &world.controller.current)?;
        draw_vehicles(&mut canvas, &world.vehicles)?;

        // 3. Draw UI overlay
        render_text_overlay(/* ... */)?;

        // 4. Present the final image to the screen
        canvas.present();
```

1.  **`canvas.copy(...)`**: This is a very fast operation that copies the entire `static_background` texture to the screen, effectively clearing the previous frame and drawing the new background in one step.
2.  **`draw_traffic_lights` / `draw_vehicles`**: These functions are still called every frame because they represent objects that change (colors) or move.
3.  **`render_text_overlay`**: The UI is drawn last, on top of everything else.
4.  **`canvas.present()`**: This updates the window with the final composed frame.

## 7. Rendering Text with `sdl2_ttf`

This part remains the same. The project uses the `sdl2_ttf` library to display text by rendering it to a `Surface`, converting that to a hardware-accelerated `Texture`, and then copying it to the canvas.

### Font Loading

The font is loaded once at the start of `main`.

```rust
let font = ttf_context.load_font("assets/fonts/DejaVuSans.ttf", 12)?;
```

### Text Rendering Process

The `render_text_overlay` function handles the conversion from text to a texture that can be drawn on the canvas.

```rust
fn render_text_overlay(
    canvas: &mut Canvas<Window>,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    text: &str,
    x: i32,
    y: i32,
) -> Result<(), String> {
    // 1. Render text to a Surface
    let surface = font.render(text).blended(Color::RGB(0, 0, 0))?;

    // 2. Convert Surface to a Texture
    let texture = texture_creator.create_texture_from_surface(&surface)?;

    // 3. Copy the Texture to the canvas
    canvas.copy(&texture, None, Some(Rect::new(x, y, surface.width(), surface.height())))?;
    Ok(())
}
```
