use macroquad::prelude::*;
use crate::constants::*;

pub fn get_cell_pos(position :Vec2) -> Vec2 {
    return Vec2::new(f32::floor(position.x / SMOOTHING_RADIUS), f32::ceil(position.y / SMOOTHING_RADIUS));
}
pub fn get_cell_world_pos(position : Vec2) -> Vec2 {
    return Vec2::new(position.x * SMOOTHING_RADIUS, position.y * SMOOTHING_RADIUS);
}
pub fn get_cell_key(cell_pos : Vec2) -> usize{
    //x and y should be coprime
    let x_hash_multiplier : u32 = 1259;
    let y_hash_multiplier : u32 = 3109;

    return ((cell_pos.x as u32 * x_hash_multiplier + cell_pos.y as u32* y_hash_multiplier)% PARTICLE_COUNT) as usize;
}