use amethyst::ecs::prelude::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use snake::SnakeHead;
use SNAKE_VELOCITY;

/// This system is responsible for handling user inputs to change the Snake's velocity
pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        WriteStorage<'s, SnakeHead>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut heads, input): Self::SystemData) {
        let vert = input.axis_value("move_y").unwrap_or(0.0);
        let horz = input.axis_value("move_x").unwrap_or(0.0);

        for mut head in (&mut heads).join() {
            head.velocity = if vert > 0.0 {
                (0.0, SNAKE_VELOCITY)
            } else if vert < 0.0 {
                (0.0, -SNAKE_VELOCITY)
            } else if horz > 0.0 {
                (SNAKE_VELOCITY, 0.0)
            } else if horz < 0.0 {
                (-SNAKE_VELOCITY, 0.0)
            } else {
                head.velocity
            };
        }
    }
}
