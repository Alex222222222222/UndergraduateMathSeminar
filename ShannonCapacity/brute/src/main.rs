use num::{bigint::BigUint, FromPrimitive, ToPrimitive};

fn main() {
    println!("Hello, world!");

    let res = iter(Stage::new_with_order_dimension(7, 2), 0);

    println!("res: {}", res);
}

/// stage
#[derive(Debug, Clone)]
pub struct Stage {
    pub order: u64,
    pub dimension: u64,
    pub points: u64,
    /// represent in bits, 0 means present, 1 means not present
    pub left_points: BigUint,
}

impl Stage {
    pub fn new_with_order_dimension(order: u64, dimension: u64) -> Stage {
        let left_points = BigUint::from_u8(0).unwrap();

        Stage {
            order,
            dimension,
            points: 0,
            left_points,
        }
    }
}

pub fn unavailable_points(point: u64, dimension: u64, order: u64, left_points: BigUint) -> BigUint {
    let point_vec = point_int_2_point_vec(point, dimension, order);
    let mut left_points = left_points;

    for i in 0..(3_u64).pow(dimension as u32) {
        let mut point_vec_new = point_vec.clone();
        let change = point_int_2_point_vec(i, dimension, 3);
        for j in 0..dimension {
            let change = change[j as usize];
            if change == 0 {
                continue;
            } else if change == 1 {
                point_vec_new[j as usize] = (point_vec_new[j as usize] + order - 1) % order;
            } else {
                point_vec_new[j as usize] = (point_vec_new[j as usize] + 1) % order;
            }
        }
        let point_new = point_vec_to_int(point_vec_new, order as usize);
        left_points.set_bit(point_new.to_u64().unwrap(), true);
    }

    left_points
}

pub fn point_int_2_point_vec(point: u64, dimension: u64, order: u64) -> Vec<u64> {
    let mut point_vec = Vec::new();

    let mut point_new = point;
    for _ in 0..dimension {
        let point_1 = point_new / order;
        let point = point_new - point_1 * order;
        point_new = point_1;
        point_vec.push(point.to_u64().unwrap());
    }

    point_vec
}

pub fn point_vec_to_int(point_vec: Vec<u64>, order: usize) -> BigUint {
    let mut point = BigUint::from_u8(0).unwrap();
    for i in 0..point_vec.len() {
        point *= order;
        point += BigUint::from_u64(point_vec[point_vec.len() - 1 - i]).unwrap();
    }

    point
}

pub fn iter(stage: Stage, next_point: u64) -> u64 {
    let left_points =
        unavailable_points(next_point, stage.dimension, stage.order, stage.left_points);
    let points = stage.points + 1;
    let mut max_points = points;
    let stage = Stage {
        order: stage.order,
        dimension: stage.dimension,
        points,
        left_points,
    };

    for i in 0..stage.order.pow(stage.dimension as u32) {
        if !stage.left_points.bit(i) {
            let res = iter(stage.clone(), i);
            if res > max_points {
                max_points = res;
            }
        }
    }

    // println!("max_points: {}", max_points);
    max_points
}
