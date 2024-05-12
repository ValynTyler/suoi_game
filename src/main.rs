use std::{fs::read_to_string, path::Path};

use suoi_game::{chess_board::ChessBoard, player::Player};

use suoi_rwin::{
    shader::ShaderStage, Camera, Context, EventHandler, GLFWContext, GraphicsObject, Model, Mouse,
    Renderer, Screen, ShaderStageType, Time,
};
use suoi_simp::{obj::Obj, Resource};
use suoi_types::{Color, Matrix, Matrix4, Vector3};

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

    unsafe { Renderer::init() };

    while context.running() {
        context.window_mut().swap_buffers();
        unsafe {
            Renderer::clear_screen(CLEAR_COLOR);
            shader.with(|| {
                shader.set_uniform("texture1", 1);

                // set uniform matrices
                shader.set_uniform(
                    "view",
                    Matrix4::look_at_dir(
                        camera.transform.position(),
                        -camera.transform.position(),
                        Vector3::up(),
                    ),
                );
                shader.set_uniform(
                    "projection",
                    camera.projection_matrix(&screen).transposition(),
                );

                shader.set_uniform("model", board.transform.mat().transposition());
                board.model.draw();

                for piece in board.pieces() {
                    shader.set_uniform("model", piece.transform.mat().transposition());
                    piece.model.draw();
                }
            });
        }

        // poll systems
        time.poll(&context);
        mouse.poll(&context);
        event_handler.poll_events(&mut context, &mut screen, &mut mouse);

        // update
        player.update(&mut context, time.delta(), &mut mouse, &mut camera);
    }
}
