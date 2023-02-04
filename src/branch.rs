use std::f32::consts::PI;
use glam::{Vec2, vec2};
use num_traits::FloatConst;
use crate::{MatrixSoil, Resource, Soil};

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

    /// ranging -pi..pi
    pub fn angle(&self) -> f32 {
        let delta = self.end - self.start;
        delta.y.atan2(delta.x)
    }

    pub fn vec(&self) -> Vec2 {
        self.end - self.start
    }
}

/// ML stands for "multiline", a sequence of line segments.
pub struct MLBranch {
    // A sequence of branch indexes, from the root of the root.
    pub id: Vec<usize>,

    pub segments: Vec<Segment>,

    /// Index in the parent's segments, where `self` branched off. No sense for a root MLBranch.
    pub parent_segment_index: usize,

    /// My own weight
    weight: f32,
    /// My entire subtree weight, including me
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

struct GrowLonger(Vec2);

struct GrowNewBranch {
    pub direction: Vec2,
    pub parent_segment_index: usize,
}

struct GrowChild(usize);

enum GrowthDecision {
    Longer(GrowLonger),
    NewBranch(GrowNewBranch),
    Child(GrowChild),
    Myself,
}

impl MLBranch {

    pub fn new(x: f32, weight: f32) -> Self {
        Self {
            id: vec![],
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

    pub fn new_branch(start: Vec2, end: Vec2, parent_segment_index: usize, parent_id: Vec<usize>, weight: f32) -> Self {
        let mut id = parent_id;
        id.push(parent_segment_index);
        Self {
            id,
            segments: vec![ Segment::new(start, end) ],
            parent_segment_index,
            weight,
            subtree_weight: weight,
            best_nitro: 0.0,
            best_water: 0.0
        }
    }

    // fn branch_count(&self) -> usize {
    //     self.segments.iter()
    //         .filter_map(|s| s.branch.as_ref().map(|_| true))
    //         .count()
    // }

    fn get_child_angle(&self, index: usize) -> f32 {
        let my_direction = self.segments[index].vec();
        let child_direction = self.segments[index].branch
            .as_ref()
            .expect("Branch expected")
            .segments[0]
            .vec();
        my_direction.angle_between(child_direction)
    }

    fn last_branch_index(&self) -> Option<usize> {
        self.segments.iter().enumerate()
            .filter_map(|(i, s)| s.branch.as_ref().map(|_| i))
            .last()
    }

    fn grow_new_branch(&self) -> Option<GrowthDecision> {
        // * On one hand, branch interval depends on my size.
        // * On the other hand, the old branches will sit too tight then?..
        // Let's just stick a branch at 1/2 of the remaining length and see!

        let last_branch_index = self.last_branch_index();

        let new_branch_segment = match last_branch_index {
            None => self.segments.len() / 2,
            Some(index) if index + 2 >= self.segments.len() => return None,
            Some(index) => index + (self.segments.len() - index) / 2,
        };

        if new_branch_segment >= self.segments.len() || self.segments[new_branch_segment].branch.is_some() {
            panic!("new_branch_segment={}: something went wrong", new_branch_segment);
        }

        let pseudo_rnd_sign = (new_branch_segment as i32 & 1) * 2 - 1;
        let new_branch_angle = match last_branch_index {
            None => self.segments[new_branch_segment].angle() + (pseudo_rnd_sign as f32) * f32::PI() / 4.0,
            Some(index) => -self.get_child_angle(index),
        };

        let next_point = self.segments[new_branch_segment].end
            + vec2(SEGMENT_LENGTH * new_branch_angle.cos(), SEGMENT_LENGTH * new_branch_angle.sin());

        // println!("New branch. Branching ratio: my: {}, start: {}. Branch at {}",
        //          branch_ratio, strategy.branching_ratio, next_point);

        Some( GrowthDecision::NewBranch( GrowNewBranch {
            direction: next_point,
            parent_segment_index: new_branch_segment
        }))
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
        // c = children's share
        // m = my share
        // c/m = children_weight_rate
        // Solution.
        // m = 1 - c
        // children_weight_rate = c / (1 - c)
        // c = (1 - c) * children_weight_rate
        // c = children_weight_rate - c*children_weight_rate
        // c * (1 + children_weight_rate) = children_weight_rate
        // c = children_weight_rate / (1 + children_weight_rate)
        let children_share = strategy.children_weight_rate / (strategy.children_weight_rate + 1.0);

        let min_child_mass: f32 = 1.0;
        let min_mass_for_children = min_child_mass / strategy.child_weight_rate;

        let last_branch_index = self.last_branch_index();

        let mut child_decisions: Vec<_> = vec![];
        if self.weight > min_mass_for_children {
            // TODO: Move this magic number into the strategy?
            if last_branch_index.is_none()
                || (last_branch_index.unwrap() as f32 / self.segments.len() as f32) < 0.3
            {
                if let Some(decision) = self.grow_new_branch() {
                    child_decisions = vec![ (decision, 1.0) ];
                }
            }
        }


        if child_decisions.is_empty() {

            let branch_resources: Vec<f32> = self.segments.iter()
                .map(|s|
                    s.branch.as_ref().map(|br| br.best_nitro + br.best_water).unwrap_or_default())
                .collect();
            let total_branch_resources: f32 = branch_resources.iter().sum();

            child_decisions = self.segments.iter()
                .enumerate()
                .filter(|(i, seg)| seg.branch.is_some())
                .map(|(i, seg)| (
                    GrowthDecision::Child( GrowChild(i) ),
                    children_share * branch_resources[i] / total_branch_resources
                ))
                .collect();
        }


        let mut result = child_decisions;

        let my_share = if result.is_empty() { 1.0 } else { 1.0 - children_share };

        let my_decision = if self.get_length() / self.get_radius() < strategy.conic_ratio {
            let last_segment = self.segments.last().unwrap();
            let next_point = last_segment.end + (last_segment.end - last_segment.start);
            GrowthDecision::Longer(GrowLonger(next_point))
        } else {
            GrowthDecision::Myself
        };

        result.push( (my_decision, my_share) );

        result
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
                GrowthDecision::Longer(GrowLonger(direction)) if direction.y >= 0.0 => {
                    let last_segment = self.segments.last()
                        .expect("Empty branch, really?");
                    self.segments.push(Segment::new(last_segment.end, direction));
                    self.weight += new_material;
                }

                GrowthDecision::NewBranch(
                    GrowNewBranch{ direction, parent_segment_index }
                ) if direction.y >= 0.0 =>
                    {
                    let cur_segment = &mut self.segments[parent_segment_index];
                    if cur_segment.branch.is_some() {
                        panic!("GrowthDecision::NewBranch - already have a branch");
                    }
                    cur_segment.branch = Some(Box::new(
                        MLBranch::new_branch(
                            cur_segment.end,
                            direction,
                            parent_segment_index,
                            self.id.clone(),
                            new_material)));
                }

                GrowthDecision::Child(GrowChild(index)) =>
                    self.segments[index].branch
                        .as_mut()
                        .expect("GrowthDecision::Child - bad index")
                        .grow(new_material, soil, strategy),

                _ => self.weight += new_material,
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

#[cfg(test)]
mod test {



}

