use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;

#[derive(Debug)]
pub struct Ui {
    score_area: web_sys::HtmlParagraphElement,
}

impl Ui {
    pub fn new(document: &web_sys::Document) -> Ui {
        let score_area = document.get_element_by_id("score_area").expect("Failed to get canvas")
            .dyn_into::<web_sys::HtmlParagraphElement>().expect("Failed to cast canvas");

        Ui {
            score_area
        }
    }

    pub fn set_score<T: std::fmt::Display>(&self, value: &T) {
        self.score_area.set_inner_text(&format!("Score = {}", value));
    }
}