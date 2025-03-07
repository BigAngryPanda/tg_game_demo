use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;

#[derive(Debug)]
pub struct Ui {
    //score_area: web_sys::HtmlParagraphElement,
    ctx: web_sys::CanvasRenderingContext2d,
    score_area: Label,
    time_area: Label,
}

impl Ui {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Ui {
        let ctx: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .expect("Failed to get context")
            .expect("Failed to get js object")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Failed to get CanvasRenderingContext2d");

        ctx.set_fill_style_str("white");
        ctx.set_text_align("center");

        let w = canvas.width();
        let h = canvas.height();

        let score_label = Label {
            x: (-0.95 + 1.0)*(w as f64 / 2.0), // map [-1; 1] to [0; w]
            y: (0.95 - 1.0)*(h as f64 / -2.0),
            w: 0.05*w as f64,
            h: 0.01*h as f64
        };

        let time_label = Label {
            x: (-0.0 + 1.0)*(w as f64 / 2.0), // map [-1; 1] to [0; w]
            y: (0.9 - 1.0)*(h as f64 / -2.0),
            w: 0.5*w as f64,
            h: 0.01*h as f64
        };

        ctx.clear_rect(0.0, 0.0, w as f64, h as f64);

        Ui {
            ctx,
            score_area: score_label,
            time_area: time_label
        }
    }

    pub fn set_score(&mut self, score: u64) {
        self.score_area.clear(&self.ctx);
        self.score_area.draw(&self.ctx, &format!("Score = {}", score));
    }

    pub fn set_time(&mut self, t: f64) {
        self.clear_timer();
        self.time_area.draw(&self.ctx, &format!("{:.2}", t));
    }

    pub fn clear_timer(&mut self) {
        self.time_area.clear(&self.ctx);
    }
}

#[derive(Debug)]
struct Label {
    // top left corner in webgl coords
    x: f64,
    y: f64,
    w: f64,
    h: f64
}

impl Label {
    fn draw(&mut self, ctx: &web_sys::CanvasRenderingContext2d, text: &str) {
        ctx.fill_text(text, self.x, self.y).unwrap();
    }

    fn clear(&mut self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.clear_rect(self.x - self.w / 2.0, self.y - self.h, self.w, self.h);
    }
}
