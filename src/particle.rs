use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Particle {
    pub velocity: Vec2,
    pub position: Vec2,
    pub force: Vec2,
    
    pub density: f32,
    pub pressure: f32,
}
impl Particle {
    pub fn new(velocity: Vec2, position: Vec2, force: Vec2, density: f32, pressure: f32) -> Particle{
        return Particle {
            velocity : velocity,
            position : position,
            force: force,
            density: density,
            pressure: pressure
        }
    }
}