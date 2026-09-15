use macroquad::prelude::*;
use ::rand::random;
use std::f32::consts::*;
use crate::constants::*;
use crate::particle::*;
use crate::spatialhashing::*;


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

pub struct SysStruct {
    pub particles: [Particle; PARTICLE_COUNT as usize],
    particle_indexes: [usize; PARTICLE_COUNT as usize],
    particle_start_indexes: [usize; PARTICLE_COUNT as usize],
    particle_cell_keys: [usize; PARTICLE_COUNT as usize],
}
impl SysStruct {
    pub fn new(particle_count: u32) -> SysStruct {
        let mut particle_array: [Particle; PARTICLE_COUNT as usize] = std::array::from_fn(|_| Particle::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(X_LEFT_BOUNDARY + (random::<f32>() * (X_RIGHT_BOUNDARY - X_LEFT_BOUNDARY)), Y_BOTTOM_BOUNDARY + (random::<f32>() * (Y_TOP_BOUNDARY - Y_BOTTOM_BOUNDARY))), 
                Vec2::new(0.0, 0.0), 
                0.0, 
                0.0
            ));
        let particle_indexes: [usize; PARTICLE_COUNT as usize] = std::array::from_fn(|i| i);
        let particle_start_indexes: [usize; PARTICLE_COUNT as usize] = [0.0 as usize; PARTICLE_COUNT as usize];
        let particle_cell_keys: [usize; PARTICLE_COUNT as usize] = [0.0 as usize; PARTICLE_COUNT as usize];
        return SysStruct {
            particles : particle_array,
            particle_indexes : particle_indexes,
            particle_start_indexes : particle_start_indexes,
            particle_cell_keys : particle_cell_keys,

        };
    }
    pub fn update_cell_particles(&mut self) {
        //generate the cell keys
        //must loop through particles this way in order for particle indexes to sync with cell keys
        for i in 0..self.particle_indexes.len() {
            let particle_index = self.particle_indexes[i];
            let particle = self.particles[particle_index];
            let cell_pos = get_cell_pos(particle.position);

            self.particle_cell_keys[i] = get_cell_key(cell_pos);
        }

        //sort the arrays based on the cell keys with selection sort

        for i in 0..self.particles.len() {
            let mut lowest_value :usize = self.particle_cell_keys[i];
            let mut lowest_value_index: usize = i;
            for j in i..self.particles.len() {
                if self.particle_cell_keys[j] < lowest_value {
                    lowest_value = self.particle_cell_keys[j];
                    lowest_value_index = j;
                }
            }
            self.particle_cell_keys.swap(i, lowest_value_index);
            self.particle_indexes.swap(i, lowest_value_index);
        }

        //calculate the start indexes
        let mut current_index: usize = (PARTICLE_COUNT + 1) as usize;
        for i in 0..self.particles.len() {
            if (self.particle_cell_keys[i] != current_index) {
                self.particle_start_indexes[self.particle_cell_keys[i]] = i;
                current_index = self.particle_cell_keys[i];
            }
        }
    }
    pub fn clear_forces(&mut self) {
        for i in 0..self.particles.len() {
            let particle: &mut Particle = &mut self.particles[i];
            particle.force = Vec2::ZERO;
        }
    }
    pub fn apply_pull(&mut self, position: Vec2) {
        for i in 0..self.particles.len() {
            let particle: &mut Particle = &mut self.particles[i];
            let dist = position.distance(particle.position);

            if dist > MOUSE_EFFECT_RADIUS {
                continue;
            }
            let dir = (position - particle.position).normalize();
            let force = dir * MOUSE_PULL_STRENGTH;

            particle.force += force;
        }
    }
    pub fn update_densities(&mut self, delta_time: f32) {
        //update density and pressure
        for i in 0..self.particles.len() {
            let mut density: f32 = 0.0;
            //loop through only neighbouring cells with the same cell key
            let cell_pos = get_cell_pos(self.particles[i].position);
            for x_dif in -1..=1 {
                for y_dif in -1..=1 {
                    let new_cell_pos = Vec2::new(cell_pos.x + x_dif as f32, cell_pos.y + y_dif as f32);
                    let cell_key  = get_cell_key(new_cell_pos);
                    for j in self.particle_start_indexes[cell_key]..self.particles.len() {
                        if self.particle_cell_keys[j] != cell_key {
                            break;
                        }
                        let other_particle = self.particles[self.particle_indexes[j]];

                        //update density
                        let dist = self.particles[i].position.distance(other_particle.position);
                        density += smoothing_kernel(dist, SMOOTHING_RADIUS);
                    }
                }
            }
            let particle: &mut Particle = &mut self.particles[i];
            particle.density = density.max(0.0001);
            particle.pressure = (particle.density - TARGET_DENSITY) * PRESSURE_MULTIPLYER;
        }
    }
    pub fn update_force_with_prediction(&mut self, delta_time: f32) {
        for i in 0..self.particles.len(){
            let predicted_pos: Vec2 = self.particles[i].position + self.particles[i].velocity * delta_time;

            let mut pressure_force :Vec2 = Vec2::ZERO;
            let mut viscosity_force: Vec2 = Vec2::ZERO;

            //loop through only neighbouring cells with the same cell key
            let cell_pos = get_cell_pos(self.particles[i].position);
            for x_dif in -1..=1 {
                for y_dif in -1..=1 {
                    let new_cell_pos = Vec2::new(cell_pos.x + x_dif as f32, cell_pos.y + y_dif as f32);
                    let cell_key  = get_cell_key(new_cell_pos);
                    for j in self.particle_start_indexes[cell_key]..self.particles.len() {
                        if self.particle_cell_keys[j] != cell_key {
                            break;
                        }
                        let other_particle = self.particles[self.particle_indexes[j]];
                        let other_particle_predicted_pos = other_particle.position + other_particle.velocity * delta_time;

                        //update forces
                        let dist = predicted_pos.distance(other_particle_predicted_pos);
                        if dist > SMOOTHING_RADIUS || dist == 0.0 {
                            continue;
                        }
                        let pressure_direction = (predicted_pos - other_particle_predicted_pos) / dist;

                        let shared_pressure = (self.particles[i].pressure + other_particle.pressure) / 2.0;
                        let gradient = smoothing_kernel_derivative(dist, SMOOTHING_RADIUS);

                        pressure_force -= gradient* pressure_direction * shared_pressure / other_particle.density;

                        let velocity_dif = self.particles[i].velocity - other_particle.velocity;
                        let viscosity_weight = smoothing_kernel(dist, SMOOTHING_RADIUS);

                        viscosity_force += velocity_dif * viscosity_weight * VISCOSITY / other_particle.density;
                    }
                }
            }
            self.particles[i].force += pressure_force - viscosity_force;
        }
    }
    pub fn update_position(&mut self, delta_time: f32) {
        for i in 0..self.particles.len() {
            let mut particle: &mut Particle = &mut self.particles[i];
            particle.position += delta_time * particle.velocity;
        }
    }
    pub fn update_physics(&mut self, delta_time: f32) {
        self.update_cell_particles();
        self.update_densities(delta_time);
        self.update_force_with_prediction(delta_time);
        //update forces

        //update veloctiy and position
        for particle in &mut self.particles {
            let acceleration = particle.force / particle.density;

            particle.velocity += acceleration * delta_time;
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
