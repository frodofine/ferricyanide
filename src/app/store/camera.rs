use webgl_matrix::{Mat4, ProjectionMatrix, Vec3, Vector};

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

fn look_at_rh(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {
    let z = eye.sub(target);
    let zn = z.scale(1.0 / z.mag());

    let x = cross(up, &zn);
    let xn = x.scale(1.0 / x.mag());

    let yn = cross(&zn, &xn);

    [
        xn[0],
        yn[0],
        zn[0],
        0.0,
        xn[1],
        yn[1],
        zn[1],
        0.0,
        xn[2],
        yn[2],
        zn[2],
        0.0,
        -xn.dot(eye),
        -yn.dot(eye),
        -zn.dot(eye),
        1.0,
    ]
}

pub struct Camera {
    projection: Mat4,
    target_position: Vec3,
    orbit_radius: f32,
    left_right_radians: f32,
    up_down_radians: f32,
    camera_angle: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        #![allow(clippy::cast_precision_loss)]
        let aspect_ratio = (width as f32) / (height as f32);
        Self {
            projection: Mat4::create_perspective(
                60.0 * std::f32::consts::PI / 180.0,
                aspect_ratio,
                0.1,
                1000.0,
            ),
            orbit_radius: 9.0,
            target_position: [0.0, 0.0, 0.0],
            left_right_radians: 0.0,
            up_down_radians: 0.0,
            camera_angle: 0.0,
        }
    }

    pub fn set_target_position(&mut self, new_target: &Vec3) {
        self.target_position = *new_target;
    }

    fn is_y_inverted(&self) -> bool {
        use std::f32::consts::PI;

        match self.up_down_radians {
            x if (-PI / 2.0..PI / 2.0).contains(&x) => false,
            x if (3.0 * PI / 2.0..2.0 * PI).contains(&x) => false,
            _ => true,
        }
    }

    pub fn view(&self) -> [f32; 16] {
        let new_eye = self.get_eye_pos();

        let direction = if self.is_y_inverted() {
            [self.camera_angle.sin(), -(self.camera_angle.cos()), 0.0]
        } else {
            [self.camera_angle.sin(), self.camera_angle.cos(), 0.0]
        };

        look_at_rh(
            &new_eye,
            &self.target_position,
            &direction.scale(1.0 / direction.mag()),
        )
    }

    pub fn get_eye_pos(&self) -> Vec3 {
        let yaw = self.left_right_radians;
        let pitch = self.up_down_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos();
        let eye_y = self.orbit_radius * pitch.sin();
        let eye_z = self.orbit_radius * yaw.cos() * pitch.cos();

        self.target_position.add(&[eye_x, eye_y, eye_z])
    }

    pub const fn projection(&self) -> [f32; 16] {
        self.projection
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        if self.is_y_inverted() {
            self.left_right_radians -= delta;
        } else {
            self.left_right_radians += delta;
        }
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        use std::f32::consts::PI;
        self.up_down_radians += delta;

        if self.up_down_radians > PI * 2.0 {
            self.up_down_radians -= PI * 2.0;
        }

        if self.up_down_radians < -PI * 2.0 {
            self.up_down_radians += PI * 2.0;
        }
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.orbit_radius += zoom;

        self.orbit_radius = self.orbit_radius.max(0.2);
    }

    pub fn pan_left_right(&mut self, delta: f32) {
        self.target_position[0] += delta;
    }

    pub fn pan_up_down(&mut self, delta: f32) {
        self.target_position[1] += delta;
    }

    pub fn rotate_camera(&mut self, delta: f32) {
        self.camera_angle += delta;
    }
}
