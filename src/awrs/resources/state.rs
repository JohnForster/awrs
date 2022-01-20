#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AppState {
    _MainMenu,
    InGame(GameState),
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState {
    SetUp,
    Browsing,
    _Paused,
    UnitMenu,
    _BuildingMenu,
    MoveUnit,
    ChooseTarget,
    _EnemyTurn,
}
