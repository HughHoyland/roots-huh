use macroquad::rand;

pub trait Cap where Self: Copy + Ord {
    fn cap(self, min: Self, max: Self) -> Self;
}

impl Cap for f32 {
    fn cap(self, min: Self, max: Self) -> Self {
        match self {
            _ if self < min => min,
            _ if self > max => max,
            _ => self
        }
    }
}

pub fn rand100() -> i32 {
    (rand::rand() as i32) / 100
}
