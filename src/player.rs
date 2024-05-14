use suoi_rwin::{Camera, Context, Key, Keyboard, Mouse};
use suoi_types::Vector3;

pub struct Player {
    sensitivity: f32,
    yaw: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sensitivity: 100.0,
            yaw: Default::default(),
        }
    }
}

impl Player {
    pub fn start(&self, camera: &mut Camera) {
        camera.transform.set_position((
                Vector3::fwd() * 1.0
                + Vector3::up() * 0.8
            ) * 8.0
        );
    }

    pub fn update(
        &mut self,
        context: &mut Context,
        delta_time: f32,
        mouse: &mut Mouse,
        camera: &mut Camera,
    ) {
        if Keyboard::get_key(Key::Esc, context).is_pressed() {
            context.close()
        }

        self.turn_camera(delta_time, mouse, camera, context);
    }

    #[rustfmt::skip]
    fn turn_camera(&mut self, delta_time: f32, mouse: &mut Mouse, camera: &mut Camera, ctx: &mut Context) {
        // poll to mitigate delta skips
        if !mouse.right_button().is_pressed() {
            mouse.poll(ctx);
        }

        self.yaw -= mouse.delta().x * self.sensitivity * delta_time;

        let x_dist =
            f32::sin((self.yaw).to_radians());
        
        let z_dist =
            f32::cos((self.yaw).to_radians());

        if mouse.right_button().is_pressed() {
            ctx.disable_cursor();
            camera.transform.set_position((
                Vector3::fwd() * z_dist +
                Vector3::right() * x_dist
                 + Vector3::up() * 0.8
            ) * 8.0
        );
        } else {
            // mouse.poll(ctx);
            ctx.enable_cursor();
        }
    }
}
