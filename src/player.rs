use suoi_rwin::{Camera, Context, Key, Keyboard, Mouse};
use suoi_types::{Deg, Quaternion, Rad, Vector2, Vector3};

pub struct Player {
    sensitivity: f32,

    pitch: f32,
    yaw: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sensitivity: 155.0,
            pitch: Default::default(),
            yaw: Default::default(),
        }
    }
}

impl Player {
    pub fn start(&mut self, camera: &mut Camera) {
        camera.transform.translate(Vector3::up() * 5.0 + Vector3::fwd() * -10.0);

        let target = camera.transform.position();
        let fwd = camera.transform.forward();

        let axis = fwd.cross(target).normalized();
        let phi = target.angle(fwd);
        
        camera.transform.set_rotation(
            Quaternion::axis_angle(axis, Rad(phi))
        );
    }

    pub fn update(
        &mut self,
        context: &mut Context,
        delta_time: f32,
        mouse: &Mouse,
        camera: &mut Camera,
    ) {
        if Keyboard::get_key(Key::Esc, &context).is_pressed() {
            context.close()
        }

        if mouse.left_button().is_pressed() {
            self.turn_camera(delta_time, mouse, camera);
        }
        self.move_self(context, delta_time, camera);
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
