use std::f32::consts::PI;
use glam::{Vec2, vec2};
use crate::{DumbSoil, Resource, Soil};
use crate::numeric::Cap;

// This will define the shape of the root.
// Extension idea: Maybe make these dependent on depth or humidity?
pub struct BranchingStrategy {

    // Typical length:diameter ratio.
    // For simplicity sake, let's decide that first segment is always 2 times
    // as thick as last one.
    pub elongation_ratio: f32,

    // Branches weight:my length ratio.
    pub branching_ratio: f32,

    // Angle at which new branch tends to grow, unless it grows downwards.
    // Extension idea: maybe we want entire distribution.
    pub default_side_angle: f32,

    // TODO: Dependency on soil - water/nitro/pH.

    // Extension idea: Strength breaking a hard soil(rock)?
}

/// All recursive.
pub trait Branch {
    fn get_length(&self) -> f32;
    fn get_surface(&self) -> f32;
    fn get_weight(&self) -> f32;
    // fn get_conductivity(&self) -> f32;

    fn grow(&mut self, nutri: f32, soil: &DumbSoil);
    fn get_suck_potential(&self, what: Resource) -> f32;
}

pub enum Angle {
    Left, Right, Middle,
}

/// Distance between points in multiline.
const POINT_DISTANCE: f32 = 1.0;


struct Segment {
    // Duplicates the data. Not optimal, but convenient.
    start: Vec2,
    end: Vec2,
    weight: f32,
    branch: Option<Box<MLBranch>>,
}

/// ML stands for "multiline", a sequence of line segments.
pub struct MLBranch {
    /// N+1 point for segments.
    pub points: Vec<Vec2>,
    /// N segment weights. In the model, we use terms "weight" and "volume" interchangeably.
    pub weights: Vec<f32>,

    /// Index in the parent's segments, where `self` branched off. No sense for a root MLBranch.
    pub parent_segment_index: usize,

    /// Vec of the same size as self.weights.
    pub branches: Vec<Option<Box<MLBranch>>>,

    weight: f32,
    subtree_weight: f32,

    /// Best of `self` subtree's resource concentration.
    /// Maintain this invariant!
    pub best_nitro: f32,
    pub best_water: f32,
}

impl Branch for MLBranch {
    fn get_length(&self) -> f32 { self.weights.len() as f32 }

    fn get_surface(&self) -> f32 { self.weight / self.get_length() as f32 }

    fn get_weight(&self) -> f32 { self.weight }

    fn grow(&mut self, nutri: f32, soil: &DumbSoil) {
        todo!()
    }

    fn get_suck_potential(&self, what: Resource) -> f32 {
        100.0
    }
}

struct GrowLonger {
    direction: Vec2
}

struct GrowNewBranch {
    pub direction: Vec2,
    pub parent_segment_index: usize,
}

struct GrowChild {
    pub index: usize,
}

enum GrowthDecision {
    Longer(GrowLonger),
    Thicker,
    NewBranch(GrowNewBranch),
    Parent,
    Child(GrowChild)
}

impl MLBranch {

    pub fn new(x: f32, weight: f32) -> Self {
        Self {
            points: vec![vec2(x, 0.0), vec2(x, 1.0)],
            weights: vec![weight],
            parent_segment_index: 0,
            branches: vec![None],
            weight,
            subtree_weight: weight,
            best_nitro: 0.0,
            best_water: 0.0
        }
    }

    /// Distribute the new mass between elongation, branching and thickness.
    /// returns: distribution of (decision, weight), where sum of weights equals to 1.0
    fn growth_decision(&self, soil: &DumbSoil, strategy: &BranchingStrategy) -> Vec<(GrowthDecision, f32)> {

        let mut result = vec![];
        // Taking ML as a cylinder, so far.
        let radius = (self.weight / (self.get_length() * PI)).sqrt();
        let elongation_ratio = self.get_length() / (2.0 * radius);

        // If the strategy is 100:1, and the current ratio is 80:1, we want 20% of probability to elongate.
        // If the current ratio is 50, we want 50%.
        // TODO: Only apply if there is enough thickness.
        let elongation_probability = (1.0 - elongation_ratio / strategy.elongation_ratio)
            .cap(0.0, 1.0);

        /// Waait, it needs to depend on the resources branches provide, in the first place!

        let branch_ratio = (self.subtree_weight - self.weight) / self.get_length();
        let branching_probability = (1.0 - branch_ratio / strategy.branching_ratio)
            .cap(0.0, 1.0);

        // Extension idea: weight by the current need of the whole plant.
        let segment_resources: Vec<f32> = self.points
            // maybe I'd better have self.segments() that returns iterator...
            .skip(1)
            .iter()
            .map(|p| soil.get_resource(p, Resource::Nitro) + soil.get_resource(p, Resource::Water))
            .collect();
        let total_resources = segment_resources.iter().sum();

        let segment_distribution: Vec<f32> = segment_resources
            .iter()
            .map(|res| res / total_resources)
            .collect();

        let best_segment =

        result
    }

    pub fn grow(
        &mut self,
        // how much mass this branch or its children can gain.
        add_weight: f32,
        soil: &DumbSoil,
        strategy: &BranchingStrategy,
    ) {
        // TODO: select growth point by the best amount of the resource.


        let time_to_branch = strategy.branching_coefficient

        let grow_branch = self.find_best_point(soil, need);
        match grow_branch {
            None => {},
            Some(ResourceOnBranch{ level, point, branch }) => {
                let lll = (point - branch.end).length();
                if lll < f32::EPSILON {
                    branch.end = branch.start + (branch.end - branch.start) * 1.01
                }
            }
        }

        self.update_bests();
    }
}

