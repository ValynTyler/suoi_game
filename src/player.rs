use suoi_rwin::{Camera, Context, Key, Keyboard, Mouse};
use suoi_types::{Deg, Quaternion, Vector2, Vector3};

pub struct Player {
    sensitivity: f32,

    pitch: f32,
    yaw: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sensitivity: 5.0,
            pitch: Default::default(),
            yaw: Default::default(),
        }
    }
}

impl Player {
    pub fn start(&self, camera: &mut Camera) {
        camera
            .transform
            .translate(Vector3::fwd() * -10. + Vector3::up() * 3.);
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

        self.turn_camera(delta_time, mouse, camera);
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

        let x_dist =
            f32::sin((self.yaw).to_radians());
            
        let y_dist =
            f32::cos((self.pitch + 90.0).to_radians());
            
        let z_dist =
            f32::cos((self.yaw).to_radians());
            // f32::sin((self.pitch + 90.0).to_radians());

        camera.transform.set_position((
                // Vector3::up() * y_dist +
                Vector3::fwd() * z_dist +
                Vector3::right() * x_dist +
                Vector3::up() * 0.5
            ) * 10.0
        );
    }
}
