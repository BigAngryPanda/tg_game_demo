#[derive(Debug)]
pub struct GameState {
    score: u64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
        }
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> u64 {
        self.score
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}