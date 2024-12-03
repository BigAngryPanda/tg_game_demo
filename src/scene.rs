use crate::shape::Shape;
use crate::render::*;

#[allow(unused_imports)]
use crate::log;

#[derive(Debug, Clone, Copy)]
pub struct VerticesDescriptor {
    pub offset: usize,
    pub count: usize,
}

pub struct Scene {
    // shapes
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertices_descriptors: Vec<VerticesDescriptor>,

    // feedback shapes
    pub feedback_vertices: Vec<f32>,
    pub feedback_descriptors: Vec<VerticesDescriptor>,

    // transforms
    pub dynamic_transforms: Vec<TransformInfo>
}

impl Scene {
    pub fn new() -> Scene {
        Scene::default()
    }

    pub fn add_shape(&mut self, shape: &Shape) {
        let offset: u32 = self.vertices.len() as u32;

        for index in &shape.indices {
            self.indices.push(offset + index);
        }

        self.vertices_descriptors.push(VerticesDescriptor { offset: self.indices.len() - shape.indices.len(), count: shape.indices.len() });

        for vertex in &shape.vertices {
            self.vertices.push(vertex.x());
            self.vertices.push(vertex.y());
        }
    }

    pub fn add_feedback_shape(&mut self, shape: &Shape) {
        let offset: usize = self.feedback_vertices.len();

        for &i in shape.indices.iter() {
            self.feedback_vertices.push(shape.vertices[i as usize].x());
            self.feedback_vertices.push(shape.vertices[i as usize].y());

            self.feedback_descriptors.push(VerticesDescriptor { offset , count: shape.indices.len() });
        }

        for vertex in &shape.vertices {
            self.feedback_vertices.push(vertex.x());
            self.feedback_vertices.push(vertex.y());
        }
    }

    pub fn add_transform(&mut self, transform: TransformInfo) {
        self.dynamic_transforms.push(transform);
    }

    pub fn write_vertices(&self, render: &IndicesRender) {
        render.write_vertices(&self.vertices, "vertexPosition");
    }

    pub fn write_feedback_vertices(&self, render: &FeedbackRender) {
        //render.write_vertices(&self.feedback_vertices, "vertexPosition");
    }

    pub fn write_indices(&self, render: &IndicesRender) {
        render.write_indices(&self.indices);
    }

    pub fn draw(&self, render: &IndicesRender, transform_indices: &[usize]) {
        render.clear();

        let mut shape_idx: usize = 0;

        for &i in transform_indices {
            let transform: &TransformInfo = &self.dynamic_transforms[i];

            render.write_uniform(&transform.translation_matrix(), "translation");
            render.write_uniform(&transform.scale_matrix(), "scale");

            let desc: VerticesDescriptor = self.vertices_descriptors[shape_idx];

            render.draw(desc.count as i32, (std::mem::size_of::<u32>() * desc.offset) as i32);

            shape_idx += 1;
        }
    }

    pub fn draw_feedback(&self, fr: &FeedbackRender, transform_indices: &[usize]) {
        //fr.begin_draw();

        let mut shape_idx: usize = 0;

        for &i in transform_indices {
            let transform: &TransformInfo = &self.dynamic_transforms[i];

            fr.write_uniform(&transform.translation_matrix(), "translation");
            fr.write_uniform(&transform.scale_matrix(), "scale");

            let desc: VerticesDescriptor = self.feedback_descriptors[shape_idx];

            log::write_debug(&desc);

            //fr.draw(desc.count as i32, desc.offset as i32);

            shape_idx += 1;
        }

        //fr.end_draw();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertices_descriptors: Vec::new(),
            feedback_vertices: Vec::new(),
            feedback_descriptors: Vec::new(),
            dynamic_transforms: Vec::new()
        }
    }
}