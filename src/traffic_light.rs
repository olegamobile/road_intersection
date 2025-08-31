use std::time::{Duration, Instant};
use crate::Direction;

const MAX_PHASE_DURATION: Duration = Duration::from_secs(30);

/// Traffic light controller: cycles through 4 directions in order
pub struct TrafficLightController {
    pub current: Direction,
    phase_duration: Duration,
    last_switch: Instant,
    base_phase_duration: Duration,
    max_phase_duration: Duration, // Added for fair distribution
    last_car_cleared_time: Option<Instant>,
    last_green_direction: Direction, // To remember the last green phase before AllRed
}

impl TrafficLightController {
    pub fn new(phase_secs: u64) -> Self {
        Self {
            current: Direction::North,
            phase_duration: Duration::from_secs(phase_secs),
            last_switch: Instant::now(),
            base_phase_duration: Duration::from_secs(phase_secs),
            max_phase_duration: MAX_PHASE_DURATION, // Initialize maximum phase duration
            last_car_cleared_time: None,
            last_green_direction: Direction::West, // Initialize to West so North is the first green
        }
    }

    /// Update current green direction if enough time has passed
    pub fn update(&mut self, waiting_vehicles: u32, cars_in_intersection: bool, vehicles_on_stop_line: bool, is_congested: bool) {
        let mut should_switch = false;

        // Check for immediate switch if no cars are waiting
        if waiting_vehicles == 0 {
            if self.last_car_cleared_time.is_none() {
                self.last_car_cleared_time = Some(Instant::now());
            } else if self.last_car_cleared_time.unwrap().elapsed() >= Duration::from_millis(500) {
                should_switch = true;
            }
        } else {
            self.last_car_cleared_time = None;
        }

        // Force switch if max_phase_duration is reached, for fairness
        if should_switch || self.last_switch.elapsed() >= self.phase_duration || self.last_switch.elapsed() >= self.max_phase_duration {
            self.last_switch = Instant::now();
            self.last_car_cleared_time = None; // Reset timer after switch

            if self.current == Direction::AllRed {
                // After AllRed, transition to the next phase in sequence
                self.current = match self.last_green_direction {
                    Direction::North => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::West,
                    Direction::West => Direction::North,
                    _ => Direction::North, // Fallback, should not happen
                };
            } else {
                if cars_in_intersection || vehicles_on_stop_line {
                    self.last_green_direction = self.current; // Store current green direction
                    self.current = Direction::AllRed;
                    self.phase_duration = Duration::from_secs(2);
                    // No return here, allow the cycle to continue after AllRed
                } else {
                    self.current = match self.current {
                        Direction::North => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::West,
                        Direction::West => Direction::North,
                        _ => Direction::North, // Should not happen if logic is correct
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
