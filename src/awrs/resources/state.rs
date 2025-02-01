use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum AppState {
    MainMenu,
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
    _Waiting,
    None,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, States)]
pub enum MenuState {
    Open,
    Closed,
}
