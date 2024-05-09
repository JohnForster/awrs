use crate::awrs::resources::animation::AnimationConfig;
use bevy::prelude::*;

pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in query.iter_mut() {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            atlas.index = if atlas.index == config.last_index {
                config.first_index
            } else {
                atlas.index + 1
            };
            config.frame_timer.reset()
        }
    }
}
