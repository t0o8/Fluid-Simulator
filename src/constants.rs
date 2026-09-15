pub const PARTICLES_START_POS_X: f32 = 200.0;
pub const PARTICLES_START_POS_Y: f32 = 200.0;

pub const PARTICLE_SIZE:f32 = 4.0;
pub const PARTICLE_COUNT:u32 = 5000;

pub const GRAVITY: f32 = -19.8;

pub const DAMPENING:f32 = 0.2;
pub const SMOOTHING_RADIUS: f32 = 2.0;
pub const TARGET_DENSITY: f32 = 1.5;
pub const PRESSURE_MULTIPLYER: f32 = 3000.0;
pub const VISCOSITY: f32 = 15.0;

pub const SCREEN_START_X:f32 = 100.0; // given in pixels
pub const SCREEN_START_Y:f32 = 100.0; // given in pixels
pub const WORLD_TO_SCREEN_CONVERSION_RATIO:f32 = 10.0;

//should be zero or greater
pub const X_RIGHT_BOUNDARY: f32 = 180.0;
pub const X_LEFT_BOUNDARY: f32 = 0.0;
pub const Y_BOTTOM_BOUNDARY: f32 = 0.0;
pub const Y_TOP_BOUNDARY: f32 = 85.0;

pub const MOUSE_EFFECT_RADIUS: f32 = 10.0;
pub const MOUSE_PULL_STRENGTH: f32 = 300.0;