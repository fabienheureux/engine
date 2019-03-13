use gl;
use gl::types::*;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ptr;
use std::str;

use crate::constants::SHADER_PATH;

#[derive(Default, Debug)]
pub struct Shader {
    pub id: u32,
    vert_name: String,
    frag_name: String,
}

impl Shader {
    pub fn new() -> Self {
        unsafe {
            Self {
                id: gl::CreateProgram(),
                ..Self::default()
            }
        }
    }

    pub fn with_vert(self, shader_name: &str) -> Self {
        let fragment = format!("{}.vert", shader_name);
        let shader = self.compile_shader(gl::VERTEX_SHADER, &fragment);

        unsafe {
            gl::AttachShader(self.id, shader);
            gl::LinkProgram(self.id);
            gl::DeleteShader(shader);
        }

        self
    }

    pub fn with_frag(self, shader_name: &str) -> Self {
        let fragment = format!("{}.frag", shader_name);
        let shader = self.compile_shader(gl::FRAGMENT_SHADER, &fragment);

        unsafe {
            gl::AttachShader(self.id, shader);
            gl::LinkProgram(self.id);
            gl::DeleteShader(shader);
        }

        self
    }

    pub fn delete_program(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    #[allow(dead_code)]
    pub fn set_int(&self, var_name: &str, value: i32) {
        let shader_variable = self.get_location(var_name);
        unsafe { gl::Uniform1i(shader_variable, value) }
    }

    pub fn set_float(&self, var_name: &str, value: f32) {
        let shader_variable = self.get_location(var_name);
        unsafe { gl::Uniform1f(shader_variable, value) }
    }

    pub fn set_vec3(&self, var_name: &str, values: &(f32, f32, f32)) {
        let shader_variable = self.get_location(var_name);
        let (a, b, c) = *values;
        unsafe { gl::Uniform3f(shader_variable, a, b, c) }
    }

    #[allow(unused)]
    pub fn set_vec4(&self, var_name: &str, values: &(f32, f32, f32, f32)) {
        let shader_variable = self.get_location(var_name);
        let (a, b, c, d) = *values;
        unsafe {
            gl::Uniform4f(shader_variable, a, b, c, d);
        }
    }

    pub fn set_matrix4(&self, var_name: &str, transform: &[f32]) {
        let shader_variable = self.get_location(var_name);
        unsafe {
            gl::UniformMatrix4fv(
                shader_variable,
                1,
                gl::FALSE,
                transform.as_ptr(),
            );
        }
    }

    fn get_location(&self, var_name: &str) -> GLint {
        let var_name = CString::new(var_name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, var_name.as_ptr()) }
    }

    fn compile_shader(&self, shader_type: GLenum, file_path: &str) -> u32 {
        let mut base = PathBuf::from(SHADER_PATH);

        base.push(file_path);
        let mut shader_file = File::open(base).unwrap();
        let mut shader_string = String::new();

        // Transform file to string and store it in a variable
        shader_file
            .read_to_string(&mut shader_string)
            .expect("Failed to read vertex shader file");

        // convert to C compatible string
        let shader_source_string =
            CString::new(shader_string.as_bytes()).unwrap();

        unsafe {
            let shader = gl::CreateShader(shader_type);

            gl::ShaderSource(
                shader,
                1,
                &shader_source_string.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(shader);
            self.check_shader_compile_error(shader);

            shader
        }
    }

    unsafe fn check_shader_compile_error(&self, shader: GLuint) {
        const CAPACITY: usize = 1024;
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(CAPACITY);
        info_log.set_len(CAPACITY - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                shader,
                CAPACITY as i32,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }
    }
}
