#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AppState {
    _MainMenu,
    InGame,
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
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
