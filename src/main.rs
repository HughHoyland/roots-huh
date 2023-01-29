mod branch;

use std::marker::{PhantomData};
use glam::{Vec2, vec2};
use macroquad::color::{BEIGE, DARKBROWN, LIGHTGRAY, SKYBLUE};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::models::draw_plane;
use macroquad::prelude::is_key_down;
use macroquad::rand;
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::window::{clear_background, Conf, next_frame, screen_width};
use crate::branch::{Angle, Branch};


#[derive(Copy, Clone)]
pub enum Resource {
    Water,
    Nitro
}

pub trait Soil {
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
    fn get_resource(&self, pos: Vec2, what: Resource) -> f32 { 0.01 * pos.y }
    fn consume_resource(&mut self, pos: Vec2, what: Resource, power: f32) -> f32 { 0.0 }
    fn get_ph(&self, pos: Vec2) -> f32 { 5.5 }
    fn get_hardness(&self, pos: Vec2) -> f32 { 1.0 }

    fn emit_acid(&mut self, pos: Vec2) -> f32 { 0.0 }
    fn emit_base(&mut self, pos: Vec2) -> f32 { 0.0 }
}


fn rand100() -> i32 {
    (rand::rand() as i32) / 100
}

struct Plant {
    root: Branch,
}

impl Plant {
    pub fn new(x_coord: f32) -> Self {
        Self {
            root: Branch::new_vertical(x_coord, 10.0)
        }
    }

    pub fn grow(&mut self, soil: &DumbSoil) {
        let need: Resource = Resource::Nitro;
        let soil = DumbSoil{};
        self.root.grow(&soil, need, 100.0)
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

    fn draw_branch(&self, branch: &Branch) {
        draw_line(
            branch.start.x,
            branch.start.y + SOIL_LEVEL,
            branch.end.x,
            branch.end.y + SOIL_LEVEL,
            5.0,
            BEIGE);

        if let Some(left) = &branch.left {
            self.draw_branch(left);
        }
        if let Some(right) = &branch.right {
            self.draw_branch(right);
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
                plant.grow(&state.soil);
            }
        }

        state.draw();
        next_frame().await;
    }
}
