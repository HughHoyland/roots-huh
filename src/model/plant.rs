use crate::model::branch::{MLBranch};
use crate::model::BranchingStrategy;
use crate::model::soil::MatrixSoil;

pub struct Plant {
    pub root: MLBranch,
    pub strategy: BranchingStrategy,
    pub water_access: f32,
    pub nitro_access: f32,
}

impl Plant {
    pub fn new(id: u32, x_coord: f32, strategy: BranchingStrategy) -> Self {
        let plant = Self {
            root: MLBranch::new(id, x_coord, 10.0),
            strategy,
            water_access: 0.0,
            nitro_access: 0.0
        };
        plant
    }

    pub fn grow(&mut self, soil: &mut MatrixSoil) {
        (self.nitro_access, self.water_access) = self.root.suck(soil);

        // Extension: use sunlight too.
        // hack hack hack  + 0.2
        let new_cellulose = f32::min(self.nitro_access + 0.2, self.water_access + 0.2) * 10.0;

        self.root.grow(new_cellulose, soil, &self.strategy);
    }
}
