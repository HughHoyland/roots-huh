use std::f32::consts::PI;
use glam::Vec2;

use crate::model::Resource;

pub trait Soil {
    /// Resource, g/cm3
    fn get_resource(&self, pos: Vec2, what: Resource) -> f32;
    fn consume_resource(&mut self, pos: Vec2, what: Resource, power: f32) -> f32;

    fn get_ph(&self, pos: Vec2) -> f32;
    // 0 to 10 by Mahs' scale.
    fn get_hardness(&self, pos: Vec2) -> f32;

    fn emit_acid(&mut self, pos: Vec2) -> f32;
    fn emit_base(&mut self, pos: Vec2) -> f32;
}


pub struct MatrixSoil {
    size_x: usize,
    size_y: usize,
    step: usize,
    water: Vec<f32>,
    nitro: Vec<f32>,
}

impl MatrixSoil {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            size_x,
            size_y,
            step: 10,
            water: vec![0.0; size_y * size_x],
            nitro: vec![0.0; size_y * size_x],
        }
    }

    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.size_x || y >= self.size_y { // x < 0 || y < 0 ||
            return None;
        }

        let x = x - x % self.step;
        let y = y - y % self.step;

        Some(x + y * (self.size_x / self.step))
    }

    fn add_at(&mut self, x: usize, y: usize, what: Resource, weight: f32) -> f32 {
        let index = match self.get_index(x, y) {
            Some(i) => i,
            None => return 0.0,
        };

        let arr = match what {
            Resource::Water => &mut self.water,
            Resource::Nitro => &mut self.nitro,
        };

        let was = arr[index];
        arr[index] += weight;
        // Suckers!
        if arr[index] < 0.0 {
            arr[index] = 0.0;
            return -was;
        }
        weight
    }

    fn get_at(&self, x: usize, y: usize, what: Resource) -> f32 {
        let index = match self.get_index(x, y) {
            Some(i) => i,
            None => return 0.0,
        };

        let arr = match what {
            Resource::Water => &self.water,
            Resource::Nitro => &self.nitro,
        };

        arr[index]
    }

    pub fn add_nitro(&mut self, pos: Vec2, radius: f32, weight: f32) {
        let points = PI * radius.powi(2) / (self.step.pow(2) as f32);
        let mut weight_left = weight;

        for x in ((pos.x - radius) as usize..(pos.x + radius) as usize).step_by(self.step) {
            for y in ((pos.y - radius) as usize..(pos.y + radius) as usize).step_by(self.step) {
                if (x as f32 - pos.x).powi(2) + (y as f32 - pos.y).powi(2) <= radius.powi(2) {
                    weight_left -= self.add_at(x, y, Resource::Nitro, weight / points);
                    if weight_left < 0.0 {
                        return;
                    }
                }
            }
        }

        self.add_at(pos.x as usize, pos.y as usize, Resource::Nitro, weight_left);
    }
}

impl Soil for MatrixSoil {
    fn get_resource(&self, pos: Vec2, what: Resource) -> f32 {
        self.get_at(pos.x as usize, pos.y as usize, what)
    }

    fn consume_resource(&mut self, pos: Vec2, what: Resource, _power: f32) -> f32 {
        let consumed = self.get_resource(pos, what);
        // let consumed = cmp::min(consumed, power);
        // let consumed = self.add_at(pos.x as usize, pos.y as usize, what, -consumed);
        // return -consumed;
        return consumed
    }

    fn get_ph(&self, _pos: Vec2) -> f32 {
        5.5
    }

    fn get_hardness(&self, _pos: Vec2) -> f32 {
        1.0
    }

    fn emit_acid(&mut self, _pos: Vec2) -> f32 {
        0.0
    }

    fn emit_base(&mut self, _pos: Vec2) -> f32 {
        0.0
    }
}