use advance_craft_engine::ScenarioState as EngineScenarioState;
use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource, Clone)]
pub struct ScenarioState(pub EngineScenarioState);
