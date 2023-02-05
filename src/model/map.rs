use glam::{IVec2, vec2};
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

        Self {
            soil,
            plants: vec![Plant::new(0, 120.0)],
            size
        }
    }

}