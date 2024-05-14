use std::{fs::read_to_string, path::Path};

use suoi_game::{
    chess_board::ChessBoard,
    player::Player,
};

#[allow(unused_imports)]
use suoi_phsh::{collision_shape::CollisionShape, bounding_box::BoundingBox, ray::Ray};
use suoi_rwin::{
    shader::ShaderStage, Camera, Context, EventHandler, GLFWContext, GraphicsObject, Model, Mouse,
    Renderer, Screen, ShaderStageType, Time,
};
use suoi_simp::{obj::Obj, Resource};
use suoi_types::*;

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

    let _cube_model = Model::from(Obj::import(Path::new("assets/models/cube.obj")).unwrap());

    let square_size = Vector3 {
        x: 0.4,
        y: 0.4,
        z: 0.4,
    };

    let board = ChessBoard::new();

    player.start(&mut camera);

    unsafe { Renderer::init() };

    let _square_size = Vector3::one() * 0.4;

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
                    &camera.projection_matrix(&screen).transpose(),
                );

                shader.set_uniform("model", &Matrix4::identity());
                models[0].draw();

                for i in 0..8 {
                    for j in 0..8 {
                        shader.set_uniform(
                            "model",
                            &Matrix4::translate(Vector3::new(j as f32 - 4.0, 0.0, i as f32 - 3.0))
                                .transpose(),
                        );
                        let idx: u8 = (board.get(i, j)).into();
                        if idx as usize != 0 {
                            models[idx as usize].draw();
                        }

                        //

                        let model_matrix = (&Matrix4::translate(Vector3 {
                            x: i as f32 - 3.5,
                            y: -0.3,
                            z: j as f32 - 3.5,
                        }) * &Matrix4::scale(square_size))
                            .transpose();

                        shader.set_uniform("model", &model_matrix);
                        // cube_model.draw();
                    }
                }
            });
        }

        let inv = &view_matrix * &camera.inverse_projection_matrix(&screen);

        let pos = camera.transform.position();
        let dir = &inv
            * Vector3 {
                x: mouse.ndc(&screen).x,
                y: mouse.ndc(&screen).y,
                z: -1.0,
            };

        let _ray = Ray::point_dir(pos, dir);

        // for i in 0..8 {
        //     for j in 0..8 {
        //         let box_pos = Vector3 {
        //             x: i as f32 - 3.5,
        //             y: -0.3,
        //             z: j as f32 - 3.5,
        //         };

        //         let raycast = Box {
        //             position: box_pos,
        //             size: square_size,
        //         }
        //         .raycast(&ray);

        //         match raycast {
        //             Raycast::Miss => (),
        //             Raycast::Hit(hit) => println!("{:?}", (i, j)),
        //         }
        //     }
        // }

        // let mut piece = None;
        // let mut last_piece = None;

        // if mouse.left_button().is_pressed() {
        //     println!("{:?}", piece_pos);
        //     for p in board.pieces_mut() {
        //         if Some(p.position.clone()) == piece_pos {
        //             piece = Some(p);
        //         }

        //         if Some(p.position.clone()) == last_piece_pos {
        //             last_piece = Some(p);
        //         }
        //     }
        // }

        // if piece.is_some() && last_piece.is_some() {
        //     last_piece.unwrap().position = piece_pos.unwrap();
        // }

        // last_piece_pos = piece_pos;

        // poll systems
        time.poll(&context);
        mouse.poll(&context);
        mouse.poll(&context);
        event_handler.poll_events(&mut context, &mut screen, &mut mouse);

        // update
        player.update(&mut context, time.delta(), &mut mouse, &mut camera);
    }
}
