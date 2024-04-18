use raylib::prelude::*;

mod constants;
mod input;
mod particle;
mod vector_fn;

use constants::*;
use input::*;
use particle::*;
use vector_fn::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .build();

    let mut particles: Vec<Particle> = Vec::new();

    let mut gravity_points: Vec<GravityPoint> = Vec::new();

    let mut gravity_point_enabled: bool = true;

    for _ in 1..=AMOUNT_OF_PARTICLES {
        make_particle(&mut particles);
    }

    if AMOUNT_OF_GRAVITY_POINTS > 0 {
        for _ in 1..=AMOUNT_OF_GRAVITY_POINTS {
            make_gravity_point(&mut gravity_points, None);
        }
    }

    while !rl.window_should_close() {
        /* --- Update --- */

        let mouse_pos: Vector2 = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);

        for particle in 0..particles.len() {
            if gravity_points.len() > 0 {
                let mut closest_gravity_points: Vec<f32> = Vec::new();

                for gravity_point in 0..gravity_points.len() {
                    let closest_gravity_point = particles[particle]
                        .position
                        .distance_to(gravity_points[gravity_point].position);
                    closest_gravity_points.push(closest_gravity_point);
                }

                let mut min = closest_gravity_points[0];
                let mut min_index: usize = 0;

                for index in 0..closest_gravity_points.len() {
                    if closest_gravity_points[index] < min {
                        min = closest_gravity_points[index];
                        min_index = index;
                    }
                }

                particles[particle].closest_gravity_point = gravity_points[min_index];
            }

            attract(None, &mut particles[particle]);
            particles[particle].velocity =
                set_limit(particles[particle].velocity, PARICLE_VELOCITY_LIMIT);

            particles[particle].velocity = vector_add(
                particles[particle].velocity,
                particles[particle].acceleration,
            );
            particles[particle].position =
                vector_add(particles[particle].position, particles[particle].velocity);
        }

        /* --- Input ---*/
        spawn_particles(&rl, &mut particles);
        gravity_point_enabled = trigger_gravity_points(&rl, gravity_point_enabled);
        gravity_point_maker(&rl, &mut gravity_points, mouse_pos);

        /* --- Window Settings --- */
        rl.set_target_fps(120);
        rl.set_window_position(
            1920 + (2560 / 2) - SCREEN_WIDTH as i32 / 2,
            1080 / 2 - SCREEN_HEIGHT as i32 / 2,
        );

        /* --- Draw --- */
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);
        //println!("x : {:?}, y : {:?}", particle.velocity.x, particle.velocity.y);
        for index in 0..gravity_points.len() {
            if gravity_point_enabled {
                draw.draw_circle_v(
                    gravity_points[index].position,
                    gravity_points[index].radius,
                    gravity_points[index].color,
                );
            }
        }
        for index in 0..particles.len() {
            draw.draw_circle_v(
                particles[index].position,
                particles[index].radius,
                particles[index].color,
            );
        }
    }
}
