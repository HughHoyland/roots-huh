mod branch;
mod numeric;
mod stats;

use std::f32::consts::PI;
use glam::{Vec2};
use macroquad::color::{BEIGE, DARKBROWN, SKYBLUE};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::is_key_down;
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::window::{clear_background, Conf, next_frame, screen_width};
use crate::branch::{BranchingStrategy, MLBranch};


#[derive(Copy, Clone)]
pub enum Resource {
    Water,
    Nitro
}

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

#[derive(Clone)]
pub struct DumbSoil {}

impl Soil for DumbSoil {
    fn get_resource(&self, pos: Vec2, _what: Resource) -> f32 { 0.01 * pos.y }
    fn consume_resource(&mut self, _pos: Vec2, _what: Resource, _power: f32) -> f32 { 0.01 }
    fn get_ph(&self, _pos: Vec2) -> f32 { 5.5 }
    fn get_hardness(&self, _pos: Vec2) -> f32 { 1.0 }

    fn emit_acid(&mut self, _pos: Vec2) -> f32 { 0.0 }
    fn emit_base(&mut self, _pos: Vec2) -> f32 { 0.0 }
}


struct Plant {
    root: MLBranch,
    strategy: BranchingStrategy,
}

impl Plant {
    pub fn new(x_coord: f32) -> Self {
        let plant = Self {
            root: MLBranch::new(x_coord, 10.0),
            strategy: BranchingStrategy {
                elongation_ratio: 80.0,
                branching_ratio: 5.0,
                mass_before_children: 50.0,
                default_side_angle: -PI / 2.0,
            }
        };
        plant
    }

    pub fn grow(&mut self, soil: &mut DumbSoil) {
        let (nitro, water) = self.root.suck(soil);

        // Extension: use sunlight too.
        // Maybe need to
        let new_matter = f32::min(nitro, water);

        self.root.grow(new_matter, soil, &self.strategy);
    }
}

const SOIL_LEVEL: f32 = 50.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Root Tactics".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

struct State {
    soil: DumbSoil,
    pub plants: Vec<Plant>
}

impl State {
    pub fn new() -> Self {
        Self {
            soil: DumbSoil {},
            plants: vec![Plant::new(120.0)],
        }
    }

    pub fn draw(&self) {
        clear_background(DARKBROWN);
        draw_rectangle(0.0, 0.0, screen_width(), SOIL_LEVEL - 1.0, SKYBLUE);

        for plant in self.plants.iter() {
            self.draw_branch(&plant.root);
        }
    }

    fn draw_branch(&self, branch: &MLBranch) {

        for segment in branch.segments.iter() {
            // TODO: Conic shape, thickness.
            draw_line(
                segment.start.x,
                segment.start.y + SOIL_LEVEL,
                segment.end.x,
                segment.end.y + SOIL_LEVEL,
                1.0,
                BEIGE);

            if let Some(left) = &segment.branch {
                self.draw_branch(left);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut state = State::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_key_down(KeyCode::G) {
            for plant in state.plants.iter_mut() {
                plant.grow(&mut state.soil);
            }
        }

        state.draw();
        next_frame().await;
    }
}
