pub mod organ;
pub mod branch;
pub mod soil;
pub mod plant;

#[derive(Copy, Clone)]
pub enum Resource {
    Water,
    Nitro
}

// This will define the shape of the root.
// Extension idea: Maybe make these dependent on depth or humidity?
pub struct BranchingStrategy {

    /// Length:diameter ratio.
    /// For simplicity sake, let's decide that first segment is always 2 times
    /// as thick as last one.
    pub conic_ratio: f32,

    /// all children:my weight ratio.
    pub children_weight_rate: f32,

    /// one child:my weight ratio.
    pub child_weight_rate: f32,

    /// Angle at which new branch tends to grow, unless it grows downwards.
    /// Extension idea: maybe we want entire distribution.
    pub default_side_angle: f32,

    // TODO: Dependency on soil - water/nitro/pH.

    // Extension idea: Strength breaking a hard soil(rock)?
}
