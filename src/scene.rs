use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;
use crate::shape::*;
use crate::render::*;
use crate::shader::*;
use crate::texture::*;
use crate::rand::*;
use crate::point::*;

fn get_transforms(u: f32, d: f32, l: f32, r: f32, count_x: u32, count_y: u32) -> Vec<TransformInfo> {
    let dx = (r - l) / (2*count_x) as f32;
    let dy = (u - d) / (2*count_y) as f32;

    let mut result: Vec<TransformInfo> = Vec::new();

    for i in 0..count_x {
        for j in 0..count_y {
            result.push(TransformInfo(l + dx + (i as f32)*2.0*dx, d + dy + (j as f32)*2.0*dy));
        }
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Initial,
    Done,
}

#[derive(Debug)]
pub struct Scene {
    feedback_render: FeedbackRender,
    indices_render: IndicesRender,
    dynamic_shapes: Vec<Shape>,
    static_shapes: Vec<Shape>,
    transforms: Vec<TransformInfo>,
    transform_indices: Vec<usize>,
    state: State,
    time: f64,
    textures: Vec<Texture>
}

impl Scene {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Scene {
        let gl: web_sys::WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .expect("Failed to get context")
            .expect("Failed to get js object")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Failed to get WebGl2RenderingContext");

        gl.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGl2RenderingContext::LEQUAL);

        gl.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT | web_sys::WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        gl.clear_color(1.0, 1.0, 1.0, 1.0);

        let default_raw_texture = [
            255, 0, 0, 255,
            0, 255, 0, 255,
            0, 0, 255, 255,
            255, 0, 255, 255
        ];

        let default_texture = from_rgba_data(&gl, &default_raw_texture, 2, 2, web_sys::WebGl2RenderingContext::RGBA);

        let fig_raw_texture = include_bytes!("textures/princess.png");
        let fig_texture = from_png_data(&gl, fig_raw_texture, web_sys::WebGl2RenderingContext::RGBA);

        let back_raw_texture = include_bytes!("textures/brick.png");
        let back_texture = from_png_data(&gl, back_raw_texture, web_sys::WebGl2RenderingContext::RGBA);

        let other_fig_raw_texture = include_bytes!("textures/horn_girl.png");
        let other_fig_texture = from_png_data(&gl, other_fig_raw_texture, web_sys::WebGl2RenderingContext::RGBA);

        // feedback render
        let feedback_render = FeedbackRender::new(&gl);

        feedback_render.link_shader(feedback::VERTEX_SHADER, VERTEX_SHADER_KIND);
        feedback_render.link_shader(feedback::FRAGMENT_SHADER, FRAGMENT_SHADER_KIND);

        feedback_render.link_program();

        feedback_render.setup_render();

        feedback_render.enable_texture("tex");

        feedback_render.write_uniform(&TransformInfo(0.25, 0.25).scale_matrix(), "scale");

        let transforms = get_transforms(0.8, -1.0, -1.0, 1.0, 1, 3);

        let transform_indices: Vec<usize> = std::ops::Range{start: 0, end: transforms.len()}.into_iter().collect();

        // indices render
        let indices_render = IndicesRender::new(&gl);

        indices_render.link_shader(background::VERTEX_SHADER, VERTEX_SHADER_KIND);
        indices_render.link_shader(background::FRAGMENT_SHADER, FRAGMENT_SHADER_KIND);

        indices_render.link_program();

        indices_render.setup_render();

        indices_render.enable_texture("tex");
        indices_render.write_uniform(&TransformInfo::id(), "translation");
        indices_render.write_uniform(&TransformInfo::id(), "scale");

        Scene {
            feedback_render,
            indices_render,
            dynamic_shapes: Vec::new(),
            static_shapes: Vec::new(),
            transforms,
            transform_indices,
            state: State::Initial,
            time: 0.0,
            textures: vec![default_texture, fig_texture, back_texture, other_fig_texture]
        }
    }

    pub fn permutate_transforms(&mut self) {
        for i in 0..self.dynamic_shapes.len() {
            let j = rand_in_range(i as u64, self.transform_indices.len() as u64) as usize;

            self.transform_indices.swap(i, j);
        }
    }

    pub fn render(&mut self, dt: f64) {
        // render static shapes
        self.indices_render.setup_render();

        for i in 0..self.static_shapes.len() {
            let texture_id = self.static_shapes[i].texture_id;

            self.indices_render.draw(i, &self.textures[texture_id]);
        }

        // render dynamic shapes
        self.feedback_render.setup_render();
        self.feedback_render.write_float(self.time as f32, "t");

        for i in 0..self.dynamic_shapes.len() {
            let transform_idx = self.transform_indices[i];
            let texture_id = self.dynamic_shapes[i].texture_id;

            self.feedback_render.write_uniform(&self.transforms[transform_idx].translation_matrix(), "translation");

            self.feedback_render.draw(i, &self.textures[texture_id]);

            self.dynamic_shapes[i].update_vertices(self.feedback_render.read_vertices(i));
        }

        self.time += dt;

        if self.time >= 1.0 {
            self.state = State::Done;
        }

        self.time = self.time.clamp(0.0, 1.0);
    }

    pub fn update_renders(&mut self) {
        self.feedback_render.setup_render();
        self.feedback_render.write_vertices("vertexPosition");

        self.indices_render.setup_render();
        self.indices_render.write_vertices("vertexPosition");
    }

    pub fn add_dynamic_shape(&mut self, shape: &Shape) {
        self.dynamic_shapes.push(shape.clone());
        self.feedback_render.add(shape);
    }

    pub fn add_static_shape(&mut self, shape: &Shape) {
        self.static_shapes.push(shape.clone());
        self.indices_render.add(shape);
    }

    pub fn is_dynamic_hit(&self, idx: usize, point: Point) -> bool {
        self.dynamic_shapes[idx].contains(point)
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.state = State::Initial;
    }
}
