use bevy::prelude::*;

use crate::awrs::resources::{
    atlases::UnitAtlases,
    ron_data::{UnitSpriteData, UnitSpriteDataList},
};

pub fn create_idle_sprites(
    unit_sprite_data_handle: Res<UnitSpriteData>,
    asset_server: Res<AssetServer>,
    unit_sprite_data_lists: ResMut<Assets<UnitSpriteDataList>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    info!("Creating Idle sprites...");
    if let Some(sprite_data_list) = unit_sprite_data_lists.get(&unit_sprite_data_handle.0) {
        info!("Sprite data handle: {:?}", sprite_data_list);
        let texture_handle = asset_server.load("spritesheets/custom_units.png");
        let mut unit_atlas_handles = vec![];

        for sprite_data in &sprite_data_list.0 {
            let width = ((sprite_data.dimensions.0 * sprite_data.frames)
                + (sprite_data.spacing * (sprite_data.frames - 1))) as f32;
            let height = (sprite_data.dimensions.1) as f32;

            let mut texture_atlas =
                TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(width, height));

            for n in 0..sprite_data.frames {
                let x =
                    sprite_data.top_left.0 + (sprite_data.dimensions.0 + sprite_data.spacing) * n;
                let y = sprite_data.top_left.1;

                let rect = bevy::sprite::Rect {
                    min: Vec2::new(x as f32, y as f32),
                    max: Vec2::new(
                        (x + sprite_data.dimensions.0) as f32,
                        (y + sprite_data.dimensions.1) as f32,
                    ),
                };

                texture_atlas.add_texture(rect);
            }
            let atlas_handle = texture_atlases.add(texture_atlas);
            unit_atlas_handles.push(atlas_handle);
        }

        commands.insert_resource(UnitAtlases {
            handles: unit_atlas_handles,
        })
    }
}
