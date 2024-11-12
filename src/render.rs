use web_sys;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::Shape;

// How many f32 per vertex
const VERTEX_SIZE: usize = 2;

#[derive(Debug, Clone, Copy)]
struct ShapeDescriptor {
    pub offset: usize,
    pub count: usize,
}

impl ShapeDescriptor {
    fn offset_bytes(&self) -> u32 {
        (VERTEX_SIZE*self.offset*std::mem::size_of::<f32>()) as u32
    }

    fn size_bytes(&self) -> u32 {
        (VERTEX_SIZE*self.count*std::mem::size_of::<f32>()) as u32
    }

    fn offset_vertex(&self) -> usize {
        VERTEX_SIZE*self.offset
    }

    fn size_vertex(&self) -> usize {
        VERTEX_SIZE*self.count
    }

    fn vertex_range(&self) -> std::ops::Range<usize> {
        std::ops::Range { start: self.offset_vertex(), end: self.offset_vertex() + self.size_vertex() }
    }
}

pub struct IndicesRender {
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
}

impl IndicesRender {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> IndicesRender {
        let gl: web_sys::WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .expect("Failed to get context")
            .expect("Failed to get js object")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Failed to get WebGl2RenderingContext");

        let gl_program: web_sys::WebGlProgram = gl.create_program().expect("Failed to create program");

        // index buffer
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, gl.create_buffer().as_ref());

