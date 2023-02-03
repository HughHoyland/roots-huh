use macroquad::rand;
use num_traits;

pub trait Cap where Self: num_traits::Float {
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

pub fn rand(till: usize) -> usize {
    rand::rand() as usize % till
}
