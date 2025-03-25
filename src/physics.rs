use macroquad::prelude::*;

pub fn get_grav_force(grav_const: f32, m1: f32, m2: f32, dist: Vec3) -> Vec3 {
    if dist.length_squared() == 0. { return Vec3::ZERO; }
    let force = grav_const * m1 * m2 / dist.length_squared();
    let dir = dist.normalize();
    dir * force
}

pub fn get_veloc(force: Vec3, mass: f32, time: f32) -> Vec3 {
    force / mass * time
}

pub fn get_displ(force: Vec3, mass: f32, time: f32) -> Vec3 {
    force / mass * time * time
}