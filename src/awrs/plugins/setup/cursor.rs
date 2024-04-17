use bevy::prelude::*;

use crate::awrs::{
    constants::TILE_SIZE,
    resources::{
        atlases::CursorAtlas,
        cursor::{ChangeCursorEvent, Cursor, CursorStyle},
        tile::Tile,
    },
};

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<CursorAtlas>) {
    info!("Creating Cursor");
    let tile = Tile { x: 0, y: 0 };
    let starting_position = Vec3::new(tile.x as f32, tile.y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 2.0);

    // Combine these into the Cursor struct?
    commands
        .spawn((
            Cursor,
            GlobalTransform::default(),
            Transform::from_translation(starting_position),
        ))
        .with_children(|parent| {
            parent.spawn((
                GlobalTransform::default(),
                SpriteSheetBundle {
                    texture_atlas: ui_atlas.atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_translation(adjustment),
                    ..Default::default()
                },
            ))
        });
}

pub fn handle_change_cursor(
    mut ev_change_cursor: EventReader<ChangeCursorEvent>,
    mut q_cursor_children: Query<&mut Children, With<Cursor>>,
    mut q_sprite: Query<(&mut TextureAtlasSprite, &mut Visibility)>,
) {
    for ChangeCursorEvent(cursor_style) in ev_change_cursor.iter() {
        let sprite_index = match cursor_style {
            CursorStyle::Browse => 0,
            CursorStyle::Target => 1,
            CursorStyle::None => {
                info!("Hiding cursor");
                let cursor_children = q_cursor_children.single_mut();

                for child in cursor_children.iter() {
                    if let Ok((_, mut visibility)) = q_sprite.get_mut(*child) {
                        visibility.is_visible = false;
                    }
                }
                continue;
            }
        };
        info!("Changing cursor sprite index to {:?}", sprite_index);
        let cursor_children = q_cursor_children.single_mut();

        for child in cursor_children.iter() {
            if let Ok((mut cursor_sprite, mut visibility)) = q_sprite.get_mut(*child) {
                cursor_sprite.index = sprite_index;
                visibility.is_visible = true;
            }
        }
    }
}
