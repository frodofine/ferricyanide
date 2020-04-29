use std::cell::RefCell;
use std::collections::HashMap;

use crate::app::State;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub mod canvas;

mod shader;

mod shape;
use shape::Render;

use shape::cylinder;
use shape::sphere;

pub struct WebRenderer {
    shader_sys: shader::System,
    sphere_object: sphere::VBO,
    cylinder_object: cylinder::VBO,
    vaos: RefCell<HashMap<String, web_sys::WebGlVertexArrayObject>>,
}

impl WebRenderer {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let shader_sys = shader::System::new(gl);

        let sphere_object = sphere::VBO::new(20, 20);

        let cylinder_object = cylinder::VBO::new(30);

        Self {
            shader_sys,
            sphere_object,
            cylinder_object,
            vaos: RefCell::new(HashMap::new()),
        }
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, state: &State) -> Result<(), JsValue> {
        use shape::cylinder::Cylinder;
        use shape::sphere::Sphere;
        use shape::triangle::Triangle;

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Display a 'debug' triangle to orient the size of a 1.0 step and location of the origin.
        let verticies: [f32; 9] = [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];

        let colors: [f32; 12] = [1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];

        let my_triangle = Triangle {
            verticies,
            colors,
            shader: self.shader_sys.get_shader(shader::Kind::Basic).unwrap(),
        };

        self.shader_sys.use_program(gl, shader::Kind::Basic);
        self.prepare_for_render(gl, &my_triangle, "triangle");
        my_triangle.render(gl, state);

        let molecules = state.molecules();

        self.shader_sys.use_program(gl, shader::Kind::Cylinder);
        let mut new_cylinder = Cylinder {
            object: &self.cylinder_object,
            shader: self.shader_sys.get_shader(shader::Kind::Cylinder).unwrap(),
            color_start: [1.0, 0.0, 0.0, 1.0],
            color_end: [1.0, 0.0, 0.0, 1.0],
            radius: 0.05,
            position_start: [0.0, 0.0, 0.0],
            position_end: [0.0, 0.0, 0.0],
        };
        self.prepare_for_render(gl, &new_cylinder, "cylinder");

        for molecule in molecules {
            for bond in &molecule.bonds {
                let atom1 = &bond[0];
                let atom2 = &bond[1];

                new_cylinder.position_start = atom1.position;
                new_cylinder.position_end = atom2.position;

                new_cylinder.color_start = atom1.element.cpk_color();
                new_cylinder.color_end = atom2.element.cpk_color();

                new_cylinder.radius = atom1
                    .element
                    .covalent_radius()
                    .min(atom2.element.covalent_radius())
                    / 8.0;

                if new_cylinder.radius < 0.05 {
                    new_cylinder.radius = 0.05;
                }

                new_cylinder.render(gl, state);
            }
        }

        self.shader_sys.use_program(gl, shader::Kind::Sphere);
        let mut new_sphere = Sphere {
            object: &self.sphere_object,
            shader: self.shader_sys.get_shader(shader::Kind::Sphere).unwrap(),
            radius: 1.0,
            color: [0.0, 0.0, 0.0, 0.0],
            position: [0.0, 0.0, 0.0],
        };
        self.prepare_for_render(gl, &new_sphere, "sphere");

        for molecule in molecules {
            for atom in &molecule.atoms {
                new_sphere.color = atom.element.cpk_color();
                new_sphere.position = atom.position;
                new_sphere.radius = atom.element.covalent_radius() * 0.5;

                new_sphere.render(gl, state);
            }
        }

        Ok(())
    }

    pub fn prepare_for_render<'a>(
        &self,
        gl: &WebGl2RenderingContext,
        renderable: &impl Render<'a>,
        key: &str,
    ) {
        let mut vaos = self.vaos.borrow_mut();
        let value = vaos.get(key);
        match value {
            None => {
                let vao = gl.create_vertex_array().unwrap();
                gl.bind_vertex_array(Some(&vao));
                renderable.buffer_attributes(gl);
                vaos.insert(key.to_string(), vao);
            }
            Some(vao) => {
                gl.bind_vertex_array(Some(vao));
            }
        }
    }
}
