use std::{fs::read_to_string, path::Path};

use suoi_game::{player::Player, Angle, Deg, Quaternion, Rad, Vector, Vector3};

use suoi_rwin::{
    shader::ShaderStage, Camera, Context, EventHandler, GLFWContext, GraphicsObject, Model, Mouse,
    Renderer, Screen, ShaderStageType, Time,
};
use suoi_simp::{obj::Obj, Resource};
use suoi_types::{Color, Matrix};

const CLEAR_COLOR: Color = Color::rgb(31, 31, 31);

fn main() {
    let mut camera = Camera::default();
    let mut player = Player::default();

    let mut screen = Screen::new(800, 480);
    let mut context = Context::init(&screen);
    let mut event_handler = EventHandler::default();

    let mut time = Time::default();
    let mut mouse = Mouse::default();

    let vert_data = &read_to_string("assets/shaders/basic.vert").unwrap();
    let frag_data = &read_to_string("assets/shaders/normal.frag").unwrap();

    let shader = unsafe {
        suoi_rwin::Shader::compile(
            ShaderStage::compile(vert_data, ShaderStageType::Vertex).unwrap(),
            ShaderStage::compile(frag_data, ShaderStageType::Fragment).unwrap(),
        )
    }
    .unwrap();

    let model =
        Model::from(Obj::import(Path::new("assets/models/scene.obj")).expect("IMPORT_ERROR"));

    let mut monke =
        Model::from(Obj::import(Path::new("assets/models/monke.obj")).expect("IMPORT_ERROR"));

    unsafe { Renderer::init() };

    while context.running() {
        
        let fwd = monke.transform.forward();

        let mut dir = camera.transform.position() - monke.transform.position();
        dir = dir.unit();

        let axis = fwd.cross(dir);
        let angle = f32::acos(fwd.dot(dir));

        let q = Quaternion::axis_angle(axis, Rad(angle));
        println!("{}", q);
        
        if angle > f32::to_radians(10.0) {
            // monke.transform.set_rotation(q * monke.transform.rotation());
            monke.transform.set_rotation(q);
        }


        //
        //
        //

        context.window_mut().swap_buffers();
        unsafe {
            Renderer::clear_screen(CLEAR_COLOR);
            shader.with(|| {
                shader.set_uniform("texture1", 1);

                // set uniform matrices
                shader.set_uniform("view", camera.view_matrix());
                shader.set_uniform(
                    "projection",
                    camera.projection_matrix(&screen).transposition(),
                );

                shader.set_uniform("model", model.model_matrix().transposition());
                model.draw();

                shader.set_uniform("model", monke.model_matrix().transposition());
                monke.draw();
            });
        }

        // poll systems
        time.poll(&context);
        mouse.poll(&context);
        event_handler.poll_events(&mut context, &mut screen, &mut mouse);

        // update
        player.update(&mut context, time.delta(), &mouse, &mut camera);
    }
}
