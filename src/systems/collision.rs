use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, System, WriteStorage};
use snake::SnakeHead;

/// This system is responsible for detecting collisions between the snake head,
/// its tail, and the edge of the arena.
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (WriteStorage<'s, SnakeHead>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut heads, mut transforms): Self::SystemData) {
        use {ARENA_HEIGHT, ARENA_WIDTH, SNAKE_HEAD_SIZE};

        // Check whether the head has collided with the edge of the arena
        for (head, mut transform) in (&mut heads, &mut transforms).join() {
            let head_x = transform.translation[0];
            let head_y = transform.translation[1];

            // TODO maybe use these guys to make the arena wrap around
            let touched_edge = if head_y <= 0.0 && head.velocity.1 < 0.0 {
                true
            } else if head_y >= ARENA_HEIGHT - head.size && head.velocity.1 > 0.0 {
                true
            } else if head_x <= 0.0 && head.velocity.0 < 0.0 {
                true
            } else if head_x >= ARENA_WIDTH - head.size && head.velocity.0 > 0.0 {
                true
            } else {
                false
            };

            if touched_edge {
                transform.translation[0] = (ARENA_WIDTH - SNAKE_HEAD_SIZE) / 2.0;
                transform.translation[1] = (ARENA_HEIGHT - SNAKE_HEAD_SIZE) / 2.0;
            }
        }
    }
}
