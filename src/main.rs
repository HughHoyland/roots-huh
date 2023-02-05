mod numeric;
mod stats;
mod model;
mod draw;

use glam::{vec2};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::window::{Conf, next_frame, screen_height, screen_width};
use crate::draw::{draw_scene, SOIL_LEVEL};
use crate::numeric::{rand};
use crate::model::branch::{Branch, BranchId, MLBranch};
use crate::model::plant::Plant;
use crate::model::soil::{MatrixSoil, Soil};


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
            plants: vec![Plant::new(0, 120.0)],
            selected_plant: 0,
            selected_branch: vec![]
        }
    }

}

fn print_branch(branch: &MLBranch, offset: usize) {
    println!(
        "{: <1$}Branch {2}, length {3}, weight {4}, has {5} children:",
        "", offset, branch.id, branch.get_length(), branch.get_weight(), branch.branch_count());
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

struct DrawOutput {
    pub selected_plant: usize,
    pub selected_branch: BranchId,
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

        draw_scene(&state.plants, &state.soil);

        next_frame().await;
    }
}
