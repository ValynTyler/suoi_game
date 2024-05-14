use std::{fs::read_to_string, path::Path};

use suoi_game::{player::Player, Matrix4, Vector3};

use suoi_rwin::{
    shader::ShaderStage, Camera, ClippingPlanes, Context, EventHandler, GLFWContext,
    GraphicsObject, Model, Mouse, Projection, Renderer, Screen, ShaderStageType, Time,
};
use suoi_simp::{obj::Obj, Resource};
use suoi_types::{Color, Matrix};

const CLEAR_COLOR: Color = Color::rgb(31, 31, 31);

struct UICanvas {
    width: u32,
    height: u32,
}

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

    // UI
    let canvas = UICanvas {
        width: screen.width(),
        height: screen.height(),
    };

    let mut ui_cam = Camera::new(
        Projection::new(
            suoi_rwin::ProjectionType::Ortho(canvas.width, canvas.height),
            ClippingPlanes::new(0.001, 1000.0),
        ),
        Default::default(),
    );
    ui_cam.transform.translate(Vector3::fwd() * 1.0);

    let ui_vert = &read_to_string("assets/shaders/unlit2d.vert").unwrap();
    let ui_frag = &read_to_string("assets/shaders/unlit2d.frag").unwrap();

    let ui_shader = unsafe {
        suoi_rwin::Shader::compile(
            ShaderStage::compile(ui_vert, ShaderStageType::Vertex).unwrap(),
            ShaderStage::compile(ui_frag, ShaderStageType::Fragment).unwrap(),
        )
    }
    .unwrap();

    let quad = Model::from(Obj::import(Path::new("assets/models/quad.obj")).expect("IMPORT_ERROR"));

    unsafe { Renderer::init() };

    while context.running() {
        context.window_mut().swap_buffers();
        unsafe {
            Renderer::clear_screen(CLEAR_COLOR);
            shader.with(|| {
                shader.set_uniform("texture1", 1);

                // set uniform matrices
                shader.set_uniform("view", &camera.view_matrix());
                shader.set_uniform(
                    "projection",
                    &camera.projection_matrix(&screen).transpose(),
                    &camera.projection_matrix(&screen).transpose(),
                );

                shader.set_uniform("model", &Matrix4::identity());
                model.draw();
            });

            // UI
            ui_shader.with(|| {
                ui_shader.set_uniform("texture1", 1);

                // set uniform matrices
                ui_shader.set_uniform("view", &ui_cam.view_matrix());
                ui_shader.set_uniform(
                    "projection",
                    &ui_cam.projection_matrix(&screen).transpose(),
                );

                ui_shader.set_uniform("model", &Matrix4::uniform_scale(5.0));
                quad.draw();
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
