use advance_craft_engine::ScenarioState as EngineScenarioState;
use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub struct ScenarioState(pub EngineScenarioState);
