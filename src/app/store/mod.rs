use std::ops::Deref;

mod mouse;
use self::mouse::*;

mod camera;
use self::camera::*;

use crate::molecule::*;

pub enum Msg {
    AdvanceClock(f32),
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    Zoom(f32),
    Key(String, bool),
}

pub struct State {
    clock: f32,
    camera: Camera,
    mouse: Mouse,
    molecules: Vec<Molecule>,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            camera: Camera::new(width, height),
            mouse: Mouse::default(),
            molecules: Vec::<Molecule>::new(),
        }
    }

    pub const fn camera(&self) -> &Camera {
        &self.camera
    }

    pub const fn molecules(&self) -> &Vec<Molecule> {
        &self.molecules
    }

    // The current time in milliseconds
    //pub fn clock(&self) -> f32 {
    //    self.clock
    //}

    pub fn add_molecule(&mut self, molecule: Molecule) {
        self.camera.set_target_position(&molecule.center());
        self.molecules.push(molecule);
    }

    pub fn msg(&mut self, msg: &Msg) {
        #![allow(clippy::cast_precision_loss)]
        match msg {
            Msg::AdvanceClock(dt) => {
                self.clock += dt;
            }
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
            }
            Msg::MouseUp => {
                self.mouse.set_pressed(false);
            }
            Msg::MouseMove(x, y) => {
                if !self.mouse.get_pressed() {
                    return;
                }

                let (old_x, old_y) = self.mouse.get_pos();

                let x_delta = old_x - x;
                let y_delta = y - old_y;

                self.camera.orbit_left_right(x_delta as f32 / 50.0);
                self.camera.orbit_up_down(y_delta as f32 / 50.0);

                self.mouse.set_pos(*x, *y);
            }
            Msg::Zoom(zoom) => {
                self.camera.zoom(*zoom);
            }
            Msg::Key(key, _state) => {
                match key.as_ref() {
                    "KeyW" => self.camera.pan_up_down(-0.1),
                    "KeyA" => self.camera.pan_left_right(0.1),
                    "KeyS" => self.camera.pan_up_down(0.1),
                    "KeyD" => self.camera.pan_left_right(-0.1),
                    "KeyQ" => self.camera.rotate_camera(0.1),
                    "KeyE" => self.camera.rotate_camera(-0.1),
                    _ => {},
                }
            }
        }
    }
}

pub struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &State {
        &self.0
    }
}

impl StateWrapper {
    pub fn msg(&mut self, msg: &Msg) {
        self.0.msg(msg);
    }

    pub fn add_molecule(&mut self, molecule: Molecule) {
        self.0.add_molecule(molecule);
    }
}

pub struct Store {
    pub state: StateWrapper,
}

impl Store {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            state: StateWrapper(State::new(width, height)),
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            _ => self.state.msg(msg),
        }
    }

    pub fn add_molecule(&mut self, molecule: Molecule) {
        self.state.add_molecule(molecule);
    }
}
