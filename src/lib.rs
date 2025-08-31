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
    let intersection_x = (350, 450);
    let intersection_y = (250, 350);

    match dir {
        Direction::North => { // from North, going South
            let x = 375;
            path.push((x, -20));
            path.push((x, intersection_y.0 - 25));
            match turn {
                Turn::Straight => {
                    path.push((x, intersection_y.1));
                    path.push((x, 620));
                }
                Turn::Left => { // Turn left to go East
                    path.push((intersection_x.1, intersection_y.0));
                    path.push((intersection_x.1, 325));
                    path.push((820, 325));
                }
                Turn::Right => { // Turn right to go West
                    path.push((intersection_x.0, intersection_y.0));
                    path.push((intersection_x.0, 275));
                    path.push((-20, 275));
                }
            }
        }
        Direction::South => { // from South, going North
            let x = 425;
            path.push((x, 600));
            path.push((x, intersection_y.1 + 25));
            match turn {
                Turn::Straight => {
                    path.push((x, intersection_y.0));
                    path.push((x, -20));
                }
                Turn::Left => { // Turn left to go West
                    path.push((intersection_x.0, intersection_y.1));
                    path.push((intersection_x.0, 275));
                    path.push((-20, 275));
                }
                Turn::Right => { // Turn right to go East
                    path.push((intersection_x.1, intersection_y.1));
                    path.push((intersection_x.1, 325));
                    path.push((820, 325));
                }
            }
        }
        Direction::East => { // from East, going West
            let y = 275;
            path.push((800, y));
            path.push((intersection_x.1 + 25, y));
            match turn {
                Turn::Straight => {
                    path.push((intersection_x.0, y));
                    path.push((-20, y));
                }
                Turn::Left => { // Turn left to go North
                    path.push((intersection_x.1, intersection_y.0));
                    path.push((375, intersection_y.0));
                    path.push((375, -20));
                }
                Turn::Right => { // Turn right to go South
                    path.push((intersection_x.1, intersection_y.1));
                    path.push((425, intersection_y.1));
                    path.push((425, 620));
                }
            }
        }
        Direction::West => { // from West, going East
            let y = 325;
            path.push((-20, y));
            path.push((intersection_x.0 - 25, y));
            match turn {
                Turn::Straight => {
                    path.push((intersection_x.1, y));
                    path.push((820, y));
                }
                Turn::Left => { // Turn left to go South
                    path.push((intersection_x.0, intersection_y.1));
                    path.push((425, intersection_y.1));
                    path.push((425, 620));
                }
                Turn::Right => { // Turn right to go North
                    path.push((intersection_x.0, intersection_y.0));
                    path.push((375, intersection_y.0));
                    path.push((375, -20));
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
        let intersection_x = (350, 450);
        let intersection_y = (250, 350);
        for v in &self.vehicles {
            if v.dir == self.controller.current {
                let at_intersection_border = match v.dir {
                    Direction::North => v.y <= intersection_y.0 && v.y >= intersection_y.0 - 5,
                    Direction::South => v.y >= intersection_y.1 && v.y <= intersection_y.1 + 5,
                    Direction::East => v.x >= intersection_x.1 && v.x <= intersection_x.1 + 5,
                    Direction::West => v.x <= intersection_x.0 && v.x >= intersection_x.0 - 5,
                };
                if at_intersection_border {
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

            let at_intersection_border = match v.dir {
                Direction::North => v.y >= intersection_y.0 - 25 && v.y <= intersection_y.0,
                Direction::South => v.y <= intersection_y.1 + 25 && v.y >= intersection_y.1,
                Direction::East => v.x <= intersection_x.1 + 25 && v.x >= intersection_x.1,
                Direction::West => v.x >= intersection_x.0 - 25 && v.x <= intersection_x.0,
            };

            let mut can_move = true;
            for other in &vehicles_clone {
                if v.id == other.id {
                    continue;
                }

                let dist_s = (v.x as f32 - other.x as f32).powi(2) + (v.y as f32 - other.y as f32).powi(2);
                if dist_s < 25.0 * 25.0 {
                    if v.path_index < v.path.len() - 1 && other.path_index < other.path.len() - 1 {
                        let target = v.path[v.path_index + 1];
                        let other_target = other.path[other.path_index + 1];
                        if target == other_target {
                            let my_dist_to_target = ((v.x - target.0).pow(2) + (v.y - target.1).pow(2)) as f32;
                            let other_dist_to_target = ((other.x - target.0).pow(2) + (other.y - target.1).pow(2)) as f32;
                            if my_dist_to_target > other_dist_to_target {
                                can_move = false;
                                break;
                            }
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
        let (x, y) = match dir {
            Direction::North => (375, -20),
            Direction::South => (425, 600),
            Direction::East => (800, 275),
            Direction::West => (-20, 325),
        };
        let mut rng = rand::thread_rng();
        let turn = match rng.gen_range(0..3) {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => Turn::Straight,
        };
        let path = generate_path(dir, turn);
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
