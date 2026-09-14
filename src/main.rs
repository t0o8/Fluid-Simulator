mod constants;
mod particle;
mod position;
mod sysstruct;
mod spatialhashing;

use macroquad::prelude::*;

use crate::constants::*;
use crate::particle::*;
use crate::position::*;
use crate::sysstruct::*;

#[macroquad::main("Fluid Simulator")]
async fn main() {
    let mut fluid_struct: SysStruct = SysStruct::new(PARTICLE_COUNT);

    loop {
        clear_background(BLACK);
        
        //reset forces from previous frame
        fluid_struct.clear_forces();

        //apply mouse effects
        let mouse_world_pos = to_world_pos(mouse_position().0, mouse_position().1);
        if is_mouse_button_down(MouseButton::Left) {
            fluid_struct.apply_pull(Vec2::new(mouse_world_pos.0, mouse_world_pos.1));
            draw_circle_lines(mouse_position().0, mouse_position().1, MOUSE_EFFECT_RADIUS * WORLD_TO_SCREEN_CONVERSION_RATIO, 2.0, PURPLE);
        }


        //update physics
        let delta_time = get_frame_time();
        fluid_struct.update_physics(delta_time);



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

        //write fps
        let fps = 1.0/delta_time;
        draw_text(fps.to_string(), 10.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    };
}
