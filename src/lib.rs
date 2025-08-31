use std::time::{Duration, Instant};
use std::thread::sleep;

/// Directions of approach to the intersection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Traffic light controller: cycles through 4 directions in order
pub struct TrafficLightController {
    pub current: Direction,
    phase_duration: Duration,
    last_switch: Instant,
}

impl TrafficLightController {
    pub fn new(phase_secs: u64) -> Self {
        Self {
            current: Direction::North,
            phase_duration: Duration::from_secs(phase_secs),
            last_switch: Instant::now(),
        }
    }

    /// Update current green direction if enough time has passed
    pub fn update(&mut self) {
        if self.last_switch.elapsed() >= self.phase_duration {
            self.current = match self.current {
                Direction::North => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::West,
                Direction::West => Direction::North,
            };
            self.last_switch = Instant::now();
            println!("Green direction now: {:?}", self.current);
        }
    }
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: u32,
    pub dir: Direction,
}

pub struct World {
    pub vehicles: Vec<Vehicle>,
    pub controller: TrafficLightController,
    next_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
            controller: TrafficLightController::new(3),
            next_id: 0,
        }
    }

    pub fn update(&mut self) {
        self.controller.update();
        // TODO: move vehicles depending on current green direction
    }

    pub fn spawn_vehicle(&mut self, dir: Direction) {
        self.vehicles.push(Vehicle {
            id: self.next_id,
            dir,
        });
        self.next_id += 1;
    }
}
