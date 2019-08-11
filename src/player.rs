
pub trait Stats {
    fn ba(&self) -> f64;
}

pub struct Player {
    pub abs: i32,
    pub hits: i32,
    pub coordination: i16,
}

pub trait Actions {
    fn at_bat(&mut self, rng_value: i16) -> bool;
}

impl Stats for Player {
    fn ba(&self) -> f64 {
        self.hits as f64 / self.abs as f64
    }
}

impl Actions for Player {
    fn at_bat(&mut self, rng_value: i16) -> bool {
        self.abs = self.abs + 1;
        if rng_value < self.coordination {
            self.hits = self.hits + 1;
        }
        true
    }
}
