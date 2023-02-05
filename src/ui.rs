use crate::model::branch::BranchId;

pub struct MainLayout {
    // pub soil_level: f32,
    pub sidebar_width: f32,
    pub font_size: usize,
}

pub struct IngameUi {
    /// Player's plant is always #0, this is the selected one (you can select others too)
    pub selected: Option<BranchId>,

    /// "Path" to a selected branch - indexes of branches.
    pub hovered: Option<BranchId>,

    /// 0, 1 or 2 - for different speeds.
    /// On pause, press G to go.
    pub speed: u16,

    pub selected_mass: Option<u32>,
    pub selected_nitro_consumption: Option<u32>,
    pub selected_water_consumption: Option<u32>,
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

