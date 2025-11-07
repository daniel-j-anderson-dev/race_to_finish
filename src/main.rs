use macroquad::{
    miniquad::date,
    prelude::*,
    rand::{gen_range, srand},
};

const PLAYER_COLOR: Color = GREEN;
const PLAYER_SCALE: f32 = 0.1;

#[macroquad::main("race to finish")]
async fn main() {
    srand(date::now() as _);

    // initialize game state
    let mut player = spawn_player();

    // main game loop
    loop {
        // draw
        clear_background(WHITE);
        draw_player(player);

        // tell macroquad our logic for this frame is done
        next_frame().await;
    }
}

/// Returns the dimensions of the screen in pixels as a [Vec2]
fn screen_dimensions() -> Vec2 {
    vec2(screen_width(), screen_height())
}
/// Returns the origin of the screen in pixels as a [Vec2]
fn screen_origin() -> Vec2 {
    screen_dimensions() / 2.0
}

/// Generates a random point that is on screen
fn random_point(low: Vec2, high: Vec2) -> Vec2 {
    vec2(gen_range(low.x, high.x), gen_range(low.y, high.y))
}

/// Returns a rectangle at a random position whose size is proportional to the smallest screen dimension
fn spawn_player() -> Rect {
    let screen_dimensions = screen_dimensions();
    let player_size = (screen_dimensions * PLAYER_SCALE).min_element();
    let Vec2 {
        x: player_x,
        y: player_y,
    } = random_point(Vec2::ZERO, screen_dimensions - Vec2::splat(player_size));
    Rect::new(player_x, player_y, player_size, player_size)
}

/// draws the player [Rect] with no offset with the [PLAYER_COLOR]
fn draw_player(player: Rect) {
    draw_rectangle_ex(
        player.x,
        player.y,
        player.w,
        player.h,
        DrawRectangleParams {
            offset: Vec2::ZERO,
            rotation: 0.0,
            color: PLAYER_COLOR,
        },
    );
}
