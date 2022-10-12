pub struct GameState {
    pub score: u32,
    pub health: u32,
    pub lost: bool
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            health: 5,
            lost: false
        }
    }
}