use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader,
    WebGlUniformLocation,
};

struct UniformLocs {
    aspect: WebGlUniformLocation,
    max_iter: WebGlUniformLocation,
    zoom: WebGlUniformLocation,
    offset: WebGlUniformLocation,
}

impl UniformLocs {
    fn new(ctx: &WebGlRenderingContext, program: &WebGlProgram) -> Result<UniformLocs, String> {
        if let [aspect, max_iter, zoom, offset] = &["aspect", "max_iter", "zoom", "offset"].iter().map(|uniform| -> Result<WebGlUniformLocation, String> {
            ctx.get_uniform_location(&program, uniform).ok_or_else(|| String::from("unable to find uniform ") + uniform)
        }).collect::<Vec<_>>()[..] {
            Ok(UniformLocs {
                aspect: aspect.clone()?,
                max_iter: max_iter.clone()?,
                zoom: zoom.clone()?,
                offset: offset.clone()?,
            })
        } else {
            Err(String::from("invalid match pattern... somehow?"))
        }
    }
}

#[wasm_bindgen]
pub struct Mandelbrot {
    canvas: HtmlCanvasElement,
    ctx: WebGlRenderingContext,
    locs: UniformLocs,
    program: WebGlProgram,
    idx_buffer: WebGlBuffer,
    indices: Vec<u16>,
    pub zoom: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub iters: i32,
}

#[wasm_bindgen]
impl Mandelbrot {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        vert_src: &str,
        frag_src: &str,
        zoom: f32,
        x_pos: f32,
        y_pos: f32,
        iters: i32,
    ) -> Result<Mandelbrot, JsValue> {
        assert!(iters <= 10_000, "Due to fragment shader setup, you cannot have more than 10k iterations");
        let ctx = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let vert_shader = compile_shader(&ctx, WebGlRenderingContext::VERTEX_SHADER, vert_src)?;
        let frag_shader = compile_shader(&ctx, WebGlRenderingContext::FRAGMENT_SHADER, frag_src)?;
        let program = link_program(&ctx, &[vert_shader, frag_shader])?;
        ctx.use_program(Some(&program));
        
        let locs = UniformLocs::new(&ctx, &program)?;
        ctx.uniform1f(
            Some(&locs.aspect),
            canvas.width() as f32 / canvas.height() as f32,
        );
        ctx.uniform1i(Some(&locs.max_iter), iters);
        ctx.uniform1f(Some(&locs.zoom), zoom);
        ctx.uniform2f(
            Some(&locs.offset),
            x_pos,
            y_pos,
        );

        // code to setup the full-screen square for the fragment
        // shader render onto
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

        let idx_buffer = ctx
            .create_buffer()
            .ok_or_else(|| "failed to create index buffer")?;
        ctx.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&idx_buffer),
        );

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

        Ok(Mandelbrot {
            canvas,
            ctx,
            locs,
            program,
            idx_buffer,
            indices,
            zoom,
            x_pos,
            y_pos,
            iters,
        })
    }

    pub fn draw(&self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.idx_buffer),
        );

        self.ctx.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            (self.indices.len()) as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        self.ctx
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        self.ctx.use_program(None);
    }

    pub fn resize_viewport(&mut self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.uniform1f(
            Some(&self.locs.aspect),
            self.canvas.width() as f32 / self.canvas.height() as f32,
        );
        self.ctx.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        self.ctx.use_program(None);
    }

    pub fn refresh_zoom(&self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.uniform1f(Some(&self.locs.zoom), self.zoom);
        self.ctx.use_program(None);
    }

    pub fn refresh_position(&self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.uniform2f(
            Some(&self.locs.offset),
            self.x_pos,
            self.y_pos,
        );
        self.ctx.use_program(None);
    }

    pub fn refresh_iters(&self) {
        self.ctx.use_program(Some(&self.program));
        self.ctx.uniform1i(Some(&self.locs.max_iter), self.iters);
        self.ctx.use_program(None);
    }
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

#[wasm_bindgen(start)]
pub fn initialize() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
