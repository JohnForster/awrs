use bevy::prelude::*;

use crate::awrs::resources::state::AppState;

use super::idle_animation::animate_sprite_system;

pub struct IdleAnimationPlugin;

impl Plugin for IdleAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(animate_sprite_system),
        );
    }
}
