pub enum GameEvent {
    PlayerMove(i32, i32),
    DamageGoblin,
    WriteToLog(String),
    Exit,
}