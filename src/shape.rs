use crate::point::Point;

#[allow(unused_imports)]
use crate::log;

#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub vertices: Vec<Point>,
    pub indices: Vec<u32>,
    pub texture_id: usize
}

impl Shape {
    pub fn square(texture_id: usize) -> Shape {
        Shape {
            vertices: vec![Point::new(-1.0, -1.0), Point::new(-1.0, 1.0), Point::new(1.0, 1.0), Point::new(1.0, -1.0)],
            indices: vec![0, 1, 2, 2, 3, 0],
            texture_id
        }
    }

    pub fn update_vertices(&mut self, vertices: &[f32]) {
        for i in 0..vertices.len()/2 {
            let vert_idx = self.indices[i] as usize;
            self.vertices[vert_idx] = Point::new(vertices[2*i], vertices[2*i + 1]);
        }
    }

    // https://ics.uci.edu/~eppstein/161/960307.html
    // https://dl.acm.org/doi/pdf/10.1145/368637.368653
    pub fn contains(&self, point: Point) -> bool {
        let (x, y) = (point.x(), point.y());

        let mut hit = true;

        for i in 0..self.vertices.len() {
            let j = (i + 1) % self.vertices.len();

            let p_i = self.vertices[i];
            let p_j = self.vertices[j];

            if ((y < p_i.y()) == (y > p_j.y())) && // fancy way to check if point.y() is in range of y[i] and y[i+1]
                (x - p_i.x() - (y - p_i.y())*(p_j.x() - p_i.x())/(p_j.y() - p_i.y()) < 0.0)  {
                    hit = !hit;
            }
        }

        !hit
    }
}
