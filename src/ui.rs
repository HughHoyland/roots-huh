use macroquad::color::{DARKGRAY, WHITE};
use macroquad::prelude::{draw_rectangle, screen_height};
use macroquad::text::draw_text;
use crate::model::branch::BranchId;
use crate::model::map::Map;

pub struct MainLayout {
    // pub soil_level: f32,
    pub sidebar_width: f32,
    pub font_size: f32,
}

pub struct IngameUi {
    /// Player's plant is always #0, this is the selected one (you can select others too)
    pub selected: Option<BranchId>,

    /// "Path" to a selected branch - indexes of branches.
    pub hovered: Option<BranchId>,

    /// 0, 1 or 2 - for different speeds.
    /// On pause, press G to go.
    pub speed: u16,

    pub selected_mass: Option<f32>,
    pub selected_nitro_consumption: Option<f32>,
    pub selected_water_consumption: Option<f32>,
}

impl IngameUi {
    pub fn new() -> Self {
        Self {
            selected: None,
            hovered: None,
            speed: 0,
            selected_mass: None,
            selected_nitro_consumption: None,
            selected_water_consumption: None
        }
    }
}

pub fn draw_ui(_map: &Map, ui_state: &mut IngameUi, ui_layout: &MainLayout) {
    // The magic 1.2 works around camera translation, that I haven't figured out.
    draw_rectangle(0.0, 0.0, ui_layout.sidebar_width * 1.2, screen_height(), DARKGRAY);

    let text_top = 100.0;
    let sidebar_offset = 10.0;
    if ui_state.selected.is_some() {
        draw_text("Basic root", sidebar_offset, text_top, ui_layout.font_size as f32, WHITE);
        if let Some(mass) = ui_state.selected_mass {
            let descr = format!("Mass: {:.02}", mass);
            draw_text(&descr, sidebar_offset, text_top + ui_layout.font_size * 1.2, ui_layout.font_size as f32, WHITE);
        }
        if let Some(water) = ui_state.selected_water_consumption {
            let descr = format!("Water need: {:.02}", water);
            draw_text(&descr, sidebar_offset, text_top + ui_layout.font_size * 1.2 * 2.0, ui_layout.font_size as f32, WHITE);
        }
        if let Some(nitro) = ui_state.selected_nitro_consumption {
            let descr = format!("Water need: {:.03}", nitro);
            draw_text(&descr, sidebar_offset, text_top + ui_layout.font_size * 1.2 * 3.0, ui_layout.font_size as f32, WHITE);
        }
    }
}
