use glam::{Vec2, vec2};
use macroquad::color::{BEIGE, BLUE, BROWN, Color, DARKBROWN, DARKGREEN, GRAY, GREEN, SKYBLUE};
use macroquad::input::mouse_position;
use macroquad::prelude::{clear_background, draw_line, draw_poly_lines, draw_rectangle, screen_height, screen_width};
use crate::model::branch::{Branch, GrowthDecision, MLBranch};
use crate::model::plant::Plant;
use crate::model::Resource;
use crate::model::soil::{MatrixSoil, Soil};
use crate::numeric::distance_to_segment;

pub const SOIL_LEVEL: f32 = 50.0;


pub fn draw_scene(plants: &Vec<Plant>, soil: &MatrixSoil) {

    clear_background(DARKBROWN);
    draw_rectangle(0.0, 0.0, screen_width(), SOIL_LEVEL - 1.0, SKYBLUE);

    let mouse_pos: Vec2 = mouse_position().into();
    let mouse_pos = vec2(mouse_pos.x, mouse_pos.y - SOIL_LEVEL);

    let mut hover_drawn = false;

    for plant in plants.iter() {
        draw_branch(&plant.root, mouse_pos, &mut hover_drawn);
        let decision = plant.root.growth_decision(&soil, 1.0, &plant.strategy);
        draw_decision(plant.root.segments[0].start.x, decision);
    }

    let max_y = (screen_height() - SOIL_LEVEL) as i32;

    for x in (0..screen_width() as i32).step_by(20) {
        for y in (0..max_y).step_by(10) {
            let pos = vec2(x as f32, y as f32);
            let water = soil.get_resource(pos, Resource::Water);
            let nitro = soil.get_resource(pos, Resource::Nitro);

            if water > 0.0 {
                let size = resource_draw_size(water);
                draw_poly_lines(pos.x, pos.y + SOIL_LEVEL, 3, size, 0.0, 1.0, BLUE);
            }
            if nitro > 0.0 {
                let size = resource_draw_size(nitro);
                draw_poly_lines(pos.x + 5.0, pos.y + 2.0 + SOIL_LEVEL, 4, size, 0.0, 1.0, GRAY);
            }
        }

    }
}

fn draw_branch(branch: &MLBranch, mouse_pos: Vec2, hover_drawn: &mut bool) {
    let mut color = BEIGE;

    if ! *hover_drawn {
        let d_mouse = distance_to_segment(
            mouse_pos,
            branch.segments[0].start,
            branch.segments.last().unwrap().end);

        if d_mouse < 5.0 {
            color = GREEN;
            *hover_drawn = true;
        }
    }

    for (i, segment) in branch.segments.iter().enumerate() {
        if let Some(left) = &segment.branch {
            draw_branch(left, mouse_pos, hover_drawn);
        }

        let thickness = 7.0 * (branch.get_length() - i as f32) / branch.get_length();
        draw_line(
            segment.start.x,
            segment.start.y + SOIL_LEVEL,
            segment.end.x,
            segment.end.y + SOIL_LEVEL,
            1.0 + thickness,
            color);
    }
}

fn resource_draw_size(quantity: f32) -> f32 {
    if quantity < 0.0 {
        0.0
    } else if quantity < 2.9 {
        quantity * 2.0
    } else {
        quantity.ln() + 3.0
    }
}

fn draw_decision(x: f32, decisions: Vec<(GrowthDecision, f32)>) {
    let mut long = 0.0;
    let mut branches = 0.0;
    let mut new_branches = 0.0;
    let mut thick = 0.0;
    for (d, weight) in decisions.iter() {
        match d {
            GrowthDecision::Longer(_) => long += weight,
            GrowthDecision::Child(_) => branches += weight,
            GrowthDecision::NewBranch(_) => new_branches += weight,
            GrowthDecision::Myself => thick += weight,
        }
    }

    let rect_width = 15.0;
    let rect_height = 30.0;

    fn draw_bar(x: f32, height: f32, color: Color) {
        let offset = 10.0;
        let rect_width = 15.0;

        draw_rectangle(
            x,
            SOIL_LEVEL - offset - height,
            rect_width,
            height + 1.0,
            color);
    }

    draw_bar(x - rect_width * 1.6, long * rect_height, BROWN);
    draw_bar(x - rect_width * 0.5, branches * rect_height, DARKGREEN);
    draw_bar(x + rect_width * 0.6, thick * rect_height, DARKBROWN);
    draw_bar(x + rect_width * 1.7, new_branches * rect_height, DARKGREEN);
    if long + branches + thick + new_branches < 0.99 {
        println!("Not enough weight!");
    }
}