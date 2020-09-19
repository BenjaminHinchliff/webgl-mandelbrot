use log::{info, Level};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlBuffer, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub struct Mandelbrot {
    ctx: WebGlRenderingContext,
    program: WebGlProgram,
    idx_buffer: WebGlBuffer,
    indices: Vec<u16>,
}

#[wasm_bindgen]
impl Mandelbrot {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        vert_src: &str,
        frag_src: &str,
    ) -> Result<Mandelbrot, JsValue> {
        let ctx = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let vert_shader = compile_shader(&ctx, WebGlRenderingContext::VERTEX_SHADER, vert_src)?;
        let frag_shader = compile_shader(&ctx, WebGlRenderingContext::FRAGMENT_SHADER, frag_src)?;
        let program = link_program(&ctx, &[vert_shader, frag_shader])?;
        ctx.use_program(Some(&program));
        ctx.uniform1f(ctx.get_uniform_location(&program, "aspect").as_ref(), canvas.width() as f32 / canvas.height() as f32);
        ctx.uniform1i(ctx.get_uniform_location(&program, "max_iter").as_ref(), 50);
        ctx.uniform1f(ctx.get_uniform_location(&program, "zoom").as_ref(), 2.0);
        ctx.uniform2f(ctx.get_uniform_location(&program, "offset").as_ref(), 3.0 / 4.0, 1.0 / 2.0);

        let verts = vec![
            -1.0, -1.0, // bottom left
            1.0, -1.0, // bottom right
            1.0, 1.0, // top right
            -1.0, 1.0, // top left
        ];

        let tex_coords = vec![
            0.0, 0.0, // bottom left
            1.0, 0.0, // bottom right
            1.0, 1.0, // top right
            0.0, 1.0, // top left
        ];

        let indices: Vec<u16> = vec![
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        let buffer = ctx
            .create_buffer()
            .ok_or_else(|| "failed to create vert buffer")?;
        ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&verts);

            ctx.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            )
        }

        ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(0);

        ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        let tex_buffer = ctx
            .create_buffer()
            .ok_or_else(|| "failed to create tex buffer")?;
        ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&tex_buffer));

        unsafe {
            let tex_array = js_sys::Float32Array::view(&tex_coords);

            ctx.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &tex_array,
                WebGlRenderingContext::STATIC_DRAW,
            )
        }

        ctx.vertex_attrib_pointer_with_i32(1, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(1);

        ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        let idx_buffer = ctx.create_buffer().ok_or_else(|| "failed to create index buffer")?;
        ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&idx_buffer));

        unsafe {
            let vert_array = js_sys::Uint16Array::view(&indices);

            ctx.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            )
        }

        ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);

        ctx.use_program(None);

        info!("mandelbrot setup");

        Ok(Mandelbrot {
            ctx,
            program,
            idx_buffer,
            indices,
        })
    }

    pub fn draw(&self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.idx_buffer));
        self.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        self.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.ctx.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            (self.indices.len()) as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        self.ctx.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        self.ctx.use_program(None);
    }
}

#[wasm_bindgen(start)]
pub fn initialize() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Error
    })
    .expect("failed to init logging");
}

fn compile_shader(
    ctx: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("unable to create shader object"))?;
    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    if ctx
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("unknown error creating shader")))
    }
}

fn link_program(
    ctx: &WebGlRenderingContext,
    shaders: &[WebGlShader],
) -> Result<WebGlProgram, String> {
    let program = ctx
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    for shader in shaders {
        ctx.attach_shader(&program, shader);
    }

    ctx.link_program(&program);

    if ctx
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(ctx
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
