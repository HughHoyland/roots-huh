use std::f32::consts::PI;
use glam::{Vec2, vec2};
use crate::{MatrixSoil, Resource, Soil};
use crate::numeric::Cap;
use crate::stats::{MaxIndex, StatVec};

// This will define the shape of the root.
// Extension idea: Maybe make these dependent on depth or humidity?
pub struct BranchingStrategy {

    // Typical length:diameter ratio.
    // For simplicity sake, let's decide that first segment is always 2 times
    // as thick as last one.
    // TODO: Think again if it's a good parameter. I think roots can be of any
    // length, given plant has the materials?
    pub elongation_ratio: f32,

    // Branches weight:my weight ratio.
    pub branching_ratio: f32,

    pub mass_before_children: f32,

    // Angle at which new branch tends to grow, unless it grows downwards.
    // Extension idea: maybe we want entire distribution.
    pub default_side_angle: f32,

    // TODO: Dependency on soil - water/nitro/pH.

    // Extension idea: Strength breaking a hard soil(rock)?
}

/// All recursive.
pub trait Branch {
    fn get_length(&self) -> f32;
    fn get_radius(&self) -> f32;
    fn get_surface(&self) -> f32;
    fn get_weight(&self) -> f32;
    // fn get_conductivity(&self) -> f32;

    fn grow(&mut self, nutri: f32, soil: &MatrixSoil);
    fn get_suck_potential(&self, what: Resource) -> f32;
}

// pub enum Angle {
//     Left, Right, Middle,
// }

/// Distance between points in multiline.
const SEGMENT_LENGTH: f32 = 1.0;


pub struct Segment {
    // `start` duplicates the data. Not optimal, but convenient.
    pub start: Vec2,
    pub end: Vec2,
    pub branch: Option<Box<MLBranch>>,
}

impl Segment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end, branch: None }
    }

    // pub fn get_segment_resource(&self, soil: &MatrixSoil, what: Resource) -> f32 {
    //     soil.get_resource(self.end, what) + match self.branch.as_ref() {
    //         None => 0.0,
    //         Some(branch) => match what {
    //             Resource::Water => branch.best_water,
    //             Resource::Nitro => branch.best_nitro,
    //         }
    //     }
    // }

    /// ranging -pi..pi
    pub fn angle(&self) -> f32 {
        let delta = self.end - self.start;
        delta.y.atan2(delta.x)
    }
}

/// ML stands for "multiline", a sequence of line segments.
pub struct MLBranch {
    pub segments: Vec<Segment>,

    /// Index in the parent's segments, where `self` branched off. No sense for a root MLBranch.
    pub parent_segment_index: usize,

    weight: f32,
    subtree_weight: f32,

    /// Best of `self` subtree's resource concentration.
    /// Maintain this invariant!
    pub best_nitro: f32,
    pub best_water: f32,
}

impl Branch for MLBranch {
    fn get_length(&self) -> f32 { self.segments.len() as f32 }

    /// This would be the *average* radius.
    fn get_radius(&self) -> f32 {
        // Taking it as a cylinder, so far, while it should be cone.
        (self.weight / (self.get_length() * PI)).sqrt()
    }

    fn get_surface(&self) -> f32 { self.weight / self.get_length() as f32 }

    fn get_weight(&self) -> f32 { self.weight }

    fn grow(&mut self, _nutri: f32, _soil: &MatrixSoil) {
        todo!()
    }

    fn get_suck_potential(&self, _what: Resource) -> f32 {
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
    NewBranch(GrowNewBranch),
    Child(GrowChild),
    Myself,
}

impl MLBranch {

    pub fn new(x: f32, weight: f32) -> Self {
        Self {
            segments: vec![
                Segment::new(vec2(x, 0.0), vec2(x, SEGMENT_LENGTH))
            ],
            parent_segment_index: 0,
            weight,
            subtree_weight: weight,
            best_nitro: 0.0,
            best_water: 0.0
        }
    }

