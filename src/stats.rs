use std::cmp::Ordering;

pub trait MaxIndex<TItem> {
    /// * returns (index_of_max, max)
    fn max_index(&self) -> (usize, TItem);
    fn max(&self) -> TItem;
}

impl MaxIndex<f32> for Vec<f32> {
    fn max_index(&self) -> (usize, f32) {
        self
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|(_i, r)| (r * 10000.0) as u64)
            // Ideally, I would use a "nonempty vec" type, but this will do too.
            .expect("Unexpected empty vector")
    }

    fn max(&self) -> f32 {
        Iterator::max_by(self.iter(), |a, b| if *a < *b { Ordering::Less } else { Ordering::Greater })
            .cloned()
            .expect("Unexpected empty vector")
    }
}

pub struct StatVec {
    pub vec: Vec<f32>,
    sum: f32,
    max_index: usize,
}

impl StatVec {
    pub fn new(vec: Vec<f32>) -> Self {
        let sum: f32 = vec.iter().sum();
        let (max_index, _max_val) = vec.max_index();

        Self { vec, sum, max_index }
    }

    pub fn sum(&self) -> f32 { self.sum }
}

impl MaxIndex<f32> for StatVec {
    fn max_index(&self) -> (usize, f32) {
        (self.max_index, self.vec[self.max_index])
    }

    fn max(&self) -> f32 {
        self.vec[self.max_index]
    }
}



