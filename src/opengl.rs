use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glutin::{GlContext, GlWindow};
use nalgebra_glm as glm;
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem, ptr};

// This is just a namespace for now.
pub struct OpenGL;

impl OpenGL {
    pub fn initialize(gl_window: &GlWindow) {
        unsafe {
            gl_window
                .make_current()
                .expect("Error setting the current context");

            gl::load_with(|symbol| {
                gl_window.get_proc_address(symbol) as *const _
            });

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::STENCIL_TEST);
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

    pub fn create_camera_ubo(binding_point: u32) -> u32 {
        let mat_4_size = mem::size_of::<glm::Mat4>();
        let vec3_size = mem::size_of::<glm::TVec3<f32>>();

        // Allocate bytes of memory for this uniform block.
        // -------------------
        // projection: 64b
        // view: 64b
        // cam_pos: 16b
        let total = 2 * mat_4_size + vec3_size;
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
            if let Some(texture_id) = texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            };

            gl::BindVertexArray(vao);
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
            if let Some(texture_id) = texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            };

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, triangles);

            // Cleanup
            gl::BindVertexArray(0);
        }
    }
}
