use std::{fs::read_to_string, path::Path};

use suoi_game::{chess_board::ChessBoard, player::Player};

use suoi_phsh::{collision_shape::CollisionShape, r#box::Box, ray::Ray};
use suoi_rwin::{
    shader::ShaderStage, Camera, ClippingPlanes, Context, EventHandler, GLFWContext,
    GraphicsObject, Model, Mouse, Projection, Renderer, Screen, ShaderStageType, Time,
};
use suoi_simp::{obj::Obj, Resource};
use suoi_types::*;

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
    let frag_data = &read_to_string("assets/shaders/basic.frag").unwrap();

    let shader = unsafe {
        suoi_rwin::Shader::compile(
            ShaderStage::compile(vert_data, ShaderStageType::Vertex).unwrap(),
            ShaderStage::compile(frag_data, ShaderStageType::Fragment).unwrap(),
        )
    }
    .unwrap();

    fn models() -> Result<Vec<Model>, suoi_simp::ImportError> {
        Ok(vec![
            Model::from(Obj::import(Path::new("assets/models/board.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/pawn.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/knight.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/bishop.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/rook.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/queen.obj"))?),
            Model::from(Obj::import(Path::new("assets/models/king.obj"))?),
        ])
    }
    let models = models().expect("IMPORT_ERROR");

    let mut board = ChessBoard::new(&models[0]);

    board.start(&models);
    player.start(&mut camera);

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

    let cube = Box {
        position: Vector3::zero(),
        size: Vector3::one(),
    };

    while context.running() {
        context.window_mut().swap_buffers();

        let view_matrix = Matrix4::look_at_dir(
            camera.transform.position(),
            -camera.transform.position(),
            Vector3::up(),
        );

        unsafe {
            Renderer::clear_screen(CLEAR_COLOR);
            shader.with(|| {
                shader.set_uniform("texture1", 1);

                // set uniform matrices
                shader.set_uniform("view", &view_matrix);
                shader.set_uniform(
                    "projection",
                    &camera.projection_matrix(&screen).transposition(),
                );

                shader.set_uniform("model", &board.transform.mat().transposition());
                board.model.draw();

                for piece in board.pieces() {
                    shader.set_uniform("model", &piece.transform.mat().transposition());
                    piece.model.draw();
                }
            });

            // UI
            ui_shader.with(|| {
                ui_shader.set_uniform("texture1", 1);

                // set uniform matrices
                ui_shader.set_uniform("view", &ui_cam.view_matrix());
                ui_shader.set_uniform(
                    "projection",
                    &ui_cam.projection_matrix(&screen).transposition(),
                );

                ui_shader.set_uniform("model", &Matrix4::uniform_scale(5.0));
                quad.draw();
            });
        }

        let fwd = Vector3 {
            x: view_matrix.get(0, 2).unwrap(),
            y: view_matrix.get(1, 2).unwrap(),
            z: view_matrix.get(2, 2).unwrap(),
        };
        let ray = Ray::point_dir(camera.transform.position(), -fwd);
        println!("{:?}", cube.raycast(&ray));

        // poll systems
        time.poll(&context);
        mouse.poll(&context);
        mouse.poll(&context);
        event_handler.poll_events(&mut context, &mut screen, &mut mouse);

        // update
        player.update(&mut context, time.delta(), &mut mouse, &mut camera);
    }
}
