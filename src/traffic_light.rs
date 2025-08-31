use std::time::{Duration, Instant};
use crate::Direction;

/// Traffic light controller: cycles through 4 directions in order
pub struct TrafficLightController {
    pub current: Direction,
    pub all_red_phase: bool,
    phase_duration: Duration,
    last_switch: Instant,
    base_phase_duration: Duration,
}

impl TrafficLightController {
    pub fn new(phase_secs: u64) -> Self {
        Self {
            current: Direction::North,
            all_red_phase: false,
            phase_duration: Duration::from_secs(phase_secs),
            last_switch: Instant::now(),
            base_phase_duration: Duration::from_secs(phase_secs),
        }
    }

    /// Update current green direction if enough time has passed
    pub fn update(&mut self, waiting_vehicles: u32, cars_in_intersection: bool, is_congested: bool) {
        if self.last_switch.elapsed() >= self.phase_duration {
            self.last_switch = Instant::now();

            if self.all_red_phase {
                self.all_red_phase = false;
                self.current = match self.current {
                    Direction::North => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::West,
                    Direction::West => Direction::North,
                };
            } else {
                if cars_in_intersection {
                    self.all_red_phase = true;
                    self.phase_duration = Duration::from_secs(2);
                    return; // Return early to avoid changing direction
                } else {
                    self.current = match self.current {
                        Direction::North => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::West,
                        Direction::West => Direction::North,
                    };
                }
            }

            if is_congested {
                self.phase_duration = self.base_phase_duration + Duration::from_secs(5);
            } else if waiting_vehicles > 5 {
                self.phase_duration = self.base_phase_duration + Duration::from_secs(2);
            } else if waiting_vehicles == 0 {
                self.phase_duration = self.base_phase_duration.saturating_sub(Duration::from_secs(1));
                if self.phase_duration < Duration::from_secs(1) {
                    self.phase_duration = Duration::from_secs(1);
                }
            } else {
                self.phase_duration = self.base_phase_duration;
            }
        }
    }
}