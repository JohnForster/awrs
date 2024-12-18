use bevy::prelude::*;

use crate::awrs::{
    constants::TILE_SIZE,
    resources::{
        atlases::CursorAtlas,
        cursor::{ChangeCursorEvent, Cursor, CursorStyle},
        tile::Tile,
    },
};

pub fn get_cursor_adjustment(cursor_style: &CursorStyle) -> Vec3 {
    match cursor_style {
        CursorStyle::Target => Vec3::new(4.0, -5.0, 2.0),
        CursorStyle::TargetSplash => Vec3::new(0.0, 0.0, 2.0),
        CursorStyle::Browse => Vec3::new(4.0, -5.0, 2.0),
        CursorStyle::None => Vec3::ZERO,
    }
}

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<CursorAtlas>) {
    info!("Creating Cursor");
    let tile = Tile { x: 0, y: 0 };
    let starting_position = Vec3::new(tile.x as f32, tile.y as f32, 0.0) * TILE_SIZE;
    let adjustment = get_cursor_adjustment(&CursorStyle::Browse);

    // Combine these into the Cursor struct?
    commands
        .spawn((
            Cursor,
            SpatialBundle {
                transform: Transform::from_translation(starting_position),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((SpriteSheetBundle {
                texture: ui_atlas.texture.clone(),
                atlas: TextureAtlas {
                    layout: ui_atlas.layout.clone(),
                    index: CursorStyle::Browse as usize,
                },
                transform: Transform::from_translation(adjustment),
                ..Default::default()
            },));
        });
}

pub fn handle_change_cursor(
    mut ev_change_cursor: EventReader<ChangeCursorEvent>,
    mut q_cursor_children: Query<&mut Children, With<Cursor>>,
    mut q_sprite: Query<(&mut TextureAtlas, &mut Visibility, &mut Transform)>,
) {
    for ChangeCursorEvent(cursor_style) in ev_change_cursor.read() {
        let sprite_index = match cursor_style {
            CursorStyle::Browse => 0,
            CursorStyle::Target => 1,
            CursorStyle::TargetSplash => 2,
            CursorStyle::None => {
                info!("Hiding cursor");
                let cursor_children = q_cursor_children.single_mut();

                for child in cursor_children.iter() {
                    if let Ok((_, mut visibility, _)) = q_sprite.get_mut(*child) {
                        *visibility = Visibility::Hidden;
                    }
                }
                continue;
            }
        };
        info!("Changing cursor sprite index to {:?}", sprite_index);
        let cursor_children = q_cursor_children.single_mut();

        for child in cursor_children.iter() {
            if let Ok((mut atlas, mut visibility, mut transform)) = q_sprite.get_mut(*child) {
                atlas.index = sprite_index;
                *visibility = Visibility::Visible;
                transform.translation = get_cursor_adjustment(cursor_style);
            }
        }
    }
}
