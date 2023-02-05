use macroquad::color::{DARKGRAY, LIGHTGRAY, WHITE};
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::Rect;
use macroquad::prelude::{draw_rectangle, screen_height};
use macroquad::shapes::draw_poly_lines;
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

// TODO: egui
pub fn draw_ui(_map: &Map, ui_state: &mut IngameUi, ui_layout: &MainLayout) {
    // The magic 1.2 works around camera translation, that I haven't figured out.
    draw_rectangle(0.0, 0.0, ui_layout.sidebar_width * 1.2, screen_height(), DARKGRAY);

    // speed controls

    let sidebar_offset = 10.0;
    let line_height = ui_layout.font_size * 1.2;

    let play = Rect::new(sidebar_offset, sidebar_offset, line_height, line_height);
    draw_rectangle(play.x, play.y, play.w, play.h, LIGHTGRAY);
    if ui_state.speed != 0 {
        draw_rectangle(sidebar_offset + line_height * 0.2, sidebar_offset + line_height * 0.2, line_height * 0.2, line_height * 0.6, DARKGRAY);
        draw_rectangle(sidebar_offset + line_height * 0.6, sidebar_offset + line_height * 0.2, line_height * 0.2, line_height * 0.6, DARKGRAY);
    } else {
        draw_poly_lines(sidebar_offset + line_height * 0.5, sidebar_offset + line_height * 0.5, 3, line_height * 0.3, 0.0, 2.0, DARKGRAY);
    }

    if is_mouse_button_pressed(MouseButton::Left)
        && play.contains(mouse_position().into()) {
        ui_state.speed = if ui_state.speed == 0 { 1 } else { 0 };
    }

    let text_top = 100.0;
    if ui_state.selected.is_some() {

        let draw_line = |text: &str, line_no: f32| {
            draw_text(text, sidebar_offset, text_top + line_height * line_no, ui_layout.font_size as f32, WHITE);
        };

        draw_line("Basic root", 0.0);
        if let Some(mass) = ui_state.selected_mass {
            let descr = format!("Mass: {:.02}", mass);
            draw_line(&descr, 1.0);

            let descr = format!("Brings water/nitro: {:.02}/{:.02}", 0.0, 0.0);
            draw_line(&descr, 3.0);
        }
        if let Some(water) = ui_state.selected_water_consumption {
            let nitro = ui_state.selected_nitro_consumption.unwrap_or_default();
            let descr = format!("Needs water/nitro: {:.02}/{:.02}", water, nitro);
            draw_line(&descr, 2.0);
        }
    }
}
