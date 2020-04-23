use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::*;

static BASIC_VS: &str = include_str!("./basic_vs.glsl");
static BASIC_FS: &str = include_str!("./basic_fs.glsl");

static SPHERE_VS: &str = include_str!("./sphere_vs.glsl");
static CYLINDER_VS: &str = include_str!("./cylinder_vs.glsl");

static LIGHTING_FS: &str = include_str!("./lighting_fs.glsl");

/// Identifiers for our different shaders
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Kind {
    Basic,
    Sphere,
    Cylinder,
}

/// Powers retrieving and using our shaders
pub struct System {
    programs: HashMap<Kind, Shader>,
    active_program: RefCell<Kind>,
}

impl System {
    /// Create a new `System`
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let mut programs = HashMap::new();

        let basic_shader =
            Shader::new(gl, BASIC_VS, BASIC_FS).unwrap();

        let sphere_shader =
            Shader::new(gl, SPHERE_VS, LIGHTING_FS).unwrap();

        let cylinder_shader =
            Shader::new(gl, CYLINDER_VS, LIGHTING_FS).unwrap();

        let active_program = RefCell::new(Kind::Basic);
        gl.use_program(Some(&basic_shader.program));

        programs.insert(Kind::Basic, basic_shader);
        programs.insert(Kind::Sphere, sphere_shader);
        programs.insert(Kind::Cylinder, cylinder_shader);

        Self {
            programs,
            active_program,
        }
    }

    /// Get one of our Shader's
    pub fn get_shader(&self, shader_kind: Kind) -> Option<&Shader> {
        self.programs.get(&shader_kind)
    }

    /// Use a shader program. We cache the last used shader program to avoid unnecessary
    /// calls to the GPU.
    pub fn use_program(&self, gl: &WebGl2RenderingContext, shader_kind: Kind) {
        if *self.active_program.borrow() == shader_kind {
            return;
        }

        gl.use_program(Some(&self.programs.get(&shader_kind).unwrap().program));
        *self.active_program.borrow_mut() = shader_kind;
    }
}

/// One per `Shader`
pub struct Shader {
    pub program: WebGlProgram,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
}

impl Shader {
    /// Create a new Shader program from a vertex and fragment shader
    fn new(
        gl: &WebGl2RenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Self, JsValue> {
        let vert_shader = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(gl, &vert_shader, &frag_shader)?;

        let uniforms = RefCell::new(HashMap::new());

        Ok(Self { program, uniforms })
    }

    /// Get the location of a uniform.
    /// If this is our first time retrieving it we will cache it so that for future retrievals
    /// we won't need to query the shader program.
    pub fn get_uniform_location(
        &self,
        gl: &WebGl2RenderingContext,
        uniform_name: &str,
    ) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();

        if uniforms.get(uniform_name).is_none() {
            if cfg!(debug_assertions) {
                uniforms.insert(
                    uniform_name.to_string(),
                    gl.get_uniform_location(&self.program, uniform_name)
                        .unwrap_or_else(|| panic!(r#"Uniform '{}' not found"#, uniform_name)),
                );
            } else {
                let uniform_location = match gl.get_uniform_location(&self.program, uniform_name) {
                    Some(s) => s,
                    None => std::process::abort(),
                };
                uniforms.insert(uniform_name.to_string(), uniform_location);
            }
        }

        Some(uniforms.get(uniform_name).expect("loc").clone())
    }
}

/// Create a shader program using the `WebGL` APIs
fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the `WebGL` APIs
fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);

    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
