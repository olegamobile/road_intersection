use rand::Rng;
use std::time::{Duration, Instant};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const ROAD_WIDTH: u32 = 100;

pub const ROAD_X: u32 = (WINDOW_WIDTH - ROAD_WIDTH) / 2; // 350
pub const ROAD_Y: u32 = (WINDOW_HEIGHT - ROAD_WIDTH) / 2; // 250

pub const INTERSECTION_X_START: u32 = ROAD_X;
pub const INTERSECTION_X_END: u32 = ROAD_X + ROAD_WIDTH;
pub const INTERSECTION_Y_START: u32 = ROAD_Y;
pub const INTERSECTION_Y_END: u32 = ROAD_Y + ROAD_WIDTH;

pub const NORTHBOUND_LANE_X: i32 = (ROAD_X + ROAD_WIDTH / 2 + ROAD_X + ROAD_WIDTH) as i32 / 2;
pub const SOUTHBOUND_LANE_X: i32 = (ROAD_X + ROAD_WIDTH / 2 + ROAD_X) as i32 / 2;
pub const EASTBOUND_LANE_Y: i32 = (ROAD_Y + ROAD_WIDTH / 2 + ROAD_Y + ROAD_WIDTH) as i32 / 2;
pub const WESTBOUND_LANE_Y: i32 = (ROAD_Y + ROAD_WIDTH / 2 + ROAD_Y) as i32 / 2;

pub const VEHICLE_SIZE: u32 = 20;

