use num_traits::FloatConst;
use crate::model::branch::MLBranch;
use crate::model::BranchingStrategy;
use crate::model::soil::MatrixSoil;

pub struct Plant {
    pub root: MLBranch,
    pub strategy: BranchingStrategy,
}

impl Plant {
    pub fn new(x_coord: f32) -> Self {
        let plant = Self {
            root: MLBranch::new(x_coord, 10.0),
            strategy: BranchingStrategy {
                conic_ratio: 80.0,
                children_weight_rate: 0.8,
                child_weight_rate: 0.03,
                default_side_angle: -f32::PI() / 4.0,
            }
        };
        plant
    }

    pub fn grow(&mut self, soil: &mut MatrixSoil) {
        let (nitro, water) = self.root.suck(soil);

        // Extension: use sunlight too.
        // hack hack hack  + 0.2
        let new_matter = f32::min(nitro + 0.2, water + 0.2) * 10.0;

        self.root.grow(new_matter, soil, &self.strategy);
    }
}
