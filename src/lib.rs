pub mod vehicle;
pub mod traffic_light;

use rand::Rng;
use vehicle::{Vehicle, generate_path};
use traffic_light::TrafficLightController;

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
pub const VEHICLE_SAFETY_GAP: u32 = 10;

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

        self.controller.update(waiting_vehicles, cars_in_intersection, self.is_congested(self.controller.current));

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
                if v.path_index < v.path.len() -1 { // Only check for collisions before and at the intersection
                    let next_pos = v.path[v.path_index + 1];
                    let next_x = v.x + (next_pos.0 - v.x).signum() * 5;
                    let next_y = v.y + (next_pos.1 - v.y).signum() * 5;

                    for other in &vehicles_clone {
                        if v.id == other.id { continue; }

                        // Bounding box collision detection with safety gap
                        if next_x < other.x + (VEHICLE_SIZE + VEHICLE_SAFETY_GAP) as i32 &&
                           next_x + (VEHICLE_SIZE + VEHICLE_SAFETY_GAP) as i32 > other.x &&
                           next_y < other.y + (VEHICLE_SIZE + VEHICLE_SAFETY_GAP) as i32 &&
                           next_y + (VEHICLE_SIZE + VEHICLE_SAFETY_GAP) as i32 > other.y {
                            can_move = false;
                            break;
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
        if self.is_congested(dir) {
            return;
        }

        let mut rng = rand::thread_rng();
        let turn = match rng.gen_range(0..3) {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => Turn::Straight,
        };
        let path = generate_path(dir, turn);
        let (x, y) = (path[0].0, path[0].1);

        if let Some(last_vehicle) = self.vehicles.iter().filter(|v| v.dir == dir).last() {
            let dist_sq = (x - last_vehicle.x).pow(2) + (y - last_vehicle.y).pow(2);
            if dist_sq < ((VEHICLE_SIZE + VEHICLE_SAFETY_GAP) * (VEHICLE_SIZE + VEHICLE_SAFETY_GAP)) as i32 {
                return;
            }
        }

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

    pub fn is_congested(&self, dir: Direction) -> bool {
        let (lane_length, num_vehicles) = match dir {
            Direction::North | Direction::South => (
                ROAD_Y,
                self.vehicles.iter().filter(|v| v.dir == dir).count() as u32,
            ),
            Direction::East | Direction::West => (
                ROAD_X,
                self.vehicles.iter().filter(|v| v.dir == dir).count() as u32,
            ),
        };
        let capacity = lane_length / (VEHICLE_SIZE + VEHICLE_SAFETY_GAP);
        num_vehicles >= capacity
    }
}