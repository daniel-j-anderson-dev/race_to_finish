use macroquad::{
    miniquad::date,
    prelude::*,
    rand::{gen_range, srand},
};

const PLAYER_COLOR: Color = GREEN;
const PLAYER_SCALE: f32 = 0.1;
const PLAYER_CONTROLS: Controls = Controls {
    basis: Basis2 {
        x: vec2(1.0, 0.0),
        y: vec2(0.0, 1.0),
    },
    speed: 5.0,
    x_negative: KeyCode::Left,
    x_positive: KeyCode::Right,
    y_negative: KeyCode::Up,
    y_positive: KeyCode::Down,
};

#[macroquad::main("race to finish")]
async fn main() {
    srand(date::now() as _);

    // initialize game state
    let mut player = spawn_player(PLAYER_SCALE);

    // main game loop
    loop {
        // draw
        clear_background(WHITE);
        draw_player(player, PLAYER_COLOR);

        handle_input(&mut player, PLAYER_CONTROLS);

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
fn spawn_player(player_scale: f32) -> Rect {
    let screen_dimensions = screen_dimensions();
    let player_size = (screen_dimensions * player_scale).min_element();
    let Vec2 {
        x: player_x,
        y: player_y,
    } = random_point(Vec2::ZERO, screen_dimensions - Vec2::splat(player_size));
    Rect::new(player_x, player_y, player_size, player_size)
}

/// draws the player [Rect] with no offset with the [PLAYER_COLOR]
fn draw_player(player: Rect, color: Color) {
    draw_rectangle_ex(
        player.x,
        player.y,
        player.w,
        player.h,
        DrawRectangleParams {
            offset: Vec2::ZERO,
            rotation: 0.0,
            color: color,
        },
    );
}

#[derive(Clone, Copy)]
struct Basis2 {
    x: Vec2,
    y: Vec2,
}

#[derive(Clone, Copy)]
struct Controls {
    basis: Basis2,
    speed: f32,
    x_negative: KeyCode,
    x_positive: KeyCode,
    y_negative: KeyCode,
    y_positive: KeyCode,
}
fn handle_input(player: &mut Rect, controls: Controls) {
    let mut offset = Vec2::ZERO;

    if is_key_down(controls.x_positive) {
        offset.x += 1.0
    }
    if is_key_down(controls.x_negative) {
        offset.x -= 1.0
    }
    if is_key_down(controls.y_negative) {
        offset.y -= 1.0
    }
    if is_key_down(controls.y_positive) {
        offset.y += 1.0
    }

    if offset.length_squared() > 0.0 {
        offset = offset.normalize() * controls.speed;
        let world_offset = controls.basis.x * offset.x + controls.basis.y * offset.y;
        player.x += world_offset.x;
        player.y += world_offset.y;
    }
}
