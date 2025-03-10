use gl::types::*;

pub struct VertexArray(pub GLuint);

impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        };
        if vao != 0 {
            return Some(Self(vao));
        } else {
            return None;
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        };
    }

    pub fn clear_binding() {
        unsafe {
            gl::BindVertexArray(0);
        };
    }
}
