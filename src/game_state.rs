#[derive(Debug)]
pub struct GameState {
    score: u64,
    prev_score: u64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
            prev_score: 0,
        }
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn even_score(&mut self) {
        self.prev_score = self.score
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn prev_score(&self) -> u64 {
        self.prev_score
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}