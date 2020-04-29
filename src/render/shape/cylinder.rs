use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use webgl_matrix::{Mat4, Matrix, Vec3, Vector};

use crate::app::State;
use crate::render::shader::Kind;
use crate::render::shader::Shader;
use crate::render::shape::Render;

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    /*
     *  i  -j   k
     * u0  u1  u2
     * v0  v1  v2
     */
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

pub struct VBO {
    pub verticies: Vec<f32>,
    pub indicies: Vec<u16>,
}

pub struct Cylinder<'a, 'b> {
    pub object: &'b VBO,
    pub shader: &'a Shader,
    pub radius: f32,
    pub color_start: [f32; 4],
    pub color_end: [f32; 4],
    pub position_start: [f32; 3],
    pub position_end: [f32; 3],
}

impl VBO {
    pub fn new(slices: u16) -> Self {
        #![allow(clippy::identity_op)]

        use std::f32::consts::PI;

        let mut vertex_position_data = Vec::<f32>::new();
        let mut index_data = Vec::<u16>::new();

        // Extra value is to force completion of the circle
        for i in 0..slices {
            let theta = f32::from(i) * 2.0 * PI / f32::from(slices);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            // Index on top circle (0, 2, 4, 6, 8...)
            vertex_position_data.push(cos_theta);
            vertex_position_data.push(sin_theta);
            vertex_position_data.push(0.5);

            // Index on bottom circle (1, 3, 5, 7, 9...)
            vertex_position_data.push(cos_theta);
            vertex_position_data.push(sin_theta);
            vertex_position_data.push(-0.5);

            // Rectangle on side
            index_data.push(i * 2 + 0);
            index_data.push(i * 2 + 2);
            index_data.push(i * 2 + 1);

            index_data.push(i * 2 + 2);
            index_data.push(i * 2 + 1);
            index_data.push(i * 2 + 3);
        }

        vertex_position_data.push(1.0);
        vertex_position_data.push(0.0);
        vertex_position_data.push(0.5);

        vertex_position_data.push(1.0);
        vertex_position_data.push(0.0);
        vertex_position_data.push(-0.5);

        Self {
            verticies: vertex_position_data,
            indicies: index_data,
        }
    }
}

impl<'a, 'b> Render<'a> for Cylinder<'a, 'b> {
    fn shader_kind() -> Kind {
        Kind::Cylinder
    }

    fn shader(&'a self) -> &'a Shader {
        self.shader
    }

    fn buffer_attributes(&self, gl: &WebGl2RenderingContext) {
        #![allow(clippy::cast_sign_loss)]

        let shader = self.shader();

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        Cylinder::buffer_f32_data(gl, &self.object.verticies[..], pos_attrib as u32, 3);
        Cylinder::buffer_u16_indices(gl, &self.object.indicies[..]);
    }

    fn render(&self, gl: &WebGl2RenderingContext, state: &State) {
        #![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

        let shader = self.shader();

        let base_vector = self.position_start.sub(&self.position_end);
        let height = base_vector.mag();
        let base_vector_norm = base_vector.scale(1.0 / height);

        let height_uni = shader.get_uniform_location(gl, "height");
        gl.uniform1f(height_uni.as_ref(), height);

        let radius_uni = shader.get_uniform_location(gl, "radius");
        gl.uniform1f(radius_uni.as_ref(), self.radius);

        let color_start_uni = shader.get_uniform_location(gl, "color_start");
        gl.uniform4f(
            color_start_uni.as_ref(),
            self.color_end[0],
            self.color_end[1],
            self.color_end[2],
            self.color_end[3],
        );

        let color_end_uni = shader.get_uniform_location(gl, "color_end");
        gl.uniform4f(
            color_end_uni.as_ref(),
            self.color_start[0],
            self.color_start[1],
            self.color_start[2],
            self.color_start[3],
        );

        let model_uni = shader.get_uniform_location(gl, "model");
        let mut model = Mat4::identity();

        let cross_prod = cross(&base_vector_norm, &[0.0, 0.0, 1.0]);
        let angle = -1.0 * base_vector_norm.dot(&[0.0, 0.0, 1.0]).acos();

        model.translate(&self.position_start.add(&self.position_end).scale(0.5));
        model.rotate(angle, &cross_prod);

        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &model);

        let view_uni = shader.get_uniform_location(gl, "view");
        let view = state.camera().view();
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &view);

        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &perspective);

        let camera_position_uni = shader.get_uniform_location(gl, "cameraPos");
        let camera_postion = &state.camera().get_eye_pos();
        gl.uniform3f(
            camera_position_uni.as_ref(),
            camera_postion[0],
            camera_postion[1],
            camera_postion[2],
        );

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            self.object.indicies.len() as i32,
            GL::UNSIGNED_SHORT,
            0,
        );
    }
}
