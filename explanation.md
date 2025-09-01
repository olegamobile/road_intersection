# Road Intersection - Project Explanation

This document provides a detailed explanation of the Road Intersection simulation project. It is intended for beginners in Rust, but it assumes some basic knowledge of programming concepts.

## Project Architecture

The project is a simple traffic simulation at a road intersection. It is built using the Rust programming language and the `sdl2` library for graphics and event handling.

The project is structured into several files:

-   `src/main.rs`: This is the main entry point of the application. It contains the main loop, event handling, and rendering logic.
-   `src/lib.rs`: This file contains the core logic of the simulation. It defines the `World` struct, which holds the state of the simulation, and the `Direction` and `Turn` enums.
-   `src/vehicle.rs`: This file defines the `Vehicle` struct and the logic for generating vehicle paths.
-   `src/traffic_light.rs`: This file defines the `TrafficLightController` struct, which manages the state of the traffic lights.

### The `World` Struct

The `World` struct is the main container for the simulation state. It is defined in `src/lib.rs`:

```rust
pub struct World {
    pub vehicles: Vec<Vehicle>,
    pub controller: TrafficLightController,
    next_id: u32,
}
```

-   `vehicles`: A vector of `Vehicle` structs, representing all the vehicles in the simulation.
-   `controller`: A `TrafficLightController` struct, which manages the state of the traffic lights.
-   `next_id`: A counter for assigning unique IDs to new vehicles.


The `World` struct has an `update` method that is called in each frame of the main loop. This method updates the state of the simulation, including the traffic lights and the vehicle positions.

### The Main Loop

The main loop is in the `main` function in `src/main.rs`. It is a standard game loop that handles events, updates the world state, and renders the scene.

```rust
'running: loop {
    for event in event_pump.poll_iter() {
        // Handle events
    }

    // Update simulation
    world.update();

    // Render the scene
    canvas.clear();
    draw_roads(&mut canvas)?;
    // ...
    canvas.present();
}
```

## Vehicle Movement Logic

The vehicle movement logic is implemented in `src/lib.rs` and `src/vehicle.rs`.

### The `Vehicle` Struct

The `Vehicle` struct is defined in `src/vehicle.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub dir: Direction,
    pub turn: Turn,
    pub x: i32,
    pub y: i32,
    pub passed: bool,
    pub path: Vec<(i32, i32)>,
    pub path_index: usize,
}
```

-   `id`: A unique ID for the vehicle.
-   `dir`: The direction from which the vehicle is approaching the intersection.
-   `turn`: The turn the vehicle intends to make at the intersection.
-   `x`, `y`: The current coordinates of the vehicle.
-   `passed`: A boolean that is true if the vehicle has passed the intersection.
-   `path`: A vector of coordinates that the vehicle will follow.
-   `path_index`: The index of the current target coordinate in the `path` vector.

### Vehicle Spawning

Vehicles are spawned in the `spawn_vehicle` method of the `World` struct. This method is called when the user presses one of the arrow keys or when random generation is on.

When a vehicle is spawned, a path is generated for it using the `generate_path` function from `src/vehicle.rs`. The path is a series of waypoints that the vehicle will follow to cross the intersection.

### Vehicle Movement

The `update_vehicle_positions` method in `src/lib.rs` is responsible for moving the vehicles. In each frame, this method updates the position of each vehicle based on its path and speed.

The vehicle moves towards the next waypoint in its path. When it reaches a waypoint, the `path_index` is incremented to target the next waypoint.

```rust
if !should_stop {
    if v.path_index < v.path.len() - 1 {
        let target = v.path[v.path_index + 1];
        let dx = target.0 - v.x;
        let dy = target.1 - v.y;
        let dist = ((dx * dx + dy * dy) as f32).sqrt();
        if dist < 5.0 {
            v.path_index += 1;
        } else {
            v.x += (dx as f32 / dist * 5.0) as i32;
            v.y += (dy as f32 / dist * 5.0) as i32;
        }
    } else {
        v.passed = true;
    }
}
```

A vehicle will stop if it is at a red light or if there is another vehicle in front of it. The collision detection is a simple bounding box check.

## Traffic Light Controller

The traffic light controller is implemented in `src/traffic_light.rs`.

### The `TrafficLightController` Struct

The `TrafficLightController` struct is defined in `src/traffic_light.rs`:

```rust
pub struct TrafficLightController {
    pub current: Direction,
    last_switch: Instant,
    max_phase_duration: Duration,
    last_car_cleared_time: Option<Instant>,
    last_green_direction: Direction,
}
```

-   `current`: The current direction that has a green light.
-   `last_switch`: The time when the light last changed.
-   `max_phase_duration`: The maximum duration for a green light phase.
-   `last_car_cleared_time`: The time when the last car cleared the intersection.
-   `last_green_direction`: The direction that had the last green light.

### Traffic Light Logic

The `update` method of the `TrafficLightController` contains the logic for switching the traffic lights. The lights are switched based on a set of rules:

1.  A green light phase has a maximum duration.
2.  If there are no cars waiting to cross in the current green direction, the light will switch to the next direction after a short delay.
3.  Before switching to the next green light, the controller will set the light to `AllRed` for a short period to allow the intersection to clear.

This logic is implemented in the `update` method:

```rust
pub fn update(&mut self, waiting_vehicles: u32, cars_in_intersection: bool, vehicles_on_stop_line: bool, _is_congested: bool) {
    // ...
    let should_switch = should_switch_due_to_no_cars || max_phase_duration_reached;

    if self.current == Direction::AllRed {
        if !cars_in_intersection {
            self.current = self.next_green_direction();
            // ...
        }
    } else if should_switch {
        if cars_in_intersection || vehicles_on_stop_line {
            self.current = Direction::AllRed;
            // ...
        } else {
            self.current = self.next_green_direction();
            // ...
        }
    }
}
```