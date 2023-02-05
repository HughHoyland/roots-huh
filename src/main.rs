mod branch;
mod numeric;
mod stats;
mod soil;
mod organ;

use std::f32::consts::PI;
use glam::{vec2, Vec2};
use macroquad::color::{BEIGE, BLACK, BLUE, BROWN, Color, DARKBROWN, DARKGREEN, GRAY, GREEN, SKYBLUE};
use macroquad::input::{is_key_pressed, KeyCode, mouse_position};
use macroquad::prelude::is_key_down;
use macroquad::shapes::{draw_circle, draw_line, draw_poly_lines, draw_rectangle};
use macroquad::window::{clear_background, Conf, next_frame, screen_height, screen_width};
use crate::branch::{Branch, BranchingStrategy, GrowthDecision, MLBranch};
use crate::numeric::{distance_to_segment, rand};
use crate::soil::{MatrixSoil, Soil};


#[derive(Copy, Clone)]
pub enum Resource {
    Water,
    Nitro
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
                conic_ratio: 80.0,
                children_weight_rate: 0.8,
                child_weight_rate: 0.03,
                default_side_angle: -PI / 4.0,
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
    pub soil: MatrixSoil,
    pub plants: Vec<Plant>,
    // Player's plant is always #0, this is the selected one (you can select others too)
    pub selected_plant: usize,
    /// "Path" to a selected branch - indexes of branches.
    pub selected_branch: Vec<usize>,
}

impl State {
    pub fn new() -> Self {
        let width = screen_width() as usize;
        let height = (screen_height() - SOIL_LEVEL) as usize;
        let mut soil = MatrixSoil::new(width, height);
        for _ in 0..100 {
            let r = rand(70) as f32 + 10.0;
            let x = rand(width - 2 * r as usize) + r as usize;
            let y = rand(height - 2 * r as usize) + r as usize;
            let pos = vec2(x as f32, y as f32);
            let weight = rand(10) as f32 + 2.0;
            soil.add_nitro(pos, r, weight);
        }

        Self {
            soil,
            plants: vec![Plant::new(120.0)],
            selected_plant: 0,
            selected_branch: vec![]
        }
    }

    fn resource_draw_size(quantity: f32) -> f32 {
        if quantity < 0.0 {
            0.0
        } else if quantity < 2.9 {
            quantity * 2.0
        } else {
            quantity.ln() + 3.0
        }
    }

    fn draw_decision(x: f32, decisions: Vec<(GrowthDecision, f32)>) {
        let mut long = 0.0;
        let mut branches = 0.0;
        let mut new_branches = 0.0;
        let mut thick = 0.0;
        for (d, weight) in decisions.iter() {
            match d {
                GrowthDecision::Longer(_) => long += weight,
                GrowthDecision::Child(_) => branches += weight,
                GrowthDecision::NewBranch(_) => new_branches += weight,
                GrowthDecision::Myself => thick += weight,
            }
        }

        let rect_width = 15.0;
        let rect_height = 30.0;

        fn draw_bar(x: f32, height: f32, color: Color) {
            let offset = 10.0;
            let rect_width = 15.0;

            draw_rectangle(
                x,
                SOIL_LEVEL - offset - height,
                rect_width,
                height + 1.0,
                color);
        }

        draw_bar(x - rect_width * 1.6, long * rect_height, BROWN);
        draw_bar(x - rect_width * 0.5, branches * rect_height, DARKGREEN);
        draw_bar(x + rect_width * 0.6, thick * rect_height, DARKBROWN);
        draw_bar(x + rect_width * 1.7, new_branches * rect_height, DARKGREEN);
        // draw_bar(x + rect_width * 1.6, rect_height, BLACK);
        if long + branches + thick < 0.99 {
            println!("Not enough weight!");
        }
    }

    pub fn draw(&self) {
        clear_background(DARKBROWN);
        draw_rectangle(0.0, 0.0, screen_width(), SOIL_LEVEL - 1.0, SKYBLUE);

        let mouse_pos: Vec2 = mouse_position().into();

        let mut hover_drawn = false;

        for plant in self.plants.iter() {
            self.draw_branch(&plant.root, mouse_pos, &mut hover_drawn);
            let decision = plant.root.growth_decision(&self.soil, 1.0, &plant.strategy);
            Self::draw_decision(plant.root.segments[0].start.x, decision);
        }

        let max_y = (screen_height() - SOIL_LEVEL) as i32;

        for x in (0..screen_width() as i32).step_by(20) {
            for y in (0..max_y).step_by(10) {
                let pos = vec2(x as f32, y as f32);
                let water = self.soil.get_resource(pos, Resource::Water);
                let nitro = self.soil.get_resource(pos, Resource::Nitro);

                if water > 0.0 {
                    let size = Self::resource_draw_size(water);
                    draw_poly_lines(pos.x, pos.y + SOIL_LEVEL, 3, size, 0.0, 1.0, BLUE);
                }
                if nitro > 0.0 {
                    let size = Self::resource_draw_size(nitro);
                    draw_poly_lines(pos.x + 5.0, pos.y + 2.0 + SOIL_LEVEL, 4, size, 0.0, 1.0, GRAY);
                }
            }

        }
    }

    fn draw_branch(&self, branch: &MLBranch, mouse_pos: Vec2, hover_drawn: &mut bool) {

        let mut color = BEIGE;

        if ! *hover_drawn {
            let d_mouse = distance_to_segment(
                mouse_pos,
                branch.segments[0].start,
                branch.segments.last().unwrap().end);

            if d_mouse < 5.0 {
                color = GREEN;
                *hover_drawn = true;
            }
        }

        for (i, segment) in branch.segments.iter().enumerate() {
            if let Some(left) = &segment.branch {
                self.draw_branch(left, mouse_pos, hover_drawn);
            }

            let thickness = 7.0 * (branch.get_length() - i as f32) / branch.get_length();
            // TODO: Conic shape, thickness.
            draw_line(
                segment.start.x,
                segment.start.y + SOIL_LEVEL,
                segment.end.x,
                segment.end.y + SOIL_LEVEL,
                1.0 + thickness,
                color);

        }

        // draw_circle(
        //     branch.segments[0].start.x,
        //     branch.segments[0].start.y + SOIL_LEVEL,
        //     10.0, GREEN);
        // draw_circle(
        //     branch.segments.last().unwrap().start.x,
        //     branch.segments.last().unwrap().start.y + SOIL_LEVEL,
        //     10.0, GRAY);
    }
}

fn print_branch(branch: &MLBranch, offset: usize) {
    println!(
        "{: <1$}Branch {2}, length {3}, weight {4}, has {5} children:",
        "", offset, branch, branch.get_length(), branch.get_weight(), branch.branch_count());
    for segment in branch.segments.iter() {
        if let Some(branch) = segment.branch.as_ref() {
            print_branch(branch, offset + 2);
        }
    }
}

fn print_plant(p0: &Plant) {
    let branch = &p0.root;
    print_branch(branch, 0);
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

        if is_key_pressed(KeyCode::P) {
            print_plant(&state.plants[0]);
        }

        state.draw();
        next_frame().await;
    }
}
