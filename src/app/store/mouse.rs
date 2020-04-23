#[derive(Default)]
pub struct Mouse {
    pressed: bool,
    x: i32,
    y: i32,
}

impl Mouse {
    pub const fn get_pressed(&self) -> bool {
        self.pressed
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub const fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
