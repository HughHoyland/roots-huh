use glam::{IVec2, vec2};
use num_traits::FloatConst;
use crate::model::BranchingStrategy;
use crate::model::plant::Plant;
use crate::model::soil::MatrixSoil;
use crate::numeric::rand;

pub struct Map {
    pub soil: MatrixSoil,
    pub plants: Vec<Plant>,
    pub size: IVec2,
}

impl Map {
    pub fn new(size: IVec2, nitros: usize) -> Self {
        let width = size.x;
        let height = size.y;

        let mut soil = MatrixSoil::new(width as usize, height as usize);
        for _ in 0..nitros {
            let r = rand(70) as f32 + 10.0;
            let x = rand(width - 2 * r as i32) + r as i32;
            let y = rand(height - 2 * r as i32) + r as i32;
            let pos = vec2(x as f32, y as f32);
            let weight = rand(10) as f32 + 2.0;
            soil.add_nitro(pos, r, weight);
        }

        let strat1 = BranchingStrategy {
            conic_ratio: 80.0,
            children_weight_rate: 0.8,
            child_weight_rate: 0.03,
            default_side_angle: -f32::PI() / 5.0,
        };

        let strat2 = BranchingStrategy {
            conic_ratio: 90.0,
            children_weight_rate: 0.9,
            child_weight_rate: 0.07,
            default_side_angle: -f32::PI() / 5.0,
        };

        let strat3 = BranchingStrategy {
            conic_ratio: 60.0,
            children_weight_rate: 0.5,
            child_weight_rate: 0.02,
            default_side_angle: -f32::PI() / 7.0,
        };

        Self {
            soil,
            plants: vec![
                Plant::new(0, 120.0, strat1),
                Plant::new(1, 240.0, strat2),
                Plant::new(2, 400.0, strat3)
            ],
            size
        }
    }

}