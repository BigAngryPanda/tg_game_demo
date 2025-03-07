const DEFAULT_ROUND_TIME: f64 = 1.0; // 1 sec

#[derive(Debug)]
pub struct GameState {
    score: u64,
    timer: f64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
            timer: DEFAULT_ROUND_TIME
        }
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn start_timer(&mut self) {
        self.timer = DEFAULT_ROUND_TIME;
    }

    pub fn tick_timer(&mut self, dt: f64) -> bool {
        self.timer -= dt;

        let result = self.timer <= 0.0;

        self.timer = self.timer.clamp(0.0, DEFAULT_ROUND_TIME);

        result
    }

    pub fn time(&self) -> f64 {
        self.timer
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}