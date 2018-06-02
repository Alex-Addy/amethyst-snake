use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use snake::SnakeHead;

/// This system is responsible for moving the Snake according its velocity
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, SnakeHead>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (head, mut transforms, time): Self::SystemData) {
        for (head, transform) in (&head, &mut transforms).join() {
            transform.translation[0] += head.velocity.0 * time.delta_seconds();
            transform.translation[1] += head.velocity.1 * time.delta_seconds();
        }
    }
}
