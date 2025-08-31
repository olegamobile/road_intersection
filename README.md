# Road Intersection Simulation

This project implements a road intersection simulation as part of the 01-Edu curriculum. It simulates vehicle movement and traffic light control at a four-way intersection.

## Task
The original task description can be found here: [https://github.com/01-edu/public/tree/master/subjects/road_intersection](https://github.com/01-edu/public/tree/master/subjects/road_intersection)

## Features

*   **Vehicle Spawning:** Vehicles can be manually spawned from North, South, East, or West approaches using keyboard controls (Up, Down, Left, Right arrow keys). A random spawn option is also available (R key).
*   **Vehicle Movement:** Vehicles follow predefined paths based on their chosen turn (Left, Right, Straight).
*   **Traffic Light Control:** An intelligent traffic light system manages the flow of vehicles through the intersection.
*   **Collision Avoidance:** Vehicles attempt to avoid collisions with other vehicles.
*   **Intersection Clearing:** The traffic light controller ensures the intersection is clear before changing to a new green light phase.
*   **Visual Simulation:** The simulation is rendered using SDL2, showing roads, lanes, traffic lights, and vehicles.

## Controller Logic (Traffic Light)

The `TrafficLightController` (located in `src/traffic_light.rs`) is responsible for managing the state of the traffic lights. It cycles through four directions (North, South, East, West) and also has an "AllRed" state to clear the intersection.

The `update()` method of the `TrafficLightController` implements the following logic:

1.  **Max Phase Duration:** Each green light phase has a maximum duration of 5 seconds. If this time elapses, the light will attempt to switch to the next phase.
2.  **No Cars Waiting:** If there are no vehicles waiting at the current green light's approach for 500 milliseconds, the light will switch to the next phase. This helps optimize flow when there's no traffic for a particular direction.
3.  **Intersection Clearing:** If it's time to switch to a new phase (either due to max duration or no cars waiting), but there are vehicles currently *within the intersection* or *on any stop line*, the traffic light will first enter an "AllRed" state. It will remain "AllRed" until the intersection is completely clear of vehicles. Once clear, it will then proceed to the next scheduled green light direction.
4.  **Normal Cycle:** In the absence of the above conditions, the traffic lights cycle through the directions in a fixed order: North -> South -> East -> West -> North.

This logic aims to balance efficient traffic flow with safety by ensuring the intersection is clear before allowing new traffic to enter.

## How to Run

1.  **Prerequisites:**
    *   Rust programming language and Cargo (Rust's package manager).
    *   SDL2 development libraries. On Windows, you might need to download the SDL2 development libraries and place them in a location where Rust can find them (e.g., in your system's `PATH` or in the project directory).

2.  **Build and Run:**
    Navigate to the project root directory in your terminal and run:
    ```bash
    cargo run
    ```

## Authors
- Oleg Balandin
- Inka Säävuori
