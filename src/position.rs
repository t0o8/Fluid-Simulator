use macroquad::prelude::*;
use crate::constants::*;

pub fn to_screen_pos(x: f32, y: f32) -> (f32, f32) {
    return (SCREEN_START_X + (x * WORLD_TO_SCREEN_CONVERSION_RATIO), screen_height()-((y * WORLD_TO_SCREEN_CONVERSION_RATIO) + SCREEN_START_Y))
}
pub fn to_world_pos(x: f32, y:f32) -> (f32, f32) {
    return ((x - SCREEN_START_X) / WORLD_TO_SCREEN_CONVERSION_RATIO, (-y + screen_height() - SCREEN_START_Y) / WORLD_TO_SCREEN_CONVERSION_RATIO)
}