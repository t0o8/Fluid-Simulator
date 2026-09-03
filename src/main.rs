use macroquad::prelude::*;
use macroquad::shapes::*;
use ::rand::random;
use std::f64::consts::*;

use crate::constants::*;

mod constants;

#[macroquad::main("Fluid Simulator")]
async fn main() {
    let mut fluid_struct: SysStruct = SysStruct::new(PARTICLE_COUNT);

    loop {
        //update physics
        let delta_time = get_frame_time();
        fluid_struct.update_physics(delta_time);
        
        clear_background(BLACK);


        let mut total_density:f32 = 0.0;

        for particle in &fluid_struct.particles{
            let mut color: Color = Color {r: 0.0, g: 0.0, b:0.0, a:0.0};
            if particle.density > TARGET_DENSITY * 1.001 {
                color = Color {r: 1.0, g: 0.0, b:0.0, a:3.0};
            } else if particle.density < TARGET_DENSITY * 0.999{
                color = Color {r: 0.0, g: 0.0, b:1.0, a:3.0};
            } else {
                color = Color {r: 1.0, g: 1.0, b:1.0, a:3.0};
            }
            draw_circle(particle.position.x, screen_height() - particle.position.y, PARTICLE_SIZE, color);
            if ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5) == 0.0) {
                //avoid dividing by zero 
                continue;
            }
            let line_x_dir:f32 = 10.0 * particle.force.x / ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5));
            let line_y_dir:f32 = 10.0 * particle.force.y / ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5));
            let start_pos: (f32, f32) = to_screen_pos(particle.position.x, particle.position.y);
            let end_pos: (f32, f32) = to_screen_pos(particle.position.x + line_x_dir, particle.position.y + line_y_dir);

            draw_line(start_pos.0, start_pos.1, end_pos.0, end_pos.1, 1.0, PURPLE);

            if particle.position.distance(Vec2::new(mouse_position().0, screen_height() - mouse_position().1)) < SMOOTHING_RADIUS {
                total_density += particle.density;
            }
        }
        draw_line(X_RIGHT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_RIGHT_BOUNDARY, screen_height() - Y_TOP_BOUNDARY, 1.0, RED);
        draw_line(X_LEFT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_LEFT_BOUNDARY, screen_height() - Y_TOP_BOUNDARY, 1.0, RED);
        draw_line(X_RIGHT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, X_LEFT_BOUNDARY, screen_height() - Y_BOTTOM_BOUNDARY, 1.0, RED);

        //draw_circle_lines(mouse_position().0, mouse_position().1, SMOOTHING_RADIUS, 2.0, PURPLE);
        next_frame().await
    };
}

fn to_screen_pos(x: f32, y: f32) -> (f32, f32) {
    return (x, screen_height()-y)
}
fn smoothing_kernel(distance: f32, radius: f32) -> f32 {
    if distance >= radius { return 0.0; }
    let volume = PI as f32* radius.powi(5) / 10.0;
    (radius - distance).powi(3) / volume
}
fn smoothing_kernel_derivative(r: f32, h: f32) -> f32 {
    if r >= h || r == 0.0 { return 0.0; }
    let scale = 30.0 / (PI as f32 * h.powi(5));
    -scale * (h - r).powi(2)
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
                Vec2::new(X_LEFT_BOUNDARY + (random::<f32>() * (X_RIGHT_BOUNDARY - X_LEFT_BOUNDARY)), Y_BOTTOM_BOUNDARY + (random::<f32>() * (Y_TOP_BOUNDARY - Y_BOTTOM_BOUNDARY))), 
                Vec2::new(0.0, 0.0), 
                0.0, 
                0.0
            ));
            //Vec2::new(PARTICLES_START_POS_X + (i % 50) as f32 * particle_distance_factor, PARTICLES_START_POS_Y + (i / 50) as f32 * particle_distance_factor), 
        }
        return SysStruct {
            particles : particle_vec
        };
    }

    pub fn update_physics(&mut self, delta_time: f32) {
        //update density and pressure
        for i in 0..self.particles.len() {
            let mut density: f32 = 0.0;
            for j in 0..self.particles.len() {
                let dist = self.particles[i].position.distance(self.particles[j].position);
                density += smoothing_kernel(dist, SMOOTHING_RADIUS);
            }
            let particle: &mut Particle = &mut self.particles[i];
            particle.density = density.max(0.0001); 
            particle.pressure = (particle.density - TARGET_DENSITY) * PRESSURE_MULTIPLYER;
        }

        //update forces
        for i in 0..self.particles.len() {
            let mut pressure_force :Vec2 = Vec2::ZERO;
            for j in 0..self.particles.len() {
                if i == j {continue;}

                let dist = self.particles[i].position.distance(self.particles[j].position);
                //don't interact with particles right next to each other or those outside of smoothing radius
                if dist > SMOOTHING_RADIUS || dist == 0.0 {
                    continue;
                }
                let pressure_direction = (self.particles[i].position - self.particles[j].position) / dist;

                let shared_pressure = (self.particles[i].pressure + self.particles[j].pressure) / 2.0;
                let gradient = smoothing_kernel_derivative(dist, SMOOTHING_RADIUS);

                pressure_force -= gradient* pressure_direction * shared_pressure / self.particles[j].density;
            }

            self.particles[i].force = pressure_force;
        }


        for particle in &mut self.particles {
            let accerlation = particle.force / particle.density;

            particle.velocity += accerlation * delta_time;
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

            if particle.position.x < X_LEFT_BOUNDARY {
                particle.velocity.x = -particle.velocity.x * DAMPENING;
                particle.position.x = X_LEFT_BOUNDARY;
            } else if particle.position.x > X_RIGHT_BOUNDARY {
                particle.velocity.x = -particle.velocity.x * DAMPENING;
                particle.position.x = X_RIGHT_BOUNDARY;
            }
        }
    }
}
