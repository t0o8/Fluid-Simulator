use macroquad::prelude::*;
use macroquad::shapes::*;

use crate::constants::*;

mod constants;

#[macroquad::main("Fluid Simulator")]
async fn main() {
    let mut fluid_struct: SysStruct = SysStruct::new(100);

    loop {
        //update physics
        let delta_time = get_frame_time();

        fluid_struct.update_physics(delta_time);

        clear_background(BLACK);
        for particle in &fluid_struct.particles{
            draw_circle(particle.position.x, screen_height() - particle.position.y, PARTICLE_SIZE, BLUE)
        }
        draw_line(X_RIGHT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_RIGHT_BOUNDARY, screen_height() - Y_TOP_BOUNDARY, 1.0, RED);
        draw_line(X_LEFT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_LEFT_BOUNDARY, screen_height() - Y_TOP_BOUNDARY, 1.0, RED);
        draw_line(X_RIGHT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_LEFT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, 1.0, RED);
        next_frame().await
    };
}

pub struct Particle {
    velocity: Vec2,
    position: Vec2,
    force: Vec2,
    
    density: f32,
    pressure: f32,
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



pub struct SysStruct {
    particles: Vec<Particle>,
}
impl SysStruct {
    pub fn new(particle_count: u32) -> SysStruct {
        let mut particle_vec: Vec<Particle> = Vec::new();
        for i in 1..=particle_count {
            let particle_distance_factor:f32 = 15.0; 
            particle_vec.push(Particle::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(500.0 + (i % 50) as f32 * particle_distance_factor, 300.0 + (i / 50) as f32 * particle_distance_factor), 
                Vec2::new(0.0, 0.0), 
                0.0, 
                0.0
            ));
        }
        return SysStruct {
            particles : particle_vec
        };
    }

    pub fn update_physics(&mut self, delta_time: f32) {
        for particle in &mut self.particles {

            particle.velocity.y += GRAVITY * delta_time;

            particle.position.x += particle.velocity.x * delta_time;
            particle.position.y += particle.velocity.y * delta_time;

            if particle.position.y < Y_BOTTOM_BOUNDARY {
                particle.velocity.y = -particle.velocity.y * DAMPENING;
                particle.position.y = Y_BOTTOM_BOUNDARY;
            } else if particle.position.y > Y_TOP_BOUNDARY {
                particle.velocity.y = -particle.velocity.y * DAMPENING;
                particle.position.y = Y_TOP_BOUNDARY;
            }

            if particle.position.x > X_LEFT_BOUNDARY {
                particle.velocity.x = -particle.velocity.x * DAMPENING;
                particle.position.x = X_LEFT_BOUNDARY;
            } else if particle.position.x < X_RIGHT_BOUNDARY {
                particle.velocity.x = -particle.velocity.x * DAMPENING;
                particle.position.x = X_RIGHT_BOUNDARY;
            }
        }
    }
}
