mod point;
mod shape;
mod scene;
mod shader;
mod log;
mod render;

use wasm_bindgen::prelude::*;

use shape::Shape;
use render::*;
use scene::*;

#[wasm_bindgen]
pub fn start_webgl() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;

    let mut feedback_render = FeedbackRender::new(&canvas);

    feedback_render.link_shader(shader::FEEDBACK_VERTEX_SHADER, shader::VERTEX_SHADER_KIND);
    feedback_render.link_shader(shader::FRAGMENT_SHADER, shader::FRAGMENT_SHADER_KIND);

    feedback_render.link_program();

    feedback_render.add(&Shape::triangle());
    feedback_render.add(&Shape::square());

    feedback_render.write_vertices("vertexPosition");

    feedback_render.write_uniform(&TransformInfo {
        scale_x: 0.25,
        scale_y: 0.25,
        translation_x: -0.5,
        translation_y: 0.0,
    }.scale_matrix(), "scale");

    feedback_render.write_uniform(&TransformInfo {
        scale_x: 0.25,
        scale_y: 0.25,
        translation_x: -0.5,
        translation_y: 0.0,
    }.translation_matrix(), "translation");

    feedback_render.clear();

    feedback_render.draw(0);

    feedback_render.read_vertices(0);

    feedback_render.write_uniform(&TransformInfo {
        scale_x: 0.25,
        scale_y: 0.25,
        translation_x: 0.5,
        translation_y: 0.0,
    }.translation_matrix(), "translation");

    feedback_render.draw(1);

    feedback_render.read_vertices(1);

    //scene.write_feedback_vertices(&feedback_render);

    //scene.draw(&render, &[0]);
    //scene.draw_feedback(&feedback_render, &[1]);

    //feedback_render.debug();

    Ok(())
}