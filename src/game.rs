use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;
use crate::shape::*;
use crate::point::*;
use crate::ui::*;
use crate::game_state::*;
use crate::scene::*;

#[derive(Debug)]
pub struct Game {
    window: web_sys::Window,
    input_queue: Vec<Point>,
    ui: Ui,
    state: GameState,
    performance: web_sys::Performance,
    timestamp: f64,
    scene: Scene
}

impl Game {
    pub fn new() -> Game {
        let window: web_sys::Window = web_sys::window().expect("Failed to get window");

        let document = window.document().expect("Failed to get Document");
        let canvas = document.get_element_by_id("canvas_gl").expect("Failed to get canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast canvas");

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let mut scene = Scene::new(canvas);

        let canvas_ui = document.get_element_by_id("canvas_ui").expect("Failed to get canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast canvas");

        canvas_ui.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas_ui.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let mut ui = Ui::new(&canvas_ui);
        ui.set_score(0);
        ui.set_time(0.0);

        scene.add_dynamic_shape(&Shape::square(1));
        scene.add_dynamic_shape(&Shape::square(3));
        scene.add_dynamic_shape(&Shape::square(3));

        scene.add_static_shape(&Shape::square(2));

        scene.update_renders();

        let performance: web_sys::Performance = window
            .performance()
            .expect("performance should be available");

        Game {
            window,
            input_queue: Vec::new(),
            ui,
            state: GameState::new(),
            performance,
            timestamp: 0.0,
            scene
        }
    }

    pub fn store_input(&mut self, input: Point) {
        self.input_queue.push(input);
    }

    pub fn run(&mut self) {
        let dt = self.update_time();
        self.handle_input();
        self.scene.render(dt);
        self.update_ui();

        if self.scene.state() == State::Done && self.state.tick_timer(dt) {
            self.update_scene();
        }
    }

    pub fn window(&self) -> web_sys::Window {
        self.window.clone()
    }

    pub fn request_next_frame(&self, f: &Closure<dyn FnMut()>) -> Result<i32, JsValue> {
        self.window.request_animation_frame(f.as_ref().unchecked_ref())
    }

    fn handle_input(&mut self) {
        if let Some(input) = self.input_queue.pop() {
            if self.scene.state() == State::Done && self.scene.is_dynamic_hit(0, input) {
                self.on_hit();
            }
        }
    }

    fn on_hit(&mut self) {
        self.update_scene();

        self.state.add_score();

        self.ui.set_score(self.state.score());
    }

    fn update_time(&mut self) -> f64 {
        let dt = (self.performance.now() - self.timestamp) / 1000.0;

        self.timestamp = self.performance.now();

        dt
    }

    fn update_scene(&mut self) {
        self.scene.permutate_transforms();
        self.scene.reset();
        self.state.start_timer();
    }

    fn update_ui(&mut self) {
        match self.scene.state() {
            State::Initial => {
                self.ui.clear_timer();
            },
            State::Done => {
                self.ui.set_time(self.state.time());
            }
        }
    }
}