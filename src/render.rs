use web_sys;

use crate::shape::*;
use crate::texture::Texture;

#[allow(unused_imports)]
use crate::log;

#[derive(Debug, Clone, Copy)]
pub struct TransformInfo(pub f32, pub f32);

impl TransformInfo {
    // column order
    pub fn scale_matrix(&self) -> [f32; 16] {
        [
            self.0, 0.0, 0.0, 0.0,
            0.0, self.1, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn translation_matrix(&self) -> [f32; 16] {
        [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            self.0, self.1, 1.0, 1.0
        ]
    }

    pub fn id() -> [f32; 16] {
        [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]
    }
}

#[derive(Debug, Clone, Copy)]
struct ShapeDescriptor {
    pub offset: usize,
    pub count: usize,
}

impl ShapeDescriptor {
    fn offset_bytes(&self, v_size: usize) -> u32 {
        (v_size*self.offset*std::mem::size_of::<f32>()) as u32
    }

    fn size_bytes(&self, v_size: usize) -> u32 {
        (v_size*self.count*std::mem::size_of::<f32>()) as u32
    }

    fn offset_vertex(&self, v_size: usize) -> usize {
        v_size*self.offset
    }

    fn size_vertex(&self, v_size: usize) -> usize {
        v_size*self.count
    }

    fn vertex_range(&self, v_size: usize) -> std::ops::Range<usize> {
        std::ops::Range { start: self.offset_vertex(v_size), end: self.offset_vertex(v_size) + self.size_vertex(v_size) }
    }
}

#[derive(Debug)]
pub struct IndicesRender {
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    descriptors: Vec<ShapeDescriptor>,
    vertex_buffer: Option<web_sys::WebGlBuffer>,
    vertex_location: u32,
}

impl IndicesRender {
    pub fn new(gl: &web_sys::WebGl2RenderingContext) -> IndicesRender {
        let gl_program: web_sys::WebGlProgram = gl.create_program().expect("Failed to create program");

        // index buffer
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, gl.create_buffer().as_ref());

        let vertex_buffer = gl.create_buffer();

        // vertices
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());

        // textures
        gl.active_texture(web_sys::WebGl2RenderingContext::TEXTURE0);

        IndicesRender {
            context: gl.clone(),
            program: gl_program,
            vertices: Vec::new(),
            indices: Vec::new(),
            descriptors: Vec::new(),
            vertex_buffer,
            vertex_location: 0
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
        let offset: u32 = self.vertices.len() as u32;

        for index in &shape.indices {
            self.indices.push(offset + index);
        }

        self.descriptors.push(ShapeDescriptor { offset: self.indices.len() - shape.indices.len(), count: shape.indices.len() });

        for vertex in &shape.vertices {
            self.vertices.push(vertex.x());
            self.vertices.push(vertex.y());
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

            let vertex_location: u32 = self.context.get_attrib_location(&self.program, vertex_in) as u32;

            self.context.enable_vertex_attrib_array(vertex_location);

            let idx_array = web_sys::js_sys::Uint32Array::view(&self.indices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &idx_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn write_uniform(&self, data: &[f32], uniform: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, uniform).expect("Failed to get uniform location");

        self.context.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
    }

    pub fn enable_texture(&self, texture: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, texture).expect("Failed to get uniform location");

        self.context.uniform1i(Some(&location), 0);
    }

    pub fn draw(&self, shape_idx: usize, texture: &Texture) {
        self.context.bind_texture(web_sys::WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        let desc: &ShapeDescriptor = &self.descriptors[shape_idx];

        self.context.draw_elements_with_i32(
            web_sys::WebGl2RenderingContext::TRIANGLES,
            desc.count as i32,
            web_sys::WebGl2RenderingContext::UNSIGNED_INT,
            desc.offset as i32
        );
    }

    pub fn setup_render(&self) {
        self.context.use_program(Some(&self.program));
        self.context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, self.vertex_buffer.as_ref());

        self.context.vertex_attrib_pointer_with_i32(self.vertex_location, Self::VERTEX_SIZE as i32, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    }

    const VERTEX_SIZE: usize = 2;
}

// Troubles with sync
// https://github.com/gfxfundamentals/webgl-fundamentals/discussions/363
#[derive(Debug)]
pub struct FeedbackRender {
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
    vertices: Vec<f32>,
    descriptors: Vec<ShapeDescriptor>,

    // state
    vertex_buffer: Option<web_sys::WebGlBuffer>,
    vertex_location: u32,
}

impl FeedbackRender {
    pub fn new(gl: &web_sys::WebGl2RenderingContext) -> FeedbackRender {
        let gl_program: web_sys::WebGlProgram = gl.create_program().expect("Failed to create program");

        let vertex_buffer = gl.create_buffer();

        // vertices
        gl.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());

        // transform feedback
        let varyings: wasm_bindgen::JsValue = wasm_bindgen::JsValue::from(vec![String::from("vertOut")]);

        gl.transform_feedback_varyings(&gl_program, &varyings, web_sys::WebGl2RenderingContext::SEPARATE_ATTRIBS);

        gl.bind_transform_feedback(web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK, gl.create_transform_feedback().as_ref());

        gl.bind_buffer_base(web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, gl.create_buffer().as_ref());

        // textures
        gl.active_texture(web_sys::WebGl2RenderingContext::TEXTURE0);

        FeedbackRender {
            context: gl.clone(),
            program: gl_program,
            vertices: Vec::new(),
            descriptors: Vec::new(),
            vertex_buffer,
            vertex_location: 0
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
        self.descriptors.push(ShapeDescriptor { offset: self.vertices.len() / Self::VERTEX_SIZE, count: shape.indices.len() });

        for &i in shape.indices.iter() {
            self.vertices.push(shape.vertices[i as usize].x());
            self.vertices.push(shape.vertices[i as usize].y());
        }
    }

    pub fn write_vertices(&mut self, vertex_in: &str) {
        unsafe {
            let vert_array = web_sys::js_sys::Float32Array::view(&self.vertices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        self.vertex_location = self.context.get_attrib_location(&self.program, vertex_in) as u32;

        self.context.enable_vertex_attrib_array(self.vertex_location);

        unsafe {
            let tf_array = web_sys::js_sys::Float32Array::view(&self.vertices);

            self.context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
                &tf_array,
                web_sys::WebGl2RenderingContext::STREAM_READ,
            );
        }
    }

    pub fn enable_texture(&self, texture: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, texture).expect("Failed to get uniform location");

        self.context.uniform1i(Some(&location), 0);
    }

    pub fn write_float(&self, var: f32, uniform: &str) {
        let location =
            self.context.get_uniform_location(&self.program, uniform);

        self.context.uniform1f(location.as_ref(), var);
    }

    pub fn write_uniform(&self, data: &[f32], uniform: &str) {
        let location: web_sys::WebGlUniformLocation =
            self.context.get_uniform_location(&self.program, uniform).expect("Failed to get uniform location");

        self.context.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
    }

    pub fn draw(&self, shape_idx: usize, texture: &Texture) {
        self.context.use_program(Some(&self.program));

        self.context.bind_texture(web_sys::WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        self.context.begin_transform_feedback(web_sys::WebGl2RenderingContext::TRIANGLES);

        let desc: &ShapeDescriptor = &self.descriptors[shape_idx];

        self.context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, desc.offset as i32, desc.count as i32);

        self.context.end_transform_feedback();

        self.context.finish();

        let buffer: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(self.vertices.as_ptr() as *mut u8, std::mem::size_of::<f32>()*self.vertices.len())
        };

        self.context.get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset_and_length(
            web_sys::WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, buffer,
            desc.offset_bytes(Self::VERTEX_SIZE), desc.size_bytes(Self::VERTEX_SIZE));
    }

    pub fn read_vertices(&self, shape_idx: usize) -> &[f32] {
        let desc: &ShapeDescriptor = &self.descriptors[shape_idx];

        &self.vertices[desc.vertex_range(Self::VERTEX_SIZE)]
    }

    pub fn setup_render(&self) {
        self.context.use_program(Some(&self.program));
        self.context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, self.vertex_buffer.as_ref());

        self.context.vertex_attrib_pointer_with_i32(self.vertex_location, Self::VERTEX_SIZE as i32, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    }

    // How many f32 per vertex
    const VERTEX_SIZE: usize = 2;
}
