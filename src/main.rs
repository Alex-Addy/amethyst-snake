extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage};

mod snake;
use snake::Snake;

fn run() -> Result<(), amethyst::Error> {
    let display_path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
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
