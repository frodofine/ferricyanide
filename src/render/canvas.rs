use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::app::App;
use crate::app::Msg;

pub fn create_webgl_context(
    app: &Rc<App>,
    app_div_id: &str,
    width: u32,
    height: u32,
) -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = init_canvas(app, app_div_id, width, height)?;

    let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

fn init_canvas(
    app: &Rc<App>,
    app_div_id: &str,
    width: u32,
    height: u32,
) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(width);
    canvas.set_height(height);

    canvas.set_tab_index(1);

    attach_keypress_handler(&canvas, Rc::clone(app))?;

    attach_mouse_down_handler(&canvas, Rc::clone(app))?;
    attach_mouse_up_handler(&canvas, Rc::clone(app))?;
    attach_mouse_move_handler(&canvas, Rc::clone(app))?;
    attach_mouse_wheel_handler(&canvas, Rc::clone(app))?;

    attach_touch_start_handler(&canvas, Rc::clone(app))?;
    attach_touch_move_handler(&canvas, Rc::clone(app))?;
    attach_touch_end_handler(&canvas, Rc::clone(app))?;

    let app_div: HtmlElement = if let Some(container) = document.get_element_by_id(app_div_id) {
        container.dyn_into()?
    } else {
        let app_div = document.create_element("div")?;
        app_div.set_id(app_div_id);
        app_div.dyn_into()?
    };

    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseDown(x, y));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::MouseEvent| {
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseUp);
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseMove(x, y));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    #![allow(clippy::cast_possible_truncation)]

    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();

        let zoom_amount = event.delta_y() / 500.;

        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::Zoom(zoom_amount as f32));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_start_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseDown(x, y));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_move_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        event.prevent_default();
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseMove(x, y));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_end_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::TouchEvent| {
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::MouseUp);
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("touchend", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_keypress_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::KeyboardEvent| {
        if let Ok(mut store) = app.store.try_borrow_mut() {
            store.msg(&Msg::Key(event.code(), true));
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback_and_bool(
        "keydown",
        handler.as_ref().unchecked_ref(),
        true,
    )?;

    handler.forget();

    Ok(())
}