    pub fn new_branch(start: Vec2, end: Vec2, parent_segment_index: usize, weight: f32) -> Self {
        Self {
            segments: vec![ Segment::new(start, end) ],
            parent_segment_index,
            weight,
            subtree_weight: weight,
            best_nitro: 0.0,
            best_water: 0.0
        }
    }

    /// Distribute the new mass between elongation, branching and thickness.
    /// returns: distribution of (decision, weight), where sum of weights equals to 1.0
    fn growth_decision(
        &self,
        soil: &MatrixSoil,
        _new_material: f32,
        strategy: &BranchingStrategy
    ) -> Vec<(GrowthDecision, f32)>
    {
        let elongation_ratio = self.get_length() / (2.0 * self.get_radius());

        // If the strategy is 100:1, and the current ratio is 80:1, we want 20% of probability to elongate.
        // If the current ratio is 50, we want 50%.
        // TODO: Only apply if there is enough thickness.
        let elongation_probability = (1.0 - elongation_ratio / strategy.elongation_ratio)
            .cap(0.0, 1.0);

        let branch_ratio = (self.subtree_weight - self.weight) / self.weight;
        let branching_probability = (1.0 - branch_ratio / strategy.branching_ratio)
            .cap(0.0, 1.0);

        // Extension idea: weight by the current need of the whole plant.
        let segment_resources: Vec<f32> = self.segments
            .iter()
            .map(|s| soil.get_resource(s.end, Resource::Nitro)
                + soil.get_resource(s.end, Resource::Water))
            .collect();
        let segment_resources = StatVec::new(segment_resources);

        // let segment_weights: Vec<f32> = segment_resources.vec
        //     .iter()
        //     .map(|res| res / segment_resources.sum())
        //     .collect();

        let branch_resources: Vec<f32> = self.segments.iter()
            .map(|s|
                s.branch.as_ref().map(|br| br.best_nitro + br.best_water).unwrap_or_default())
            .collect();
        let branch_resources = StatVec::new(branch_resources);

        let (best_segment_idx, best_segment_resource) = segment_resources.max_index();
        let (_best_branch_idx, best_branch_resource) = branch_resources.max_index();
        let last_segment = self.segments.last().expect("Empty branch!");

        // Eventually, I will need a weighted sum function for them, parametrized by the current needs.
        let end_resource = soil.get_resource(last_segment.end, Resource::Nitro)
            + soil.get_resource(last_segment.end, Resource::Water);

        // If the end has a good enough resource - grow longer.
        // 0.8 is very arbitrary. Could be part of the strategy too.
        let elongate_is_good_enough =
            ((best_branch_resource > f32::EPSILON)
                && (end_resource / best_branch_resource * elongation_probability > 0.8))
                || ((best_segment_resource > f32::EPSILON)
                && (end_resource / best_segment_resource * elongation_probability > 0.8));

        if elongate_is_good_enough {
            println!("Elongate: ratio: own: {} expected: {}, resource {}/{}, probability: {}, weight: {}",
                elongation_ratio, strategy.elongation_ratio,
                end_resource, best_segment_resource, elongation_probability, self.weight);

            let next_point = last_segment.end + (last_segment.end - last_segment.start);
            return vec![(
                GrowthDecision::Longer(GrowLonger { direction: next_point }),
                1.0
            )];
        }

        // Grow a new branch.
        // * more construction material increases the likelihood
        // * very rich soil increases the probability
        // * existing branching ratio affects the probability.
        if self.weight >= strategy.mass_before_children
            && branching_probability > 0.9
            && self.segments[best_segment_idx].branch.is_none()
        {
            let best_segment = &self.segments[best_segment_idx];
            // TODO: Rewrite. Use strategy. Make it a strategy method?
            // TODO: Make left and right branches interchange.
            // TODO: Maybe grow a new branch per each N grams of current branch mass, at regular intervals?
            let new_branch_angle = if best_segment.angle() > PI / 2.0 + f32::EPSILON {
                PI / 2.0
            } else if best_segment.angle() > PI / 4.0 + f32::EPSILON {
                PI / 4.0
            } else {
                PI * (3.0/4.0)
            };

            let next_point = best_segment.end
                + vec2(SEGMENT_LENGTH * new_branch_angle.cos(), SEGMENT_LENGTH * new_branch_angle.sin());

            println!("New branch. Branching ratio: my: {}, strat: {}. Branch at {}",
                branch_ratio, strategy.branching_ratio, next_point);

            return vec![(
                GrowthDecision::NewBranch(GrowNewBranch {
                    direction: next_point,
                    parent_segment_index: best_segment_idx
                }),
                1.0
            )];
        }

        // Distribute to children branches proportionally to their contribution
        if best_branch_resource > f32::EPSILON {
            let branch_weights: Vec<_> = branch_resources.vec
                .iter()
                .enumerate()
                .filter_map(|(index, val)| if *val > f32::EPSILON {
                        Some((
                            GrowthDecision::Child(GrowChild{ index }),
                            (*val / branch_resources.sum())
                        ))
                    } else { None }
                )
                .collect();
            return branch_weights;
        }

        // If the sum of my chilren's (transport capacity) is greater than mine,
        // allocate some (all?) resources to grow myself

        let next_point = last_segment.end + (last_segment.end - last_segment.start);
        vec![(
            GrowthDecision::Longer(GrowLonger { direction: next_point }),
            1.0
        )]
    }

