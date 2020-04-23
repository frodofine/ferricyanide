use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use webgl_matrix::{Mat4, Matrix};

use crate::app::State;
use crate::render::shape::Render;
use crate::render::shader::Shader;
use crate::render::shader::Kind;

pub struct VBO {
    pub verticies: Vec<f32>,
    pub indicies: Vec<u16>,
}

pub struct Sphere<'a,'b> {
    pub object: &'b VBO,
    pub shader: &'a Shader,
    pub radius: f32,
    pub color: [f32;4],
    pub position: [f32;3],
}

impl VBO {
    pub fn new(lat_bands: u16, long_bands: u16) -> Self {
        use std::f32::consts::PI;

        let mut vertex_position_data = Vec::<f32>::new();
        let mut index_data = Vec::<u16>::new();

        for lat_number in 0..lat_bands {
            let theta = f32::from(lat_number) * PI / f32::from(lat_bands);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
    
            for long_number in 0..long_bands {
                let phi = f32::from(long_number) * 2.0 * PI / f32::from(long_bands);
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
    
                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;
    
                vertex_position_data.push(x);
                vertex_position_data.push(y);
                vertex_position_data.push(z);
    
                let first = lat_number * long_bands + long_number;
                let second = first + long_bands;

                if second >= lat_bands * long_bands - 1 {
                    index_data.push(first);
                    index_data.push(lat_bands * long_bands);
                    index_data.push(first + 1);
    
                    index_data.push(lat_bands * long_bands);
                    index_data.push(first);
                    index_data.push(first + 1);
    
                    continue;
                }

                index_data.push(first);
                index_data.push(second);
                index_data.push(first + 1);
    
                index_data.push(second);
                index_data.push(second + 1);
                index_data.push(first + 1);
            }
        }

        vertex_position_data.push(0.0);
        vertex_position_data.push(-1.0);
        vertex_position_data.push(0.0);

        Self {
            verticies: vertex_position_data,
            indicies: index_data,
        }
    }
}

impl<'a,'b> Render<'a> for Sphere<'a,'b> {
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
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        Sphere::buffer_f32_data(gl, &self.object.verticies[..], pos_attrib as u32, 3);
        Sphere::buffer_u16_indices(gl, &self.object.indicies[..]);
    }

    fn render(&self, gl: &WebGl2RenderingContext, state: &State) {
        #![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

        let shader = self.shader();

        let radius_uni = shader.get_uniform_location(gl, "radius");
        gl.uniform1f(radius_uni.as_ref(), self.radius);

        let color_uni = shader.get_uniform_location(gl, "color");
        gl.uniform4f(color_uni.as_ref(), self.color[0], self.color[1], self.color[2], self.color[3]);

        let model_uni = shader.get_uniform_location(gl, "model");
        let mut model = Mat4::identity();
        model.translate(&self.position);
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &model);
        
        let view_uni = shader.get_uniform_location(gl, "view");
        let view = state.camera().view();
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &view);

        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &perspective);

        let camera_position_uni = shader.get_uniform_location(gl, "cameraPos");
        let camera_postion = &state.camera().get_eye_pos();
        gl.uniform3f(camera_position_uni.as_ref(), camera_postion[0], camera_postion[1], camera_postion[2]);

        gl.draw_elements_with_i32(GL::TRIANGLES, self.object.indicies.len() as i32, GL::UNSIGNED_SHORT, 0);
    }
}
