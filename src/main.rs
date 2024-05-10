use std::{fs::read_to_string, path::Path};

use suoi_game::player::Player;

use suoi_rwin::{
    shader::ShaderStage, Camera, Context, EventHandler, GLFWContext, GraphicsObject, Model, Mouse, Renderer, Screen, ShaderStageType, Time
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

    let model_path = Path::new("assets/models/scene.obj");
    let model = Model::from(Obj::import(model_path).expect("IMPORT_ERROR"));

    unsafe { Renderer::init() };

    while context.running() {
        context.window_mut().swap_buffers();
        unsafe {
            Renderer::clear_screen(CLEAR_COLOR);
            shader.with(|| {
                shader.set_uniform("texture1", 1);

                // set uniform matrices
                shader.set_uniform("model", model.model_matrix().transposition());
                shader.set_uniform("view", camera.view_matrix());
                shader.set_uniform(
                    "projection",
                    camera.projection_matrix(&screen).transposition(),
                );

                model.draw();
            });
        }

        // poll systems
        time.poll(&context);
        mouse.poll_delta();
        event_handler.poll_events(&mut context, &mut screen, &mut mouse);

        // update
        player.update(&mut context, time.delta(), &mouse, &mut camera);
    }
}
