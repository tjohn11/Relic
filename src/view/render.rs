use crate::PlayerState;
use ggez::{graphics, nalgebra as na, Context, GameResult};

pub fn render_game(player_state: &mut PlayerState, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

    let ground = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        player_state.map_model,
        graphics::BLACK,
    )?;

    // Draw player movement
    if player_state.player_physics.x_velocity > 0.0 {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[0],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - 32.0,
            ),),
        )?;
    } else {
        graphics::draw(
            ctx,
            &player_state.resources.character_sprite[1],
            (na::Point2::new(
                player_state.player_physics.x_pos,
                player_state.player_physics.y_pos - 32.0,
            ),),
        )?;
    }

    // Draw the ground
    graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;
    graphics::present(ctx)?;
    Ok(())
}