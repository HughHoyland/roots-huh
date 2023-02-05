mod numeric;
mod stats;
mod model;
mod draw;
mod ui;

use glam::{ivec2};
use macroquad::color::LIGHTGRAY;
use macroquad::input::{is_key_down, is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton};
// use macroquad::texture::{load_texture, Texture2D};
use macroquad::window::{clear_background, Conf, next_frame, screen_height, screen_width};
use crate::draw::{draw_scene, SOIL_LEVEL};
use crate::model::branch::{Branch, MLBranch};
use crate::model::map::Map;
use crate::model::plant::Plant;
use crate::model::soil::{MatrixSoil, Soil};
use crate::ui::{draw_ui, IngameUi, MainLayout};


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
    // pub soil: MatrixSoil,
    // pub plants: Vec<Plant>,
    pub ui_state: IngameUi,
    pub map: Map,

    pub ui_layout: MainLayout,
}

impl State {
    pub fn new() -> Self {
        let map_size = ivec2(screen_width() as i32 - 120, (screen_height() - SOIL_LEVEL) as i32);
        Self {
            map: Map::new(map_size, 100),
            ui_state: IngameUi::new(),
            ui_layout: MainLayout { sidebar_width: 120.0, font_size: 12.0 }
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

// pub struct Textures {
//     ui: Texture2D,
//     icons: Texture2D,
// }
//
// impl Textures {
//     pub async fn load() -> Self {
//         let texture: Texture2D = load_texture("assets/ferris.png").await.unwrap();
//         Self {
//             ui: texture,
//             icons: texture,
//         }
//     }
// }


#[macroquad::main(window_conf)]
async fn main() {

    clear_background(LIGHTGRAY);

    let mut state = State::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if state.ui_state.speed == 1 || is_key_down(KeyCode::G) {
            for plant in state.map.plants.iter_mut() {
                plant.grow(&mut state.map.soil);
            }
        }

        if is_key_pressed(KeyCode::P) {
            print_plant(&state.map.plants[0]);
        }

        state.ui_state.hovered = None;
        draw_scene(&state.map, &mut state.ui_state.hovered, &state.ui_state.selected, &state.ui_layout);

        if is_mouse_button_pressed(MouseButton::Left) && state.ui_state.hovered.is_some() {
            let selected = state.ui_state.hovered.clone().unwrap();
            let plant = state.map.plants[selected.plant as usize].root.get_branch(&selected.branch_path);
            state.ui_state.selected_mass = plant.map(|branch| branch.get_weight());
            state.ui_state.selected_water_consumption = plant.map(|branch| branch.get_weight() * 0.21);
            state.ui_state.selected_nitro_consumption = plant.map(|branch| branch.get_weight() * 0.034);
            state.ui_state.selected = Some(selected);
        }

        draw_ui(&state.map, &mut state.ui_state, &state.ui_layout);

        next_frame().await;
    }
}
