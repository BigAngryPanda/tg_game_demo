pub const VERTEX_SHADER: &str = r#"#version 300 es

in vec2 vertexPosition;

uniform mat4 scale;
uniform mat4 translation;

void main() {
    gl_Position = translation*scale*vec4(vertexPosition, 0, 1);
}"#;

pub const FEEDBACK_VERTEX_SHADER: &str = r#"#version 300 es

in vec2 vertexPosition;

uniform mat4 scale;
uniform mat4 translation;

out vec4 vertexPos;
out vec2 vertOut;

void main() {
    gl_Position = translation*scale*vec4(vertexPosition, 0, 1);
    vertexPos = gl_Position;
    vertOut = gl_Position.xy;
}"#;

pub const VERTEX_SHADER_KIND: u32 = web_sys::WebGl2RenderingContext::VERTEX_SHADER;

pub const FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec4 vertexPos;

out vec4 fragColor;

void main() {
    fragColor = vertexPos;
}"#;

pub const FRAGMENT_SHADER_KIND: u32 = web_sys::WebGl2RenderingContext::FRAGMENT_SHADER;