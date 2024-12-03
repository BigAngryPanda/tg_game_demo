use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;
use crate::shape::*;
use crate::render::*;
use crate::shader::*;
use crate::point::*;
use crate::rand::*;
use crate::ui::*;
use crate::game_state::*;
use crate::texture::*;

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

#[derive(Debug)]
pub struct Game {
    window: web_sys::Window,
    feedback_render: FeedbackRender,
    indices_render: IndicesRender,
    dynamic_shapes: Vec<Shape>,
    static_shapes: Vec<Shape>,
    transforms: Vec<TransformInfo>,
    transform_indices: Vec<usize>,
    input_queue: Vec<Point>,
    random_generator: RandomGenerator,
    ui: Ui,
    state: GameState,
    textures: Vec<Texture>
}

impl Game {
    pub fn new() -> Game {
        let window: web_sys::Window = web_sys::window().expect("Failed to get window");

        let document = window.document().expect("Failed to get Document");
        let canvas = document.get_element_by_id("canvas").expect("Failed to get canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast canvas");

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let ui = Ui::new(&document);

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

        let fig_raw_texture = include_bytes!("textures/brick.png");
        let fig_texture = from_png_data(&gl, fig_raw_texture, web_sys::WebGl2RenderingContext::RGBA);

        let back_raw_texture = include_bytes!("textures/background.png");
        let back_texture = from_png_data(&gl, back_raw_texture, web_sys::WebGl2RenderingContext::RGBA);

        // feedback render
        let mut feedback_render = FeedbackRender::new(&gl);

        feedback_render.link_shader(feedback::VERTEX_SHADER, VERTEX_SHADER_KIND);
        feedback_render.link_shader(feedback::FRAGMENT_SHADER, FRAGMENT_SHADER_KIND);

        feedback_render.link_program();

        feedback_render.setup_render();

        feedback_render.enable_texture("tex");

        feedback_render.add(&Shape::triangle(1));
        feedback_render.add(&Shape::square(1));
        feedback_render.add(&Shape::square(1));

        feedback_render.write_vertices("vertexPosition");
        feedback_render.write_uniform(&TransformInfo(0.1, 0.1).scale_matrix(), "scale");

        let dynamic_shapes = vec![Shape::triangle(1), Shape::square(1), Shape::square(1)];
        let transforms = get_transforms(0.8, -1.0, -1.0, 1.0, 3, 3);

        let transform_indices: Vec<usize> = std::ops::Range{start: 0, end: transforms.len()}.into_iter().collect();

        let mut static_shapes: Vec<Shape> = Vec::new();
        static_shapes.push(Shape::square(2));

        // indices render
        let mut indices_render = IndicesRender::new(&gl);

        indices_render.link_shader(background::VERTEX_SHADER, VERTEX_SHADER_KIND);
        indices_render.link_shader(background::FRAGMENT_SHADER, FRAGMENT_SHADER_KIND);

        indices_render.link_program();

        indices_render.setup_render();

        indices_render.add(&static_shapes[0]);

        indices_render.write_vertices("vertexPosition");
        indices_render.enable_texture("tex");
        indices_render.write_uniform(&TransformInfo::id(), "translation");
        indices_render.write_uniform(&TransformInfo::id(), "scale");

        Game {
            window,
            feedback_render,
            indices_render,
            dynamic_shapes,
            static_shapes,
            transforms,
            transform_indices,
            input_queue: Vec::new(),
            random_generator: RandomGenerator::new(),
            ui,
            state: GameState::new(),
            textures: vec![default_texture, fig_texture, back_texture]
        }
    }

    pub fn store_input(&mut self, input: Point) {
        self.input_queue.push(input);
    }

    pub fn run(&mut self) {
        self.handle_input();
        self.render();
    }

    pub fn window(&self) -> web_sys::Window {
        self.window.clone()
    }

    pub fn request_next_frame(&self, f: &Closure<dyn FnMut()>) -> Result<i32, JsValue> {
        self.window.request_animation_frame(f.as_ref().unchecked_ref())
    }

    fn handle_input(&mut self) {
        if let Some(input) = self.input_queue.pop() {
            if self.dynamic_shapes[0].contains(input) {
                self.update_state();
            }
        }
    }

    fn render(&mut self) {
        // render static shapes
        self.indices_render.setup_render();

        for i in 0..self.static_shapes.len() {
            let texture_id = self.static_shapes[i].texture_id;

            self.indices_render.draw(i, &self.textures[texture_id]);
        }

        // render dynamic shapes
        self.feedback_render.setup_render();

        for i in 0..self.dynamic_shapes.len() {
            let transform_idx = self.transform_indices[i];
            let texture_id = self.dynamic_shapes[i].texture_id;

            self.feedback_render.write_uniform(&self.transforms[transform_idx].translation_matrix(), "translation");

            self.feedback_render.draw(i, &self.textures[texture_id]);

            self.dynamic_shapes[i].update_vertices(self.feedback_render.read_vertices(i));
        }
    }

    fn update_state(&mut self) {
        self.randomize_transforms(self.dynamic_shapes.len());

        self.state.add_score();

        self.ui.set_score(&self.state.score());
    }

    fn randomize_transforms(&mut self, count: usize) {
        for i in 0..count {
            let j = self.random_generator.rand_in_range(i as u64, self.transform_indices.len() as u64) as usize;

            self.transform_indices.swap(i, j);
        }
    }
}