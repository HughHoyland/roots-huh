use glam::Vec2;
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

pub fn rand(till: i32) -> i32 {
    rand::rand() as i32 % till
}

fn float_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn float_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// Return minimum distance between line segment vw and point p
/// https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment
pub fn distance_to_segment(p: Vec2, w: Vec2, v: Vec2) -> f32 {
    // i.e. |w-v|^2 -  avoid a sqrt
    let l2 = w.distance_squared(v);

    if l2 < f32::EPSILON {
        // v == w case
        return p.distance(v);
    }

    // Consider the line extending the segment, parameterized as v + t (w - v).
    // We find projection of point p onto the line.
    // It falls where t = [(p-v) . (w-v)] / |w-v|^2
    // We clamp t from [0,1] to handle points outside the segment vw.
    let t = float_max(0.0, float_min(1.0, (p - v).dot(w - v) / l2));

    let projection = v + t * (w - v);  // Projection falls on the segment
    return p.distance(projection);
}