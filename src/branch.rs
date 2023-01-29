use glam::{Vec2, vec2};
use crate::{DumbSoil, Resource, Soil};

pub enum Angle {
    Left, Right, Middle,
}

pub struct Branch {
    pub start: Vec2,
    pub end: Vec2,
    direction: Angle,
    pub branch_weight: f32,
    children_weight: f32,
    pub left: Option<Box<Branch>>,
    pub right: Option<Box<Branch>>,
}

pub struct ResourceOnBranch<'res> {
    level: f32,
    point: Vec2,
    branch: &'res mut Branch,
}

impl Branch {

    pub fn new_vertical(x_coord: f32, length: f32) -> Self {
        Branch {
            start: vec2(x_coord, 0.),
            end: vec2(x_coord, length),
            direction: Angle::Middle,
            branch_weight: 1.0,
            children_weight: 0.0,
            left: None,
            right: None,
        }
    }

    /// returns a tuple: `( resource level, point, and which branch point belongs to)`.
    /// Maybe make it a struct?..
    fn find_best_point(&mut self, soil: &DumbSoil, need: Resource) -> Option<ResourceOnBranch> {

        // let start = ResourceOnBranch {
        //     level: soil.get_resource(self.start, need),
        //     point: self.start,
        //     branch: self,
        // };
        // Some(start)

        let end = ResourceOnBranch {
            level: soil.get_resource(self.end, need),
            point: self.end,
            branch: self,
        };
        Some(end)
        //
        // let middle: Vec2 = (self.start + self.end) * 0.5;
        // let middle = ResourceOnBranch {
        //     level: soil.get_resource(middle, need),
        //     point: self.start,
        //     branch: self,
        // };
        //
        // // if let Some(left) = self.left.as_ref() {
        // //
        // // }
        // return end;
    }

    pub fn grow(&mut self, soil: &DumbSoil, need: Resource) {
        // TODO: select growth point by the best amount of the resource.

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
    }
}

