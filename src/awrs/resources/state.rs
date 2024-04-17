use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum AppState {
    _MainMenu,
    InGame,
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum GameState {
    SetUp,
    Browsing,
    _Paused,
    GameMenu,
    UnitMenu,
    _BuildingMenu,
    MoveUnit,
    ChooseTarget,
    _EnemyTurn,
    None,
}
