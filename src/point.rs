#[allow(unused_imports)]
use crate::log;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    data: [f32; 2]
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            data: [x, y]
        }
    }

    pub fn from_screen_coords(x: f32, y: f32) -> Point {
        Point {
            data: [2.0*x - 1.0, 1.0 - 2.0*y]
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }
}