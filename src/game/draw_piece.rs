//! draw_pieces module.
//!
//! Handles the logic to draw pieces onto the screen.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use crate::{RESOLUTION, TILESIZE, ZAxisLevel};
use bevy::prelude::{
    Entity, With, Commands, Res, ResMut, TextureAtlasSprite, SpriteSheetBundle, Component, Query,
    Vec3, Vec2, Transform, Name, default,
};
use crate::game::GameAsset;
use crate::game::PlayerSheet;

/// The width of the pieces sprite sheet.
const PIECES_SPRITESHEET_WIDTH: usize = 5_usize;

/// To distinguish piece entity.
#[derive(Component)]
pub(crate) struct Piece;

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

/// To clear all the pieces in a scene. Iterate over entity with [`Piece`] component and despawn
/// them.
fn clear_pieces(
    commands:   &mut Commands,
    query:      &Query<Entity, With<Piece>>,
) {

    // Iterate over the Pieces and despawn them.
    for pieces in query.iter() {
        commands.entity(pieces).despawn();
    }

}

/// call to draw the player [`Piece`]s.
///
/// Iterating over each player and drawing all the pieces once again. *row* and *col* correspond
/// to the player sheet resource. Hence each position along the columns correspond to the piece
/// type which is added to offset to it. The team corresponds to the rows and it is multiplied
/// with the spritesheet width to jump between the rows. The constant PIECE_SPRITESHEET_WIDTH is
/// nothing but the number of chess piece types i.e. 5.
pub(crate) fn draw_pieces(
    commands:   &mut Commands,
    sprite:     &Res<PlayerSheet>,
    game:       &ResMut<GameAsset>,
    query:      &Query<Entity, With<Piece>>,
) {

    // Clean up.
    clear_pieces(commands, query);

    // For each player.
    game.get().players.iter().for_each(|player| {

        // For each piece.
        player.pieces.iter().for_each(|piece| {

            let sprite = spawn_piece(
                commands,
                sprite,
                (
                        // Row.
                        player.team.as_usize()  * PIECES_SPRITESHEET_WIDTH
                        // Column.
                )   +   piece.piece_type.as_usize(),
                Vec3::new(
                    //piece_pos_x.
                    piece.position.x as f32     * RESOLUTION,
                    //piece_pos_y.
                    piece.position.y as f32     * RESOLUTION,
                    //Z level.
                    ZAxisLevel::Eight.as_f32(),
                ),
            );

            // Spawn.
            commands.entity(sprite).insert(Name::from("Piece")).insert(Piece);

        })

    });

}

/// Simple helper function to spawn [`Piece`] sprites. Sprite size is [`TILESIZE`].
fn spawn_piece(
    commands:       &mut Commands,
    tile:           &PlayerSheet,
    index:          usize,
    translation:    Vec3,
) -> Entity {

    // Spawn.
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index,
                custom_size: Some(Vec2::new(
                        // width.
                        TILESIZE.0 * RESOLUTION,
                        // height.
                        TILESIZE.1 * RESOLUTION,
                )),
                ..default()
            },
            texture_atlas: tile.0.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()

}
/*-----------------------------------------------------------------------------------------------*/