use crate::{
    asset_manager::Texture,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
};
use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glutin::{ContextTrait, WindowedContext};
use nalgebra_glm as glm;
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem, ptr};

// This is just a namespace for now.
pub struct OpenGL;

impl OpenGL {
    pub fn initialize(window_context: &WindowedContext) {
        unsafe {
            window_context
                .make_current()
                .expect("Error setting the current context");

            gl::load_with(|symbol| {
                window_context.get_proc_address(symbol) as *const _
            });

            OpenGL::set_depth_buffer(true);
            gl::Enable(gl::STENCIL_TEST);
            gl::Enable(gl::MULTISAMPLE);  
            // Specify the default color.
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        }
    }

    pub fn fill_mode() {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) }
    }

    pub fn line_mode() {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    /// Generate Vertex Array Object.
    pub fn gen_vao() -> u32 {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) }
        vao
    }

    /// Generate buffer.
    pub fn gen_buffer() -> u32 {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        id
    }

    pub fn gen_fbo() -> u32 {
        let mut fbo = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
        }
        fbo
    }

    pub fn use_fbo(fbo: u32) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        }
    }

    pub fn create_fbo(dpi_ratio: i32) -> (u32, u32) {
        let fbo = Self::gen_fbo();
        let mut cbo = 1;

        let retina_factor = dpi_ratio;

        unsafe {
            OpenGL::use_fbo(fbo);

            // Generate color attachment.
            gl::GenTextures(1, &mut cbo);

            gl::BindTexture(gl::TEXTURE_2D, cbo);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                SCREEN_WIDTH as i32 * retina_factor,
                SCREEN_HEIGHT as i32 * retina_factor,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // Attach the texture to the fbo.
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                cbo,
                0,
            );

            let mut rbo = 0;
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                SCREEN_WIDTH as i32 * retina_factor,
                SCREEN_HEIGHT as i32 * retina_factor,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
                != gl::FRAMEBUFFER_COMPLETE
            {
                panic!("fbo not completed");
            }

            OpenGL::use_fbo(0);
        }

        (fbo, cbo)
    }

    pub fn create_camera_ubo(binding_point: u32) -> u32 {
        let mat_4_size = mem::size_of::<glm::Mat4>();
        let vec3_size = mem::size_of::<glm::TVec3<f32>>();

        // Allocate bytes of memory for this uniform block.
        // -------------------
        // projection: 64b
        // view: 64b
        // skybox_v: 64b
        // cam_pos: 16b
        let total = 3 * mat_4_size + vec3_size;
        OpenGL::bind_ubo(binding_point, total as isize)
    }

    pub fn create_lights_ubo(binding_point: u32) -> u32 {
        // Allocate bytes of memory for this uniform block.
        // -------------------
        // type: 4b
        // light_dir: 12b
        // light_pos: 12b
        // ambient: 12b
        // diffuse: 12b
        // specular: 12b
        let total = 96;
        OpenGL::bind_ubo(binding_point, 2 * total as isize)
    }

    // Generate uniform object buffer then bind it to a
    // given binding point.
    pub fn bind_ubo(binding_point: u32, size: isize) -> u32 {
        let ubo = OpenGL::gen_buffer();

        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, ubo);

            // Cleanup.
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }

        ubo
    }

    pub fn set_uniform_block(shader_id: u32, binding_point: u32, name: &str) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            let index = gl::GetUniformBlockIndex(shader_id, c_str.as_ptr());
            gl::UniformBlockBinding(shader_id, index, binding_point);
        }
    }

    pub fn set_mat4_to_ubo(matrix: glm::Mat4, ubo: u32, offset: isize) {
        let matrix = glm::value_ptr(&matrix);

        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                offset,
                mem::size_of::<glm::Mat4>() as isize,
                matrix as *const _ as *const c_void,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn set_vec3_to_ubo(vec: glm::TVec3<f32>, ubo: u32, offset: isize) {
        let vec = glm::value_ptr(&vec);

        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                offset,
                mem::size_of::<glm::TVec3<f32>>() as isize,
                vec as *const _ as *const c_void,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn set_int_to_ubo(int: i32, ubo: u32, offset: isize) {
        let r_int: *const i32 = &int;

        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                offset,
                mem::size_of::<i32>() as isize,
                r_int as *const c_void,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn set_depth_buffer(enabled: bool) {
        unsafe {
            if enabled {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
        }
    }

    pub fn clear() {
        unsafe {
            // Clear color buffer with the color specified by gl::ClearColor.
            // Also clear the depth and stencil buffer.
            gl::Clear(
                gl::COLOR_BUFFER_BIT
                    | gl::DEPTH_BUFFER_BIT
                    | gl::STENCIL_BUFFER_BIT,
            );
        }
    }
    pub fn clear_color((r, g, b): (f32, f32, f32)) {
        unsafe {
            // Clear color buffer with specified color.
            gl::ClearColor(r, g, b, 1.);
            Self::clear();
        }
    }

    pub fn gen_screen_quad() -> u32 {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices: [f32; 24] = [
            // vertex, text coord.
            -1.0,  1.0, 0.0, 1.0,
            -1.0, -1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 0.0,

            -1.0,  1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0,
            1.0,  1.0, 1.0, 1.0
        ];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        vao
    }

    // Create a little plane.
    // Return vao and texture id and positions.
    pub fn gen_plane() -> (u32, i32, bool) {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices: [f32; 48] = [
            // vertex, tex, normal.
            -1., 0., 1., 0., 5., 0., 1., 0.,
            1., 0., -1., 5., 0.,0., 1., 0.,
            1., 0., 1., 5., 5., 0., 1., 0.,
            -1., 0., 1., 0., 5.,0., 1., 0.,
            -1., 0., -1., 0., 0.,0., 1., 0.,
            1., 0., -1., 5., 0.,0., 1., 0.,
        ];

        // let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();
        // let ebo = OpenGL::gen_buffer();

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::BufferData(
            //     gl::ELEMENT_ARRAY_BUFFER,
            //     (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            //     &indices[0] as *const i32 as *const c_void,
            //     gl::STATIC_DRAW,
            // );

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
                (5 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }

        (vao, 6, false)
    }

    // Create a little cube
    // Return vao and texture id and positions.
    pub fn gen_cube() -> (u32, i32, bool) {
        // position + text coord + normal
        let vertices: [f32; 288] = [
            -0.5, -0.5, -0.5, 0.0, 0.0, 0., 0., -1., 0.5, -0.5, -0.5, 1.0, 0.0,
            0., 0., -1., 0.5, 0.5, -0.5, 1.0, 1.0, 0., 0., -1., 0.5, 0.5, -0.5,
            1.0, 1.0, 0., 0., -1., -0.5, 0.5, -0.5, 0.0, 1.0, 0., 0., -1.,
            -0.5, -0.5, -0.5, 0.0, 0.0, 0., 0., -1., -0.5, -0.5, 0.5, 0.0, 0.0,
            0., 0., 1., 0.5, -0.5, 0.5, 1.0, 0.0, 0., 0., 1., 0.5, 0.5, 0.5,
            1.0, 1.0, 0., 0., 1., 0.5, 0.5, 0.5, 1.0, 1.0, 0., 0., 1., -0.5,
            0.5, 0.5, 0.0, 1.0, 0., 0., 1., -0.5, -0.5, 0.5, 0.0, 0.0, 0., 0.,
            1., -0.5, 0.5, 0.5, 1.0, 0.0, -1., 0., 0., -0.5, 0.5, -0.5, 1.0,
            1.0, -1., 0., 0., -0.5, -0.5, -0.5, 0.0, 1.0, -1., 0., 0., -0.5,
            -0.5, -0.5, 0.0, 1.0, -1., 0., 0., -0.5, -0.5, 0.5, 0.0, 0.0, -1.,
            0., 0., -0.5, 0.5, 0.5, 1.0, 0.0, -1., 0., 0., 0.5, 0.5, 0.5, 1.0,
            0.0, 1., 0., 0., 0.5, 0.5, -0.5, 1.0, 1.0, 1., 0., 0., 0.5, -0.5,
            -0.5, 0.0, 1.0, 1., 0., 0., 0.5, -0.5, -0.5, 0.0, 1.0, 1., 0., 0.,
            0.5, -0.5, 0.5, 0.0, 0.0, 1., 0., 0., 0.5, 0.5, 0.5, 1.0, 0.0, 1.,
            0., 0., -0.5, -0.5, -0.5, 0.0, 1.0, 0., -1., 0., 0.5, -0.5, -0.5,
            1.0, 1.0, 0., -1., 0., 0.5, -0.5, 0.5, 1.0, 0.0, 0., -1., 0., 0.5,
            -0.5, 0.5, 1.0, 0.0, 0., -1., 0., -0.5, -0.5, 0.5, 0.0, 0.0, 0.,
            -1., 0., -0.5, -0.5, -0.5, 0.0, 1.0, 0., -1., 0., -0.5, 0.5, -0.5,
            0.0, 1.0, 0., 1., 0., 0.5, 0.5, -0.5, 1.0, 1.0, 0., 1., 0., 0.5,
            0.5, 0.5, 1.0, 0.0, 0., 1., 0., 0.5, 0.5, 0.5, 1.0, 0.0, 0., 1.,
            0., -0.5, 0.5, 0.5, 0.0, 0.0, 0., 1., 0., -0.5, 0.5, -0.5, 0.0,
            1.0, 0., 1., 0.,
        ];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<GLfloat>() as GLsizei,
                (5 * mem::size_of::<GLfloat>()) as *const c_void,
            );

            gl::EnableVertexAttribArray(2);
        }

        (vao, 36, false)
    }

    pub fn gen_skybox() -> (u32, i32, bool) {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices: [f32; 108] = [
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0
        ];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

        (vao, 36, false)
    }

    pub fn load_cubemap(skybox: Vec<&Texture>) -> u32 {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);

            skybox.iter().enumerate().for_each(|(i, img)| {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGBA as i32,
                    img.width,
                    img.height,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.raw.as_ptr() as *const c_void,
                );
            });

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
        }

        id
    }

    pub fn load_glyph(width: i32, height: i32, image: &Vec<u8>) -> u32 {
        let mut id: u32 = 0;

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            // Load texture data.
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width,
                height,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        id
    }

    pub fn create_font_quad() -> (u32, u32) {
        let vao = Self::gen_vao();
        let vbo = Self::gen_buffer();

        let vertices: [f32; 24] = [0.; 24];

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;

            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0);
        }
        (vao, vbo)
    }

    pub fn update_font_quad(
        vao: u32,
        vbo: u32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices: [f32; 24] = [
            // vertex, text coord.
            x, y + height, 0.0, 0.0,
            x,  y, 0.0, 1.0,
            x + width, y, 1.0, 1.0,

            x, y + height, 0.0, 0.0,
            x + width, y, 1.0, 1.0,
            x + width, y + height, 1.0, 0.0,
        ];

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
            );

            gl::BindVertexArray(0);
        }
    }

    /// This method can load the attached texture into the memory and give it
    /// to the GPU.
    /// Works only for RGBA textures.
    pub fn load_2d_texture(width: i32, height: i32, image: &Vec<u8>) -> u32 {
        let mut id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            // Load texture data.
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        id
    }

    pub fn use_shader(id: u32) {
        unsafe { gl::UseProgram(id) }
    }

    pub fn draw_with_ebo(vao: u32, texture: Option<u32>, triangles: i32) {
        unsafe {
            gl::BindVertexArray(vao);

            if let Some(texture_id) = texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            };

            gl::DrawElements(
                gl::TRIANGLES,
                triangles,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            // Cleanup
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(vao: u32, texture: Option<u32>, triangles: i32) {
        unsafe {
            gl::BindVertexArray(vao);

            if let Some(texture_id) = texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            };

            gl::DrawArrays(gl::TRIANGLES, 0, triangles);

            // Cleanup
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_skybox(vao: u32, texture: u32, triangles: i32) {
        unsafe {
            gl::BindVertexArray(vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture);

            gl::DrawArrays(gl::TRIANGLES, 0, triangles);

            // Cleanup
            gl::BindVertexArray(0);
        }
    }
}
