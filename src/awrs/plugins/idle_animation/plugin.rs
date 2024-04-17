use bevy::prelude::*;

use crate::awrs::resources::state::AppState;

use super::idle_animation::animate_sprite_system;

pub struct IdleAnimationPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct AnimateSet;

impl Plugin for IdleAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, AnimateSet.run_if(in_state(AppState::InGame)))
            .add_systems(Update, (animate_sprite_system).in_set(AnimateSet));
    }
}
