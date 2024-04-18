use raylib::prelude::*;
use rand::prelude::*;

use super::vector_fn::*;
use super::constants::*;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub can_attract: bool,
    pub closest_gravity_point: GravityPoint
}

#[derive(Debug, Copy, Clone)]
pub struct GravityPoint {
    pub position: Vector2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub can_attract: bool,
}

impl Default for GravityPoint {
    fn default() -> GravityPoint{
        GravityPoint {
            position: Vector2::new(0.0, 0.1),
            mass: 0.0,
            radius: 5.0,
            color: Color::from_hex("afffff").unwrap(),
            can_attract: true,
        }
    }
}

pub fn apply_force(particle: &mut Particle, force: Vector2){
    //particle.acceleration = force;
    particle.acceleration = force / particle.mass;

}

pub fn attract(attractor: Option<&GravityPoint>, mover: &mut Particle){

    match attractor {
        Some(attractor) => {
            let mut force = vector_sub(attractor.position, mover.position);
            let distance = constrain(force.length(), MIN_CONSTRAIN_LENGTH, MAX_CONSTRAIN_LENGTH);
            let strength: f32 = ((attractor.mass * mover.mass) / f32::powf(distance, 2.0)) * GRAVITATIONAL_CONSTANT;
            force = set_magnitude(force, strength);
            apply_force(mover, force);
        }
        None => {
            let mut force = vector_sub(mover.closest_gravity_point.position, mover.position);
            let distance = constrain(force.length(), MIN_CONSTRAIN_LENGTH, MAX_CONSTRAIN_LENGTH);
            let strength: f32 = ((mover.closest_gravity_point.mass * mover.mass) / f32::powf(distance, 2.0)) * GRAVITATIONAL_CONSTANT;
            force = set_magnitude(force, strength);
            apply_force(mover, force);
        }
    }

}

pub fn make_particle(particles: &mut Vec<Particle>){

    let mut rng = thread_rng();

    let random_position: (f32, f32) = (rng.gen_range(100.0..=SCREEN_WIDTH - 100.0), rng.gen_range(100.0..=SCREEN_HEIGHT - 100.0));
    let _random_velocity: (f32, f32) = (rng.gen_range(-PARICLE_VELOCITY_LIMIT..=PARICLE_VELOCITY_LIMIT), rng.gen_range(-PARICLE_VELOCITY_LIMIT..=PARICLE_VELOCITY_LIMIT));
    let _random_mass: f32 = rng.gen_range(15.0..=50.0);

    let particle = Particle {
        position: Vector2::new(random_position.0, random_position.1),
        velocity: Vector2::new(0.0, 0.0),
        //velocity: Vector2::new(_random_velocity.0, _random_velocity.1),
        acceleration: Vector2::new(0.0, 0.0),
        //mass: 20.0,
        mass: _random_mass,
        radius: PARTICLE_RADIUS,
        color: Color::from_hex("ffffff").unwrap(),
        can_attract: false,
        closest_gravity_point: GravityPoint::default()
    };
    particles.push(particle);
}

pub fn make_gravity_point(gravity_points: &mut Vec<GravityPoint>, position: Option<Vector2>){
    
    match position {
        Some(position) => {
            let mut rng = thread_rng();

            let _random_mass: f32 = rng.gen_range(15.0..=50.0);

            let gravity_point = GravityPoint {
                position: position,
                mass: 500.0,
                radius: 5.0,
                color: Color::from_hex("afffff").unwrap(),
                can_attract: true,
            };
            gravity_points.push(gravity_point);

        }
        None => {
            let mut rng = thread_rng();

            let random_position: (f32, f32) = (rng.gen_range(100.0..=SCREEN_WIDTH - 100.0), rng.gen_range(100.0..=SCREEN_HEIGHT - 100.0));
            let _random_mass: f32 = rng.gen_range(15.0..=50.0);

            let gravity_point = GravityPoint {
                position: Vector2::new(random_position.0, random_position.1),
                mass: 500.0,
                radius: 5.0,
                color: Color::from_hex("afffff").unwrap(),
                can_attract: true,
            };
            gravity_points.push(gravity_point);
        }
    }
}
