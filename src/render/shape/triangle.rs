use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::app::State;
use crate::render::shader::Kind;
use crate::render::shader::Shader;
use crate::render::shape::Render;

pub struct Triangle<'a> {
    pub verticies: [f32; 9],
    pub colors: [f32; 12],
    pub shader: &'a Shader,
}

impl<'a> Render<'a> for Triangle<'a> {
    fn shader_kind() -> Kind {
        Kind::Basic
    }

    fn shader(&'a self) -> &'a Shader {
        self.shader
    }

    fn buffer_attributes(&self, gl: &WebGl2RenderingContext) {
        #![allow(clippy::cast_sign_loss)]

        let shader = self.shader();

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let col_attrib = gl.get_attrib_location(&shader.program, "col");

        gl.enable_vertex_attrib_array(pos_attrib as u32);
        gl.enable_vertex_attrib_array(col_attrib as u32);

        Triangle::buffer_f32_data(gl, &self.verticies[..], pos_attrib as u32, 3);
        Triangle::buffer_f32_data(gl, &self.colors[..], col_attrib as u32, 3);
    }

    fn render(&self, gl: &WebGl2RenderingContext, state: &State) {
        use webgl_matrix::Matrix;
        let shader = self.shader();

        let model_uni = shader.get_uniform_location(gl, "model");
        let model = webgl_matrix::Mat4::identity();
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &model);

        let view_uni = shader.get_uniform_location(gl, "view");
        let view = &state.camera().view();
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, view);

        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let perspective = &state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, perspective);

        gl.draw_arrays(GL::TRIANGLES, 0, 3);
    }
}
