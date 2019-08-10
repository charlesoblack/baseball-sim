
pub trait Stats {
    fn ba(&self) -> f64;
}

pub struct Player {
    pub abs: i16,
    pub hits: i16,
}

impl Stats for Player {
    fn ba(&self) -> f64 {
        self.hits as f64 / self.abs as f64
    }
}
