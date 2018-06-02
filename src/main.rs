extern crate amethyst;
extern crate rand;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{Bindings, InputBundle};
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};

mod snake;
mod systems;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

const SNAKE_HEAD_SIZE: f32 = 5.0;
const SNAKE_VELOCITY: f32 = 20.0;

const FOOD_SIZE: f32 = 4.0;

const FOOD_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const SNAKE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn run() -> Result<(), amethyst::Error> {
    let display_path = "./resources/display_config.ron";
    let binding_path = "./resources/bindings_config.ron";

    let config = DisplayConfig::load(&display_path);

    let bindings =
        Bindings::load_no_fallback(binding_path).map_err(|e| amethyst::Error::Config(e))?;
    let input_bundle = InputBundle::<String, String>::new().with_bindings(bindings);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOR, 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &[])
        .with(systems::ControlSystem, "control_system", &[])
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["movement_system"],
        )
        .with(systems::FoodSystem, "food_system", &["movement_system"]);
    let mut game = Application::<GameData>::new("./", snake::Snake, game_data)?;

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}
