
# Using SDL2 with Rust (`rust-sdl2`)

This guide provides a detailed explanation of how to use the SDL2 library in a Rust project, using the `rust-sdl2` crate.

## What is SDL2?

Simple DirectMedia Layer (SDL) is a cross-platform development library designed to provide low-level access to audio, keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D. It's widely used for writing video games and other multimedia applications. The `rust-sdl2` crate provides Rust bindings for the SDL2 library, allowing you to leverage the power and safety of Rust for game development.

## Project Setup

To use `rust-sdl2`, you first need to add it as a dependency in your `Cargo.toml` file.

```toml
[dependencies]
sdl2 = "0.36.0"
```

You may also need to install the SDL2 development libraries on your system.

*   **macOS (via Homebrew):** `brew install sdl2`
*   **Ubuntu/Debian:** `sudo apt-get install libsdl2-dev`
*   **Windows:** Download the development libraries from the SDL2 website and configure your project to link against them.

## Initializing SDL2 and Creating a Window

The first step in any SDL2 application is to initialize the library. The `sdl2::init()` function returns an `Sdl` context, which is the main entry point to the library's functionality.

From the `Sdl` context, you can access various subsystems, such as the video subsystem, which is needed to create a window.

Here is a basic example of initializing SDL2, creating a window, and setting up a main loop:

```rust
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    // 1. Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // 2. Create a window
    let window = video_subsystem.window("My SDL2 Window", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // 3. Create a canvas
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // 4. Set up the event pump
    let mut event_pump = sdl_context.event_pump()?;

    // 5. Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Game logic and rendering go here

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
```

### Explanation of the Code:

1.  **`sdl2::init()?`**: Initializes the SDL2 library and returns an `Sdl` context. The `?` operator is used for error handling.
2.  **`sdl_context.video()?`**: Initializes the video subsystem.
3.  **`video_subsystem.window(...)`**: Creates a new window with a title, width, and height. `.position_centered()` centers the window on the screen. `.build()` finalizes the window creation.
4.  **`window.into_canvas().build()`**: Creates a 2D rendering context (a `Canvas`) for the window. This is what you'll use to draw on the screen.
5.  **`canvas.set_draw_color(...)`, `canvas.clear()`, `canvas.present()`**: These methods are used for drawing.
    *   `set_draw_color` sets the color for subsequent drawing operations.
    *   `clear` fills the entire canvas with the current draw color.
    *   `present` updates the screen with what has been drawn to the canvas.
6.  **`sdl_context.event_pump()?`**: Creates an event pump, which is used to poll for events like keyboard input, mouse clicks, and window-closing.
7.  **The Main Loop**: The `'running: loop` is the heart of your application. It continuously runs, processing events and updating the screen.
    *   `event_pump.poll_iter()`: This iterates over any pending events.
    *   The `match` statement checks for specific events. In this case, we handle the `Quit` event (when the user clicks the window's close button) and the `Escape` key being pressed. Both of these events will break the loop and cause the program to exit.

## Drawing on the Canvas

The `Canvas` provides methods for drawing various shapes. Here's how to draw a rectangle:

```rust
// Inside the main loop

// Clear the screen with a black color
canvas.set_draw_color(Color::RGB(0, 0, 0));
canvas.clear();

// Draw a blue rectangle
let rect = sdl2::rect::Rect::new(100, 100, 200, 200); // x, y, width, height
canvas.set_draw_color(Color::RGB(0, 0, 255));
canvas.fill_rect(rect)?;

// Draw an empty red rectangle
let outline_rect = sdl2::rect::Rect::new(400, 100, 200, 200);
canvas.set_draw_color(Color::RGB(255, 0, 0));
canvas.draw_rect(outline_rect)?;

// Update the screen
canvas.present();
```

*   **`sdl2::rect::Rect::new(...)`**: Creates a new rectangle at the specified x and y coordinates, with the given width and height.
*   **`canvas.fill_rect(rect)?`**: Fills the specified rectangle with the current draw color.
*   **`canvas.draw_rect(rect)?`**: Draws the outline of the specified rectangle with the current draw color.

## Handling Keyboard Input

You can handle keyboard input by matching on the `Event::KeyDown` and `Event::KeyUp` events in your main loop.

```rust
// Inside the main loop's event handling
match event {
    Event::Quit {..} |
    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        break 'running
    },
    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
        println!("Spacebar was pressed!");
    },
    Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
        println!("Spacebar was released!");
    },
    _ => {}
}
```

This example prints a message when the spacebar is pressed and released. You can use this to control game characters, navigate menus, and more.

## Loading and Displaying Images

To load and display images, you'll need to use the `sdl2_image` crate. First, add it to your `Cargo.toml`:

```toml
[dependencies]
sdl2 = "0.36.0"
sdl2_image = { version = "0.36.0", features = ["png"] }
```

Then, you can initialize the image context and load textures:

```rust
// You'll need to add these use statements
use sdl2::image::{InitFlag, LoadTexture};
use std::path::Path;

// After creating the canvas
let texture_creator = canvas.texture_creator();

// Initialize the image context
let _image_context = sdl2::image::init(InitFlag::PNG)?;

// Load a texture from a file
let texture = texture_creator.load_texture(Path::new("assets/my_image.png"))?;

// Inside the main loop, after clearing the canvas
let dest_rect = sdl2::rect::Rect::new(300, 250, 100, 100);
canvas.copy(&texture, None, Some(dest_rect))?;

// Then call canvas.present()
```

*   **`sdl2::image::init(...)`**: Initializes the `sdl2_image` library.
*   **`canvas.texture_creator()`**: Gets a `TextureCreator` that can be used to create textures.
*   **`texture_creator.load_texture(...)`**: Loads an image file into a `Texture`.
*   **`canvas.copy(&texture, None, Some(dest_rect))?`**: Renders the texture to the canvas.
    *   The first `None` means you want to render the entire texture.
    *   `Some(dest_rect)` specifies the destination rectangle on the canvas where the texture will be drawn.

This covers the fundamental aspects of using `rust-sdl2`. With these building blocks, you can create games and other graphical applications in Rust.
