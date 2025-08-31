use rand::Rng;
use std::time::{Duration, Instant};

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
    phase_duration: Duration,
    last_switch: Instant,
    base_phase_duration: Duration,
}

impl TrafficLightController {
    pub fn new(phase_secs: u64) -> Self {
        Self {
            current: Direction::North,
            phase_duration: Duration::from_secs(phase_secs),
            last_switch: Instant::now(),
            base_phase_duration: Duration::from_secs(phase_secs),
        }
    }

    /// Update current green direction if enough time has passed
    pub fn update(&mut self, waiting_vehicles: u32) {
        if self.last_switch.elapsed() >= self.phase_duration {
            self.current = match self.current {
                Direction::North => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::West,
                Direction::West => Direction::North,
            };
            self.last_switch = Instant::now();

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
            println!(
                "Green direction now: {:?}, duration: {:?}",
                self.current,
                self.phase_duration
            );
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
    let intersection_y = (250, 350);
    let intersection_x = (350, 450);

    match dir {
        Direction::North => { // from North, going South
            let x = 375;
            path.push((x, -20));
            path.push((x, intersection_y.0 - 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, 620));
                }
                Turn::Left => { // Turn left to go East
                    path.push((x, 325));
                    path.push((820, 325));
                }
                Turn::Right => { // Turn right to go West
                    path.push((x, 275));
                    path.push((-20, 275));
                }
            }
        }
        Direction::South => { // from South, going North
            let x = 425;
            path.push((x, 600));
            path.push((x, intersection_y.1 + 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, -20));
                }
                Turn::Left => { // Turn left to go West
                    path.push((x, 275));
                    path.push((-20, 275));
                }
                Turn::Right => { // Turn right to go East
                    path.push((x, 325));
                    path.push((820, 325));
                }
            }
        }
        Direction::East => { // from East, going West
            let y = 275;
            path.push((800, y));
            path.push((intersection_x.1 + 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((-20, y));
                }
                Turn::Left => { // Turn left to go South
                    path.push((375, y));
                    path.push((375, 620));
                }
                Turn::Right => { // Turn right to go North
                    path.push((425, y));
                    path.push((425, -20));
                }
            }
        }
        Direction::West => { // from West, going East
            let y = 325;
            path.push((-20, y));
            path.push((intersection_x.0 - 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((820, y));
                }
                Turn::Left => { // Turn left to go North
                    path.push((375, y));
                    path.push((375, 620));
                }
                Turn::Right => { // Turn right to go South
                    path.push((425, y));
                    path.push((425, -20));
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

        self.controller.update(waiting_vehicles);

        let vehicles_clone = self.vehicles.clone();
        for v in &mut self.vehicles {
            if v.passed {
                continue;
            }
            let green_dir = self.controller.current;
            let is_green = v.dir == green_dir;

            let at_intersection_border = v.path_index == 1;

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
                    if dist_sq < 400 { // 20*20
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


            if can_move && (is_green || !at_intersection_border) {
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
            .retain(|v| v.x > -40 && v.x < 840 && v.y > -40 && v.y < 640);
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