use bevy::prelude::*;

use idle_animation::idle_animation;

pub struct IdleAnimationPlugin;

impl Plugin for IdleAnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(idle_animation.system()),
        );
    }
}
