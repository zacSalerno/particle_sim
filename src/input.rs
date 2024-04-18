use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use super::constants::*;
use super::particle::*;


pub fn spawn_particles(rl: &RaylibHandle, particles: &mut Vec<Particle> ) {
    if rl.get_mouse_wheel_move() == 1.0{
        for _ in 0..AMOUNT_OF_PARTICLES_TO_DELETE{
            make_particle(particles);
        }
    } else if rl.get_mouse_wheel_move() == -1.0 {
        for _ in 0..AMOUNT_OF_PARTICLES_TO_DELETE{
            particles.pop();
        }
    }
}

pub fn trigger_gravity_points (rl: &RaylibHandle, gravity_point_enabled: bool) -> bool {

    if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
        return !gravity_point_enabled
    } else {
        return gravity_point_enabled
    }
}

pub fn gravity_point_maker (rl: &RaylibHandle, gravity_points: &mut Vec<GravityPoint>, position: Vector2) {

    if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
        make_gravity_point(gravity_points, Some(position));
    }
}

