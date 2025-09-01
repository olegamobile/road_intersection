use std::time::{Duration, Instant};
use crate::Direction;

const MAX_PHASE_DURATION: Duration = Duration::from_secs(3); // Maximum duration for each green light phase
const NO_CARS_DELAY: Duration = Duration::from_millis(200); // Time to wait for cars before switching the light

// Traffic light controller: cycles through 4 directions in order
pub struct TrafficLightController {
    pub current: Direction,
    last_switch: Instant,
    max_phase_duration: Duration,
    last_car_cleared_time: Option<Instant>,
    last_green_direction: Direction,
}

impl TrafficLightController {
    pub fn new() -> Self {
        Self {
            current: Direction::North,
            last_switch: Instant::now(),
            max_phase_duration: MAX_PHASE_DURATION, // Initialize maximum phase duration
            last_car_cleared_time: None,
            last_green_direction: Direction::West, // Initialize to West so North is the first green
        }
    }

    fn next_green_direction(&self) -> Direction {
        match self.last_green_direction {
            Direction::North => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::West,
            Direction::West => Direction::North,
            _ => Direction::North, // Fallback, should not happen
        }
    }

    // Update current green direction if enough time has passed
    pub fn update(&mut self, waiting_vehicles: u32, cars_in_intersection: bool, vehicles_on_stop_line: bool, _is_congested: bool) {
        // Rule 1: If there are no cars waiting to cross the intersection in the desired direction in NO_CARS_DELAY value switch to the next phase
        let no_cars_waiting_for_current_green = waiting_vehicles == 0;
        if no_cars_waiting_for_current_green && self.last_car_cleared_time.is_none() {
            self.last_car_cleared_time = Some(Instant::now());
        } else if !no_cars_waiting_for_current_green {
            self.last_car_cleared_time = None;
        }

        let time_since_last_car_cleared = self.last_car_cleared_time.map_or(Duration::MAX, |t| t.elapsed());
        let should_switch_due_to_no_cars = no_cars_waiting_for_current_green && time_since_last_car_cleared >= NO_CARS_DELAY;

        // Rule 2: Use max time for phase const
        let max_phase_duration_reached = self.last_switch.elapsed() >= self.max_phase_duration;

        let should_switch = should_switch_due_to_no_cars || max_phase_duration_reached;

        if self.current == Direction::AllRed {
            if !cars_in_intersection {
                self.last_green_direction = self.next_green_direction(); // Update last_green_direction before setting current
                self.current = self.last_green_direction;
                self.last_switch = Instant::now();
                self.last_car_cleared_time = None;
            }
        } else if should_switch {
            if cars_in_intersection || vehicles_on_stop_line {
                // Rule 3: If its time to switch to the next phase but there are cars on the intersection switch to AllRed.
                self.last_green_direction = self.current; // Store current green direction
                self.current = Direction::AllRed;
                self.last_switch = Instant::now();
                self.last_car_cleared_time = None;
            } else {
                self.last_green_direction = self.current; // Store current green direction
                self.current = self.next_green_direction();
                self.last_switch = Instant::now();
                self.last_car_cleared_time = None;
            }
        }
    }
}
