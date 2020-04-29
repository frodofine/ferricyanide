use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod app;
mod molecule;
mod render;

#[wasm_bindgen]
pub struct FerricyanideDisplay {
    app: Rc<app::App>,
    gl: Rc<WebGl2RenderingContext>,
    renderer: render::WebRenderer,
}

#[wasm_bindgen]
impl FerricyanideDisplay {
    /// Create a new web client
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new(app_div_id: &str, width: u32, height: u32) -> Self {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let app = Rc::new(app::App::new(width, height));

        let gl =
            Rc::new(render::canvas::create_webgl_context(&app, app_div_id, width, height).unwrap());

        let renderer = render::WebRenderer::new(&gl);

        Self { app, gl, renderer }
    }

    /// Start our WebGL and initialize the molecules to the value in contents
    pub fn add_molecule(&self, contents: Vec<u8>, format: &str) -> Result<(), JsValue> {
        use molecule::Molecule;

        let s =
            String::from_utf8(contents).expect("Found invalid UTF-8 character in Molecule file");
        if let Ok(molecule) = Molecule::from_string_with_format(&s, format) {
            self.app.store.borrow_mut().add_molecule(molecule);
        }
        Ok(())
    }

    /// Update our simulation
    pub fn update_time(&self, dt: f32) {
        self.app.store.borrow_mut().msg(&app::Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&mut self) -> Result<(), JsValue> {
        self.renderer
            .render(&self.gl, &self.app.store.borrow().state)
    }
}

/// # Errors
///
/// This function cannot fail
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