/// Directions of approach to the intersection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

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
    pub fn update(&mut self, waiting_vehicles: u32, cars_in_intersection: bool) {
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
                if waiting_vehicles > 5 {
                    self.phase_duration = self.base_phase_duration + Duration::from_secs(2);
                } else if waiting_vehicles == 0 {
                    self.phase_duration = self.base_phase_duration.saturating_sub(Duration::from_secs(1));
                    if self.phase_duration < Duration::from_secs(1) {
                        self.phase_duration = Duration::from_secs(1);
                    }
                } else {
                    self.phase_duration = self.base_phase_duration;
                }
            } else {
                if cars_in_intersection {
                    self.all_red_phase = true;
                    self.phase_duration = Duration::from_secs(2);
                } else {
                    self.current = match self.current {
                        Direction::North => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::West,
                        Direction::West => Direction::North,
                    };
                    if waiting_vehicles > 5 {
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
    }
}

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

fn generate_path(dir: Direction, turn: Turn) -> Vec<(i32, i32)> {
    let mut path = Vec::new();

    match dir {
        Direction::North => { // from North, going South
            let x = SOUTHBOUND_LANE_X;
            path.push((x, -20));
            path.push((x, INTERSECTION_Y_START as i32 - 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, WINDOW_HEIGHT as i32 + 20));
                }
                Turn::Left => { // Turn left to go East
                    path.push((x, EASTBOUND_LANE_Y));
                    path.push((WINDOW_WIDTH as i32 + 20, EASTBOUND_LANE_Y));
                }
                Turn::Right => { // Turn right to go West
                    path.push((x, WESTBOUND_LANE_Y));
                    path.push((-20, WESTBOUND_LANE_Y));
                }
            }
        }
        Direction::South => { // from South, going North
            let x = NORTHBOUND_LANE_X;
            path.push((x, WINDOW_HEIGHT as i32 + 20));
            path.push((x, INTERSECTION_Y_END as i32 + 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, -20));
                }
                Turn::Left => { // Turn left to go West
                    path.push((x, WESTBOUND_LANE_Y));
                    path.push((-20, WESTBOUND_LANE_Y));
                }
                Turn::Right => { // Turn right to go East
                    path.push((x, EASTBOUND_LANE_Y));
                    path.push((WINDOW_WIDTH as i32 + 20, EASTBOUND_LANE_Y));
                }
            }
        }
        Direction::East => { // from East, going West
            let y = WESTBOUND_LANE_Y;
            path.push((WINDOW_WIDTH as i32 + 20, y));
            path.push((INTERSECTION_X_END as i32 + 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((-20, y));
                }
                Turn::Left => { // Turn left to go South
                    path.push((SOUTHBOUND_LANE_X, y));
                    path.push((SOUTHBOUND_LANE_X, WINDOW_HEIGHT as i32 + 20));
                }
                Turn::Right => { // Turn right to go North
                    path.push((NORTHBOUND_LANE_X, y));
                    path.push((NORTHBOUND_LANE_X, -20));
                }
            }
        }
        Direction::West => { // from West, going East
            let y = EASTBOUND_LANE_Y;
            path.push((-20, y));
            path.push((INTERSECTION_X_START as i32 - 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((WINDOW_WIDTH as i32 + 20, y));
                }
                Turn::Left => { // Turn left to go North
                    path.push((NORTHBOUND_LANE_X, y));
                    path.push((NORTHBOUND_LANE_X, -20));
                }
                Turn::Right => { // Turn right to go South
                    path.push((SOUTHBOUND_LANE_X, y));
                    path.push((SOUTHBOUND_LANE_X, WINDOW_HEIGHT as i32 + 20));
                }
            }
        }
    }
    path
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
        let mut waiting_vehicles = 0;
        for v in &self.vehicles {
            if v.dir == self.controller.current {
                if v.path_index == 1 {
                    waiting_vehicles += 1;
                }
            }
        }

        let mut cars_in_intersection = false;
        for v in &self.vehicles {
            if v.x < INTERSECTION_X_END as i32 && v.x + VEHICLE_SIZE as i32 > INTERSECTION_X_START as i32 &&
               v.y < INTERSECTION_Y_END as i32 && v.y + VEHICLE_SIZE as i32 > INTERSECTION_Y_START as i32 {
                cars_in_intersection = true;
                break;
            }
        }

        self.controller.update(waiting_vehicles, cars_in_intersection);

        let vehicles_clone = self.vehicles.clone();
        for v in &mut self.vehicles {
            if v.passed {
                continue;
            }
            let green_dir = self.controller.current;
            let is_green = v.dir == green_dir && !self.controller.all_red_phase;

            let at_intersection_border = v.path_index == 1;

            let mut should_stop = false;
            if at_intersection_border && !is_green {
                should_stop = true;
            }

            if !should_stop {
                let mut can_move = true;
                if v.path_index < 2 { // Only check for collisions before and at the intersection
                    for other in &vehicles_clone {
                        if v.id == other.id { continue; }

                        let my_next_pos = if v.path_index + 1 < v.path.len() {
                            v.path[v.path_index + 1]
                        } else {
                            (v.x, v.y)
                        };

                        // Simple distance check
                        let dist_sq = (v.x - other.x).pow(2) + (v.y - other.y).pow(2);
                        if dist_sq < (VEHICLE_SIZE * VEHICLE_SIZE) as i32 * 2 {
                            // Check if other vehicle is in front
                            let (dx, dy) = (my_next_pos.0 - v.x, my_next_pos.1 - v.y);
                            let (odx, ody) = (other.x - v.x, other.y - v.y);
                            if dx * odx + dy * ody > 0 {
                                can_move = false;
                                break;
                            }
                        }
                    }
                }
                if !can_move {
                    should_stop = true;
                }
            }


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
        }
        self.vehicles
            .retain(|v| v.x > -40 && v.x < WINDOW_WIDTH as i32 + 40 && v.y > -40 && v.y < WINDOW_HEIGHT as i32 + 40);
    }

    pub fn spawn_vehicle(&mut self, dir: Direction) {
        let mut rng = rand::thread_rng();
        let turn = match rng.gen_range(0..3) {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => Turn::Straight,
        };
        let path = generate_path(dir, turn);
        let (x, y) = (path[0].0, path[0].1);

        self.vehicles.push(Vehicle {
            id: self.next_id,
            dir,
            turn,
            x,
            y,
            passed: false,
            path,
            path_index: 0,
        });
        self.next_id += 1;
    }
}