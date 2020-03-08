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
    let ceiling = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 820.0, 50.0 ),
        graphics::BLACK,
    )?;
     let R_wall = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(820.0, 0.0, 50.0, 800.0),
        graphics::BLACK,
    )?;   
     let L_wall = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 40.0, 800.0),
        graphics::BLACK,
    )?;   

    if player_state.player_physics.direction > 0.0 {
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

    graphics::draw(ctx, &ground, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &ceiling, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &L_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::draw(ctx, &R_wall, (na::Point2::new(0.0, 0.0),))?;
    graphics::present(ctx)?;
    Ok(())
}
