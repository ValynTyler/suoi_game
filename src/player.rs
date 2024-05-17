use suoi_rwin::{Camera, Context, Key, Keyboard, Mouse};
use suoi_types::{Quaternion, Rad, Vector3};

pub struct Player {
    sensitivity: f32,

    pitch: f32,
    yaw: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sensitivity: 150.0,
            pitch: Default::default(),
            yaw: Default::default(),
        }
    }
}

impl Player {
    pub fn start(&mut self, camera: &mut Camera) {
        self.turn_camera(0.01, &Mouse::default(), camera);
    }

    pub fn update(
        &mut self,
        context: &mut Context,
        delta_time: f32,
        mouse: &Mouse,
        camera: &mut Camera,
    ) {
        if Keyboard::get_key(Key::Esc, context).is_pressed() {
            context.close()
        }

        if mouse.right_button().is_pressed() {
            context.disable_cursor();
            self.turn_camera(delta_time, mouse, camera);
        } else {
            context.enable_cursor();
        }
    }

    #[rustfmt::skip]
    fn turn_camera(&mut self, delta_time: f32, mouse: &Mouse, camera: &mut Camera) {
        self.yaw -= mouse.delta().x * self.sensitivity * delta_time;
        self.pitch -= mouse.delta().y * self.sensitivity * delta_time;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let pos = Vector3::fwd().rotate(
            Quaternion::axis_angle(Vector3::up(), Rad(self.yaw))
        );

        camera.transform.set_position(
            pos * 10.0 + Vector3::up() * 4.0
        );
    }
}