    pub fn grow(
        &mut self,
        // how much mass this branch or its children can gain.
        new_material: f32,
        soil: &MatrixSoil,
        strategy: &BranchingStrategy,
    ) {
        let decision = self.growth_decision(soil, new_material, strategy);

        for (application, _weight) in decision {
            match application {
                GrowthDecision::Longer(GrowLonger{ direction }) => {
                    let last_segment = self.segments.last()
                        .expect("Empty branch, really?");
                    self.segments.push(Segment::new(last_segment.end, direction));
                    self.weight += new_material;
                }

                GrowthDecision::NewBranch(GrowNewBranch{ direction, parent_segment_index }) => {
                    let cur_segment = &mut self.segments[parent_segment_index];
                    if cur_segment.branch.is_some() {
                        panic!("GrowthDecision::NewBranch - already have a branch");
                    }
                    cur_segment.branch = Some(Box::new(
                        MLBranch::new_branch(
                            cur_segment.end,
                            direction,
                            parent_segment_index,
                            new_material)));
                }

                GrowthDecision::Child(GrowChild{ index }) =>
                    self.segments[index].branch
                        .as_mut()
                        .expect("GrowthDecision::Child - bad index")
                        .grow(new_material, soil, strategy),

                GrowthDecision::Myself =>
                    self.weight += new_material,
            }

            self.subtree_weight += new_material;
        }

        // self.update_bests();
    }

    /// * returns (nitro, water)
    pub fn suck(&mut self, soil: &mut MatrixSoil) -> (f32, f32) {

        // FIXME: The consumption must happen AT THE SAME TIME, not sequentially,
        // so that branches will compete for resources.
        // BTW it must happen for multiple plants!
        // So Plant needs to produce a "request" object, and the Soil will "fulfill"
        // a batch of them.

        let mut best_nitro = 0.0;
        let mut best_water = 0.0;
        let mut total_nitro = 0.0;
        let mut total_water = 0.0;

        for segment in self.segments.iter_mut() {
            let nitro = soil.consume_resource(segment.end, Resource::Nitro, 1.0);
            total_nitro += nitro;
            if nitro > best_nitro {
                best_nitro = nitro;
            }
            let water = soil.consume_resource(segment.end, Resource::Water, 1.0);
            total_water += water;
            if water > best_water {
                best_water = water;
            }

            if let Some(branch) = segment.branch.as_mut() {
                let (seg_nitro, seg_water) = branch.suck(soil);
                total_water += seg_water;
                total_nitro += seg_nitro;
            }
        }
        self.best_nitro = best_nitro;
        self.best_water = best_water;

        (total_nitro, total_water)
    }
}

