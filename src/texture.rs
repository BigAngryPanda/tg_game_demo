#[allow(unused_imports)]
use crate::log;

use image::ImageReader;
use std::io::Cursor;

pub type Texture = Option<web_sys::WebGlTexture>;

pub fn from_png_data(
    gl: &web_sys::WebGl2RenderingContext,
    raw_data: &[u8],
    format: u32
) -> Texture {
    let reader = ImageReader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .expect("Cursor io never fails");

    let image = reader.decode().unwrap();

    from_rgba_data(
        gl,
        image.as_bytes(),
        image.width() as i32,
        image.height() as i32,
        format
    )
}

pub fn from_rgba_data(
    gl: &web_sys::WebGl2RenderingContext,
    rgba_data: &[u8],
    width: i32,
    height: i32,
    format: u32
) -> Texture {
    let texture = gl.create_texture();

    gl.bind_texture(
        web_sys::WebGl2RenderingContext::TEXTURE_2D,
        texture.as_ref(),
    );

    gl.pixel_storei(web_sys::WebGl2RenderingContext::UNPACK_ALIGNMENT, 1);

    let level = 0;
    let internal_format = format;
    let border = 0;
    let src_format = format;
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
        Some(rgba_data),
    )
    .expect("Failed to write texture");

    gl.generate_mipmap(web_sys::WebGl2RenderingContext::TEXTURE_2D);

    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_WRAP_T, web_sys::WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_WRAP_S, web_sys::WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_WRAP_R, web_sys::WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_MIN_FILTER, web_sys::WebGl2RenderingContext::LINEAR as i32);
    gl.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_MAG_FILTER, web_sys::WebGl2RenderingContext::LINEAR as i32);

    texture
}
