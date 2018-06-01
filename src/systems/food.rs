use snake::{SnakeHead, Food};
use rand::{thread_rng, Rng};
use amethyst::core::Transform;
use amethyst::ecs::prelude::{Join, System, ReadStorage, WriteStorage};

use {SNAKE_HEAD_SIZE, FOOD_SIZE, ARENA_HEIGHT, ARENA_WIDTH};

/// This system is responsible for managing the food. Removing it when eaten
/// and adding new food to the arena;
pub struct FoodSystem;

impl<'s> System<'s> for FoodSystem {
    type SystemData = (
        ReadStorage<'s, SnakeHead>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (heads, food, mut transforms): Self::SystemData) {
        let (mut head_x, mut head_y) = (0.0, 0.0);
        for (_head, transform) in (&heads, &transforms).join() {
            head_x = transform.translation[0];
            head_y = transform.translation[1];
        }

        for (_food, mut transform) in (&food, &mut transforms).join() {
            if rect_collide(head_x, head_y, SNAKE_HEAD_SIZE, SNAKE_HEAD_SIZE,
                            transform.translation[0], transform.translation[1], FOOD_SIZE, FOOD_SIZE) {
                let mut rng = thread_rng();
                transform.translation[0] = rng.gen_range(0.0, ARENA_WIDTH);
                transform.translation[1] = rng.gen_range(0.0, ARENA_HEIGHT);
            }
        }
    }
}

/// check for collision between two rectangles given their positions (x, y)
/// and sizes (h, w) -> (height, width)
fn rect_collide(r1_x: f32, r1_y: f32, r1_h: f32, r1_w: f32,
                r2_x: f32, r2_y: f32, r2_h: f32, r2_w: f32) -> bool {

    r1_x + r1_w >= r2_x && // r1 right edge past r2 left edge
    r1_x <= r2_x + r2_w && // r1 left edge past r2 right edge
    r1_y + r1_h >= r2_y && // r1 top edge past r2 bottom edge
    r1_y <= r2_y + r2_h    // r1 bottom edge past r2 top edge
}
