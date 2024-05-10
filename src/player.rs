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
    pub fn start(&mut self, camera: &mut Camera) {
        camera.transform.translate(Vector3::up() * 2.0)
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
    
        self.move_self(context, delta_time, camera);
        self.turn_camera(delta_time, mouse, camera);
    }

    fn move_self(&mut self, context: &mut Context, delta_time: f32, camera: &mut Camera) {
        let move_axes = Vector2 {
            x: Keyboard::input_axis(&context, Key::A, Key::D),
            y: Keyboard::input_axis(&context, Key::S, Key::W),
        };

        let fwd = camera.transform.forward();
        let right = camera.transform.right();

        let speed = 5.0;

        camera
            .transform
            .translate((-fwd * move_axes.y + right * move_axes.x) * speed * delta_time);
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

        camera.transform.set_rotation(
            Quaternion::axis_angle(Vector3::up(), Deg(self.yaw)) *
            Quaternion::axis_angle(Vector3::right(), Deg(self.pitch)),
        );
    }
}
