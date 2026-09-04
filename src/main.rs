use macroquad::prelude::*;
use macroquad::shapes::*;
use ::rand::random;
use std::cell;
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
            let mut color: Color = Color {r: 1.0, g: 0.0, b:0.0, a:1.0};
            if particle.density > TARGET_DENSITY * 1.001 {
                color = Color {r: 1.0, g: 0.0, b:0.0, a:3.0};
            } else if particle.density < TARGET_DENSITY * 0.999{
                color = Color {r: 0.0, g: 0.0, b:1.0, a:3.0};
            } else {
                color = Color {r: 1.0, g: 1.0, b:1.0, a:3.0};
            } 
            
            let particle_screen_pos = to_screen_pos(particle.position.x, particle.position.y);
            draw_circle(particle_screen_pos.0, particle_screen_pos.1, PARTICLE_SIZE, color);

            if particle.position.distance(Vec2::new(mouse_position().0, screen_height() - mouse_position().1)) < SMOOTHING_RADIUS {
                total_density += particle.density;
            }

            if ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5) == 0.0) {
                //avoid dividing by zero 
                continue;
            }
            let line_x_dir:f32 = 3.0 * particle.force.x / ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5));
            let line_y_dir:f32 = 3.0 * particle.force.y / ((particle.force.x.powi(2) + particle.force.y.powi(2)).powf(0.5));
            let start_pos: (f32, f32) = to_screen_pos(particle.position.x, particle.position.y);
            let end_pos: (f32, f32) = to_screen_pos(particle.position.x + line_x_dir, particle.position.y + line_y_dir);

            //draw force indicator
            //draw_line(start_pos.0, start_pos.1, end_pos.0, end_pos.1, 1.0, PURPLE);
        }

        //draw border lines
        let bottom_right_screen_pos = to_screen_pos(X_RIGHT_BOUNDARY, Y_BOTTOM_BOUNDARY);
        let bottom_left_screen_pos = to_screen_pos(X_LEFT_BOUNDARY, Y_BOTTOM_BOUNDARY);
        let top_right_screen_pos = to_screen_pos(X_RIGHT_BOUNDARY, Y_TOP_BOUNDARY);
        let top_left_screen_pos = to_screen_pos(X_LEFT_BOUNDARY, Y_TOP_BOUNDARY);
        
        draw_line(bottom_right_screen_pos.0,  bottom_right_screen_pos.1, top_right_screen_pos.0, top_right_screen_pos.1, 1.0, RED);
        draw_line(bottom_left_screen_pos.0,  bottom_left_screen_pos.1, top_left_screen_pos.0, top_left_screen_pos.1, 1.0, RED);
        draw_line(bottom_left_screen_pos.0,  bottom_left_screen_pos.1, bottom_right_screen_pos.0,  bottom_right_screen_pos.1, 1.0, RED);

        draw_circle_lines(mouse_position().0, mouse_position().1, SMOOTHING_RADIUS * WORLD_TO_SCREEN_CONVERSION_RATIO, 2.0, PURPLE);

        let mouse_world_pos = to_world_pos(mouse_position().0, mouse_position().1);
        next_frame().await
    };
}

fn to_screen_pos(x: f32, y: f32) -> (f32, f32) {
    return (SCREEN_START_X + (x * WORLD_TO_SCREEN_CONVERSION_RATIO), screen_height()-((y * WORLD_TO_SCREEN_CONVERSION_RATIO) + SCREEN_START_Y))
}
fn to_world_pos(x: f32, y:f32) -> (f32, f32) {
    return ((x - SCREEN_START_X) / WORLD_TO_SCREEN_CONVERSION_RATIO, (-y + screen_height() - SCREEN_START_Y) / WORLD_TO_SCREEN_CONVERSION_RATIO)
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

#[derive(Copy, Clone)]
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

fn get_cell_pos(position :Vec2) -> Vec2 {
    return Vec2::new(f32::floor(position.x / SMOOTHING_RADIUS), f32::ceil(position.y / SMOOTHING_RADIUS));
}
fn get_cell_world_pos(position : Vec2) -> Vec2 {
    return Vec2::new(position.x * SMOOTHING_RADIUS, position.y * SMOOTHING_RADIUS);
}
fn get_cell_key(cell_pos : Vec2) -> usize{
    //x and y should be coprime
    let x_hash_multiplier : u32 = 1259;
    let y_hash_multiplier : u32 = 3109;

    return ((cell_pos.x as u32 * x_hash_multiplier + cell_pos.y as u32* y_hash_multiplier)% PARTICLE_COUNT) as usize;
}

pub struct SysStruct {
    particles: [Particle; PARTICLE_COUNT as usize],
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
    pub fn update_physics(&mut self, delta_time: f32) {
        self.update_cell_particles();

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
        //update forces

        for i in 0..self.particles.len() {
            let mut pressure_force :Vec2 = Vec2::ZERO;
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
                        if dist > SMOOTHING_RADIUS || dist == 0.0 {
                            continue;
                        }
                        let pressure_direction = (self.particles[i].position - other_particle.position) / dist;

                        let shared_pressure = (self.particles[i].pressure + other_particle.pressure) / 2.0;
                        let gradient = smoothing_kernel_derivative(dist, SMOOTHING_RADIUS);

                        pressure_force -= gradient* pressure_direction * shared_pressure / other_particle.density;
                    }
                }
            }
            self.particles[i].force = pressure_force;
        }

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
