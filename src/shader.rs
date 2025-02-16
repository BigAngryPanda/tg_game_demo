pub mod background {
    pub const VERTEX_SHADER: &str = r#"#version 300 es

    in vec2 vertexPosition;

    uniform mat4 scale;
    uniform mat4 translation;

    out vec2 uv;

    void main() {
        gl_Position = translation*scale*vec4(vertexPosition, 1.0, 1.0);
        uv = vec2(1, 1) - (vertexPosition.xy + vec2(1, 1)) * 0.5;
    }"#;

    pub const FRAGMENT_SHADER: &str = r#"#version 300 es

    precision mediump float;

    in vec2 uv;

    uniform sampler2D tex;

    out vec4 fragColor;

    void main() {
        fragColor = texture(tex, uv);
    }"#;
}

pub mod feedback {
    pub const VERTEX_SHADER: &str = r#"#version 300 es

    in vec2 vertexPosition;

    uniform mat4 scale;
    uniform mat4 translation;
    uniform float t;

    out vec4 vertexPos;
    out vec2 vertOut;
    out vec2 uv;

    void main() {
        vec2 pos = vertexPosition;
        pos.y = pos.y*t + t - 1.0;
        gl_Position = translation*scale*vec4(pos, 0.0, 1.0);
        vertexPos = gl_Position;
        vertOut = gl_Position.xy;
        uv = (pos.xy + vec2(1, 1)) * 0.5;
        uv.y = -uv.y*t + t;
    }"#;

    pub const FRAGMENT_SHADER: &str = r#"#version 300 es

    precision mediump float;

    in vec4 vertexPos;
    in vec2 uv;

    uniform sampler2D tex;

    out vec4 fragColor;

    void main() {
        fragColor = texture(tex, uv);
    }"#;
}

pub const VERTEX_SHADER_KIND: u32 = web_sys::WebGl2RenderingContext::VERTEX_SHADER;

pub const FRAGMENT_SHADER_KIND: u32 = web_sys::WebGl2RenderingContext::FRAGMENT_SHADER;