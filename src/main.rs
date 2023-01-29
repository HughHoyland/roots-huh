use std::marker::{PhantomData};
use glam::{Vec2, vec2};
use macroquad::color::LIGHTGRAY;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::rand;
use macroquad::window::{clear_background, Conf, next_frame};


#[derive(Copy, Clone)]
pub enum Resource {
    Water,
    Nitro
}

pub trait Soil {
    fn get_resource(&self, pos: Vec2, what: Resource) -> f32;
    fn consume_resource(&mut self, pos: Vec2, what: Resource, power: f32) -> f32;

    fn get_ph(&self, pos: Vec2) -> f32;

    fn emit_acid(&mut self, pos: Vec2) -> f32;
    fn emit_base(&mut self, pos: Vec2) -> f32;
}

pub struct DumbSoil {}

impl Soil for DumbSoil {
    fn get_resource(&self, pos: Vec2, what: Resource) -> f32 { 0.01 * pos.y }
    fn consume_resource(&mut self, pos: Vec2, what: Resource, power: f32) -> f32 { 0.0 }
    fn get_ph(&self, pos: Vec2) -> f32 { 5.5 }
    fn emit_acid(&mut self, pos: Vec2) -> f32 { 0.0 }
    fn emit_base(&mut self, pos: Vec2) -> f32 { 0.0 }
}

enum Angle {
    Left, Right, Middle,
}

struct Branch<'branch> {
    start: Vec2,
    end: Vec2,
    direction: Angle,
    left: Option<Box<Branch<'branch>>>,
    right: Option<Box<Branch<'branch>>>,
    phantom: PhantomData<&'branch u32>,
}

fn rand100() -> i32 {
    (rand::rand() as i32) / 100
}

struct ResourceOnBranch<'branch> {
    level: f32,
    point: Vec2,
    branch: &'branch mut Branch<'branch>,
}

impl<'branch> Branch<'branch> {

    /// returns a tuple: `( resource level, point, and which branch point belongs to)`.
    /// Maybe make it a struct?..
    fn find_best_point(&'branch mut self, soil: &DumbSoil, need: Resource) -> ResourceOnBranch {

        let start = ResourceOnBranch {
            level: soil.get_resource(self.start, need),
            point: self.start,
            branch: self,
        };
        start

        // let end = ResourceOnBranch {
        //     level: soil.get_resource(self.end, need),
        //     point: self.end,
        //     branch: self,
        // };
        //
        // let middle: Vec2 = (self.start + self.end) * 0.5;
        // let middle = ResourceOnBranch {
        //     level: soil.get_resource(middle, need),
        //     point: self.start,
        //     branch: self,
        // };
        //
        // // if let Some(left) = self.left.as_ref() {
        // //
        // // }
        // return end;
    }

    pub fn grow(&'branch mut self, soil: &DumbSoil, need: Resource) {
        // TODO: select growth point by the best amount of the resource.

        let grow_branch = self.find_best_point(soil, need);
    }
}

struct Plant<'plant> {
    root: Branch<'plant>,
    phantom: PhantomData<&'plant u32>,
}

impl<'branch> Plant<'branch> {
    pub fn new(x_coord: f32) -> Self {
        Self {
            root: Branch {
                start: vec2(x_coord, 0.),
                end: vec2(x_coord, 1.0),
                direction: Angle::Middle,
                left: None,
                right: None,
                phantom: Default::default(),
            },
            phantom: Default::default(),
        }
    }

    pub fn grow(&'branch mut self, soil: &DumbSoil) {
        let need: Resource = Resource::Nitro;
        self.root.grow(soil, need)
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Root Tactics".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await
    }
}
