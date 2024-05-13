use std::{fs::read_to_string, path::Path};

use suoi_game::{chess_board::ChessBoard, player::Player};

use suoi_phsh::{r#box::Box, collision_shape::CollisionShape, ray::Ray};
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
    let cube_model = Model::from(Obj::import(Path::new("assets/models/cube.obj")).unwrap());

    board.start(&models);
    player.start(&mut camera);

    unsafe { Renderer::init() };

    let cube = Box {
        position: Vector3::up() * 1.0,
        size: Vector3::one() * 0.5,
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

                // shader.set_uniform("model", &board.transform.mat().transposition());
                shader.set_uniform("model", &Matrix4::identity());
                board.model.draw();
                
                for piece in board.pieces() {
                    shader.set_uniform("model", &piece.transform.mat().transposition());
                    piece.model.draw();
                }

                let model_matrix = &(&Matrix4::translate(cube.position) * &Matrix4::uniform_scale(cube.size.x)).transposition();
                
                shader.set_uniform("model", model_matrix);
                cube_model.draw();
            });
        }

        // let inv = &Matrix4::translate(Vector3::fwd() * -15.0) * &Matrix4::uniform_scale(150.0);
        let inv = &view_matrix * &camera.inverse_projection_matrix(&screen);

        let pos = camera.transform.position();
        let dir = &inv * Vector3 {
            x: mouse.ndc(&screen).x,
            y: mouse.ndc(&screen).y,
            z: -1.0,
        };

        // println!("{} {}", pos, dir);
        
        let ray = Ray::point_dir(pos, dir);
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