        // vertices
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, gl.create_buffer().as_ref());

        // uniform
        gl.bind_buffer(web_sys::WebGl2RenderingContext::UNIFORM_BUFFER, gl.create_buffer().as_ref());

        gl.bind_buffer(web_sys::WebGl2RenderingContext::UNIFORM_BUFFER, gl.create_buffer().as_ref());

        IndicesRender {
            context: gl,
            program: gl_program,
        }
    }

    pub fn link_shader(&self, source: &str, kind: u32) {
        let shader = self.context.create_shader(kind).expect("Failed to create fragment shader");
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);
        self.context.attach_shader(&self.program, &shader);
    }

    pub fn link_program(&self) {
        self.context.link_program(&self.program);
        self.context.use_program(Some(&self.program));
    }

    pub fn write_indices(&self, indices: &[u32]) {
        unsafe {
            let idx_array = web_sys::js_sys::Uint32Array::view(&indices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &idx_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn write_vertices(&self, vertices: &[f32], vertex_in: &str) {
        unsafe {
            let vert_array = web_sys::js_sys::Float32Array::view(&vertices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vertex_location: u32 = self.context.get_attrib_location(&self.program, vertex_in) as u32;

        self.context.enable_vertex_attrib_array(vertex_location);

        self.context.vertex_attrib_pointer_with_i32(vertex_location, 2, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    }

    pub fn write_uniform(&self, data: &[f32], uniform: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, uniform).expect("Failed to get uniform location");

        self.context.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
    }

    pub fn draw(&self, count: i32, offset: i32) {
        self.context.use_program(Some(&self.program));

        self.context.draw_elements_with_i32(
            web_sys::WebGl2RenderingContext::TRIANGLES, count, web_sys::WebGl2RenderingContext::UNSIGNED_INT, offset);
    }

    pub fn clear(&self) {
        self.context.use_program(Some(&self.program));

        self.context.clear_color(1.0, 1.0, 1.0, 1.0);
        self.context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

pub struct FeedbackRender {
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
    vertices: Vec<f32>,
    descriptors: Vec<ShapeDescriptor>,
}

impl FeedbackRender {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> FeedbackRender {
        let gl: web_sys::WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .expect("Failed to get context")
            .expect("Failed to get js object")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Failed to get WebGl2RenderingContext");

        let gl_program: web_sys::WebGlProgram = gl.create_program().expect("Failed to create program");

        // vertices
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, gl.create_buffer().as_ref());

        // uniform
        gl.bind_buffer(web_sys::WebGl2RenderingContext::UNIFORM_BUFFER, gl.create_buffer().as_ref());
        gl.bind_buffer(web_sys::WebGl2RenderingContext::UNIFORM_BUFFER, gl.create_buffer().as_ref());

        // transform feedback
        let varyings: wasm_bindgen::JsValue = wasm_bindgen::JsValue::from(vec![String::from("vertOut")]);

        gl.transform_feedback_varyings(&gl_program, &varyings, web_sys::WebGl2RenderingContext::SEPARATE_ATTRIBS);

        gl.bind_transform_feedback(web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK, gl.create_transform_feedback().as_ref());

        gl.bind_buffer_base(web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, gl.create_buffer().as_ref());

        FeedbackRender {
            context: gl,
            program: gl_program,
            vertices: Vec::new(),
            descriptors: Vec::new(),
        }
    }

    pub fn link_shader(&self, source: &str, kind: u32) {
        let shader = self.context.create_shader(kind).expect("Failed to create fragment shader");
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);
        self.context.attach_shader(&self.program, &shader);
    }

    pub fn link_program(&self) {
        self.context.link_program(&self.program);
        self.context.use_program(Some(&self.program));
    }

    pub fn add(&mut self, shape: &Shape) {
        self.descriptors.push(ShapeDescriptor { offset: self.vertices.len() / VERTEX_SIZE, count: shape.indices.len() });

        for &i in shape.indices.iter() {
            self.vertices.push(shape.vertices[i as usize].x());
            self.vertices.push(shape.vertices[i as usize].y());
        }
    }

    pub fn write_vertices(&self, vertex_in: &str) {
        unsafe {
            let vert_array = web_sys::js_sys::Float32Array::view(&self.vertices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vertex_location: u32 = self.context.get_attrib_location(&self.program, vertex_in) as u32;

        self.context.enable_vertex_attrib_array(vertex_location);

        self.context.vertex_attrib_pointer_with_i32(vertex_location, 2, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);

        unsafe {
            let tf_array = web_sys::js_sys::Float32Array::view(&self.vertices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
                &tf_array,
                web_sys::WebGl2RenderingContext::STREAM_READ,
            );
        }
    }

    pub fn write_uniform(&self, data: &[f32], uniform: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, uniform).expect("Failed to get uniform location");

        self.context.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
    }

    pub fn draw(&self, shape_idx: usize) {
        self.context.use_program(Some(&self.program));

        self.context.begin_transform_feedback(web_sys::WebGl2RenderingContext::TRIANGLES);

        let desc: &ShapeDescriptor = &self.descriptors[shape_idx];

        self.context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, desc.offset as i32, desc.count as i32);

        self.context.end_transform_feedback();

        let buffer: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(self.vertices.as_ptr() as *mut u8, std::mem::size_of::<f32>()*self.vertices.len())
        };

        self.context.get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset_and_length(
            web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, buffer,
            desc.offset_bytes(), desc.size_bytes());
    }

    pub fn clear(&self) {
        self.context.use_program(Some(&self.program));

        self.context.clear_color(1.0, 1.0, 1.0, 1.0);
        self.context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn read_vertices(&self, shape_idx: usize) -> &[f32] {
        //let buffer: &mut [u8] = unsafe {
        //    std::slice::from_raw_parts_mut(self.vertices.as_ptr() as *mut u8, std::mem::size_of::<f32>()*self.vertices.len())
        //};

        //self.context.get_buffer_sub_data_with_i32_and_u8_array(web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, buffer);

        let desc: &ShapeDescriptor = &self.descriptors[shape_idx];

        log::write(&format!("DEBUG!!! {} {}", desc.offset_vertex(), desc.size_vertex()));
        log::write(&format!("DEBUG!!! {:?}", &self.vertices[desc.vertex_range()]));

        &self.vertices[desc.vertex_range()]
    }
}
