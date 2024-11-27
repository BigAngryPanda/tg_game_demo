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
    dynamic_shapes: Vec<Shape>,
    transforms: Vec<TransformInfo>,
    transform_indices: Vec<usize>,
    input_queue: Vec<Point>,
    random_generator: RandomGenerator,
    ui: Ui,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let window: web_sys::Window = web_sys::window().expect("Failed to get window");

        let document = window.document().expect("Failed to get Document");
        let canvas = document.get_element_by_id("canvas").expect("Failed to get canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast canvas");

        let ui = Ui::new(&document);

        let mut feedback_render = FeedbackRender::new(&canvas);

        feedback_render.link_shader(FEEDBACK_VERTEX_SHADER, VERTEX_SHADER_KIND);
        feedback_render.link_shader(FRAGMENT_SHADER, FRAGMENT_SHADER_KIND);

        feedback_render.link_program();

        feedback_render.add(&Shape::triangle());
        feedback_render.add(&Shape::square());
        feedback_render.add(&Shape::square());

        feedback_render.write_vertices("vertexPosition");

        feedback_render.write_uniform(&TransformInfo(0.1, 0.1).scale_matrix(), "scale");

        feedback_render.clear();

        let dynamic_shapes = vec![Shape::triangle(), Shape::square(), Shape::square()];
        let transforms = get_transforms(0.8, -1.0, -1.0, 1.0, 3, 3);

        let transform_indices: Vec<usize> = std::ops::Range{start: 0, end: transforms.len()}.into_iter().collect();

        Game {
            window,
            feedback_render,
            dynamic_shapes,
            transforms,
            transform_indices,
            input_queue: Vec::new(),
            random_generator: RandomGenerator::new(),
            ui,
            state: GameState::new()
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
        // render dynamic shapes
        for i in 0..self.dynamic_shapes.len() {
            let transform_idx = self.transform_indices[i];

            self.feedback_render.write_uniform(&self.transforms[transform_idx].translation_matrix(), "translation");

            self.feedback_render.draw(i);

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