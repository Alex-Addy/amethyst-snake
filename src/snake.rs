
pub struct Snake;

use amethyst::prelude::*;
use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::{Trans, GameData, State, StateData};
use amethyst::renderer::{KeyboardInput, WindowEvent, Event, VirtualKeyCode, Camera, Projection,
            Material, MaterialDefaults, MeshHandle, PosTex};

use {ARENA_HEIGHT, ARENA_WIDTH};

impl<'a, 'b> State<GameData<'a, 'b>> for Snake {
    fn on_start(&mut self, state: StateData<GameData>) {
        state.world.register::<SnakeHead>();
        state.world.register::<Food>();

        initialise_camera(state.world);
        initialise_snake(state.world);
        initialise_food(state.world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

///////////////////
// Game Entities //
///////////////////

#[derive(Debug)]
pub struct SnakeHead {
    pub size: f32,
    pub velocity: (f32, f32),
}

impl Component for SnakeHead {
    type Storage = DenseVecStorage<Self>;
}

impl SnakeHead {
    pub fn new() -> SnakeHead {
        use {SNAKE_VELOCITY, SNAKE_HEAD_SIZE};

        SnakeHead {
            size: SNAKE_HEAD_SIZE,
            velocity: (0.0, SNAKE_VELOCITY),
        }
    }
}

#[derive(Debug)]
pub struct Food {}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}

//////////////////////////////
// Initialisation Functions //
//////////////////////////////

/// Initialises game camera
fn initialise_camera(world: &mut World) {
    use amethyst::core::cgmath::{Matrix4, Vector3};
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}

/// Initialises snake placing head in the middle
fn initialise_snake(world: &mut World) {
    use {SNAKE_COLOR, SNAKE_HEAD_SIZE};

    let mut head_transform = Transform::default();

    let x = (ARENA_WIDTH - SNAKE_HEAD_SIZE) / 2.0;
    let y = (ARENA_HEIGHT - SNAKE_HEAD_SIZE) / 2.0;
    head_transform.translation = Vector3::new(x, y, 0.0);

    let mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, SNAKE_HEAD_SIZE, SNAKE_HEAD_SIZE));
    let material = create_color_material(world, SNAKE_COLOR);

    // create snake head entity
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(SnakeHead::new())
        .with(head_transform)
        .with(GlobalTransform::default())
        .build();
}

// Initialises food, placing it in the field
fn initialise_food(world: &mut World) {
    use {FOOD_SIZE, FOOD_COLOR};

    let mut transform = Transform::default();

    let x = (ARENA_WIDTH - FOOD_SIZE) / 1.5;
    let y = (ARENA_HEIGHT - FOOD_SIZE) / 1.5;
    transform.translation = Vector3::new(x, y, 0.0);

    let mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, FOOD_SIZE, FOOD_SIZE));
    let material = create_color_material(world, FOOD_COLOR);

    // create snake head entity
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Food{})
        .with(transform)
        .with(GlobalTransform::default())
        .build();
}

//////////////////////
// Helper Functions //
//////////////////////

fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: [left, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [right, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
    ]
}

fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified color.
fn create_color_material(world: &World, color: [f32; 4]) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(color.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

