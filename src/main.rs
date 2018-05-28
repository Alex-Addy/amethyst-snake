extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};

mod snake;
use snake::Snake;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

const SNAKE_HEAD_SIZE: f32 = 15.0;
const SNAKE_VELOCITY: f32 = 50.0;

const SNAKE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn run() -> Result<(), amethyst::Error> {
    let display_path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&display_path);


    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOR, 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
    ;
    let mut game = Application::<GameData>::new("./", Snake, game_data)?;

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}
