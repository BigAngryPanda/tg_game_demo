mod point;
mod shape;
mod shader;
mod log;
mod render;
mod game;
mod rand;
mod ui;
mod game_state;
mod texture;

use wasm_bindgen::prelude::*;

use point::*;
use game::*;

fn set_input_callback(game: std::rc::Rc<std::cell::RefCell<Game>>) {
    let window = game.as_ref().borrow().window();

    let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let wnd = game.as_ref().borrow().window();

        let width: f32 = wnd.inner_width().expect("Failed to get window width").as_f64().unwrap() as f32;
        let height: f32 = wnd.inner_height().expect("Failed to get window height").as_f64().unwrap() as f32;

        let e = event.dyn_into::<web_sys::MouseEvent>().expect("Failed to get mouse event");

        game.as_ref().borrow_mut().store_input(Point::from_screen_coords(e.x() as f32 / width, e.y() as f32 / height));
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).expect("Failed to set event listener");

    callback.forget();
}

fn run_loop(game: std::rc::Rc<std::cell::RefCell<Game>>) {
    let draw_closure = std::rc::Rc::new(std::cell::RefCell::new(None));
    let draw_closure_clone = draw_closure.clone();

    let game_clone = game.clone();

    *draw_closure_clone.borrow_mut() = Some(Closure::new(move || {
        game.as_ref().borrow_mut().run();

        // Schedule ourself for another requestAnimationFrame callback.
        game.as_ref().borrow().request_next_frame(draw_closure.borrow().as_ref().unwrap())
            .expect("Failed to request new frame");
    }));

    game_clone.as_ref().borrow().request_next_frame(draw_closure_clone.borrow().as_ref().unwrap())
        .expect("Failed to request new frame");
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let game: std::rc::Rc<std::cell::RefCell<Game>> = std::rc::Rc::new(std::cell::RefCell::new(Game::new()));

    set_input_callback(game.clone());
    run_loop(game.clone());

    Ok(())
}

pub fn text_demo() -> Result<(), JsValue> {
    use shader::*;

    use std::io::Cursor;
    use image::ImageReader;

    let vertex_shader: &str = r#"#version 300 es

    precision mediump float;

    in vec2 vertex_in;
    in vec2 uv_in;

    out vec2 uv_out;

    void main() {
        gl_Position = vec4(vertex_in, 0, 1);
        uv_out = uv_in;
    }"#;

    let fragment_shader: &str = r#"#version 300 es

    precision mediump float;

    in vec2 uv_out;

    uniform sampler2D tex;

    out vec4 frag_color;

    void main() {
        frag_color = texture(tex, uv_out);
    }"#;

    let raw_data = include_bytes!("textures/background.png");

    let reader = ImageReader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .expect("Cursor io never fails");

    let image = reader.decode().unwrap();

    /*
    let mut buff: Vec<u8> = vec![0; 1920000];

    let dec = reader.into_decoder().unwrap();

    let (w, h) = dec.dimensions();

    match dec.read_image(&mut buff) {
        Ok(()) => {
            log::write(&"OK");
        },
        Err(err) => {
            log::write(&err);
        }
    }
    */

    //assert_eq!(reader.format(), Some(ImageFormat::Png));

    let window: web_sys::Window = web_sys::window().expect("Failed to get window");

    let document = window.document().expect("Failed to get Document");
    let canvas = document.get_element_by_id("canvas").expect("Failed to get canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast canvas");

    log::write(&canvas.width());
    log::write(&canvas.height());

    log::write(&window.inner_width().unwrap().as_f64().unwrap());
    log::write(&window.inner_height().unwrap().as_f64().unwrap());

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    let gl: web_sys::WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .expect("Failed to get context")
        .expect("Failed to get js object")
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .expect("Failed to get WebGl2RenderingContext");

    let program: web_sys::WebGlProgram = gl.create_program().expect("Failed to create program");

    let v_shader = gl.create_shader(VERTEX_SHADER_KIND).expect("Failed to create fragment shader");
    gl.shader_source(&v_shader, &vertex_shader);
    gl.compile_shader(&v_shader);
    gl.attach_shader(&program, &v_shader);

    let f_shader = gl.create_shader(FRAGMENT_SHADER_KIND).expect("Failed to create fragment shader");
    gl.shader_source(&f_shader, &fragment_shader);
    gl.compile_shader(&f_shader);
    gl.attach_shader(&program, &f_shader);

    gl.link_program(&program);

    gl.use_program(Some(&program));

    let tex = gl.create_texture();

    gl.active_texture(web_sys::WebGl2RenderingContext::TEXTURE0);
    gl.bind_texture(web_sys::WebGl2RenderingContext::TEXTURE_2D, tex.as_ref());

    let location: web_sys::WebGlUniformLocation =
        gl.get_uniform_location(&program, "tex").expect("Failed to get uniform location");

    gl.uniform1i(Some(&location), 0);

    let level = 0;
    let internal_format = web_sys::WebGl2RenderingContext::RGBA;
    let width = image.width() as i32;
    let height = image.height() as i32;
    let border = 0;
    let src_format = web_sys::WebGl2RenderingContext::RGBA;
    let src_type = web_sys::WebGl2RenderingContext::UNSIGNED_BYTE;
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        web_sys::WebGl2RenderingContext::TEXTURE_2D,
        level,
        internal_format as i32,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(image.as_bytes()),
    )?;

    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_MIN_FILTER, web_sys::WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_MAG_FILTER, web_sys::WebGl2RenderingContext::NEAREST as i32);

    let vertices: [f32; 8] = [
        -1.0, -1.0,
         1.0, -1.0,
        -1.0,  1.0,
         1.0,  1.0
    ];

    let indices: [u32; 6] = [
        0, 1, 2,
        2, 3, 1
    ];

    let uv: [f32; 8] = [
        0.0, 0.0,
        1.0, 0.0,
        0.0, 1.0,
        1.0, 1.0
    ];

    // vertices
    gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, gl.create_buffer().as_ref());
    gl.bind_buffer(web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, gl.create_buffer().as_ref());

    unsafe {
        let vert_array = web_sys::js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            web_sys::WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    unsafe {
        let idx_array = web_sys::js_sys::Uint32Array::view(&indices);

        gl.buffer_data_with_array_buffer_view(
            web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &idx_array,
            web_sys::WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vertex_location: u32 = gl.get_attrib_location(&program, "vertex_in") as u32;

    gl.enable_vertex_attrib_array(vertex_location);

    gl.vertex_attrib_pointer_with_i32(vertex_location, 2, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);

    // uv
    gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, gl.create_buffer().as_ref());

    unsafe {
        let uv_array = web_sys::js_sys::Float32Array::view(&uv);

        gl.buffer_data_with_array_buffer_view(
            web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
            &uv_array,
            web_sys::WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let uv_location: u32 = gl.get_attrib_location(&program, "uv_in") as u32;

    gl.enable_vertex_attrib_array(uv_location);

    gl.vertex_attrib_pointer_with_i32(uv_location, 2, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);

    gl.draw_elements_with_i32(
        web_sys::WebGl2RenderingContext::TRIANGLES, 6, web_sys::WebGl2RenderingContext::UNSIGNED_INT, 0);

    Ok(())
}