use crate::{
    Direction, EASTBOUND_LANE_Y, INTERSECTION_X_END, INTERSECTION_X_START, INTERSECTION_Y_END,
    INTERSECTION_Y_START, NORTHBOUND_LANE_X, SOUTHBOUND_LANE_X, Turn, VEHICLE_SIZE,
    WESTBOUND_LANE_Y, WINDOW_HEIGHT, WINDOW_WIDTH,
};

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

pub fn generate_path(dir: Direction, turn: Turn) -> Vec<(i32, i32)> {
    let mut path = Vec::new();

    match dir {
        Direction::North => {
            // from North, going South
            let x = SOUTHBOUND_LANE_X - 10;
            path.push((x, -20));
            path.push((x, INTERSECTION_Y_START as i32 - VEHICLE_SIZE as i32 - 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, INTERSECTION_Y_START as i32 + 6));
                    path.push((x, WINDOW_HEIGHT as i32 + VEHICLE_SIZE as i32));
                }
                Turn::Left => {
                    // Turn left to go East
                    path.push((x, EASTBOUND_LANE_Y - 10));
                    path.push((WINDOW_WIDTH as i32 + VEHICLE_SIZE as i32, EASTBOUND_LANE_Y - 10));
                }
                Turn::Right => {
                    // Turn right to go West
                    path.push((x, WESTBOUND_LANE_Y - 10));
                    path.push((-20, WESTBOUND_LANE_Y - 10));
                }
            }
        }
        Direction::South => {
            // from South, going North
            let x = NORTHBOUND_LANE_X - 10;
            path.push((x, WINDOW_HEIGHT as i32 + VEHICLE_SIZE as i32));
            path.push((x, INTERSECTION_Y_END as i32 + 5)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((x, INTERSECTION_Y_END as i32 - 6));
                    path.push((x, -(VEHICLE_SIZE as i32)));
                }
                Turn::Left => {
                    // Turn left to go West
                    path.push((x, WESTBOUND_LANE_Y - 10));
                    path.push((-(VEHICLE_SIZE as i32), WESTBOUND_LANE_Y - 10));
                }
                Turn::Right => {
                    // Turn right to go East
                    path.push((x, EASTBOUND_LANE_Y - 10));
                    path.push((WINDOW_WIDTH as i32 + VEHICLE_SIZE as i32, EASTBOUND_LANE_Y - 10));
                }
            }
        }
        Direction::East => {
            // from East, going West
            let y = WESTBOUND_LANE_Y - 10;
            path.push((WINDOW_WIDTH as i32 + VEHICLE_SIZE as i32, y));
            path.push((INTERSECTION_X_END as i32 + 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((INTERSECTION_X_END as i32 - 6, y));
                    path.push((-(VEHICLE_SIZE as i32), y));
                }
                Turn::Left => {
                    // Turn left to go South
                    path.push((SOUTHBOUND_LANE_X - 10, y));
                    path.push((
                        SOUTHBOUND_LANE_X - 10,
                        WINDOW_HEIGHT as i32 + VEHICLE_SIZE as i32,
                    ));
                }
                Turn::Right => {
                    // Turn right to go North
                    path.push((NORTHBOUND_LANE_X - 10, y));
                    path.push((NORTHBOUND_LANE_X - 10, -(VEHICLE_SIZE as i32)));
                }
            }
        }
        Direction::West => {
            // from West, going East
            let y = EASTBOUND_LANE_Y - 10;
            path.push((-(VEHICLE_SIZE as i32), y));
            path.push((INTERSECTION_X_START as i32 - VEHICLE_SIZE as i32 - 5, y)); // stopping point
            match turn {
                Turn::Straight => {
                    path.push((INTERSECTION_X_START as i32 + 1, y));
                    path.push((WINDOW_WIDTH as i32 + VEHICLE_SIZE as i32, y));
                }
                Turn::Left => {
                    // Turn left to go North
                    path.push((NORTHBOUND_LANE_X - 10, y));
                    path.push((NORTHBOUND_LANE_X - 10, -(VEHICLE_SIZE as i32)));
                }
                Turn::Right => {
                    // Turn right to go South
                    path.push((SOUTHBOUND_LANE_X - 10, y));
                    path.push(
                        (SOUTHBOUND_LANE_X - 10,
                        WINDOW_HEIGHT as i32 + (VEHICLE_SIZE as i32)),
                    );
                }
            }
        }
        Direction::AllRed => todo!(),
    }
    path
}