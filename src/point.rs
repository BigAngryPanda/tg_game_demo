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

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }
}