use std::ops;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point{x: a.x + b.x, y: a.y + b.y}});
impl_op_ex!(* |a: &Point, b: &Point| -> Point { Point{x: a.x * b.x, y: a.y * b.y}});
