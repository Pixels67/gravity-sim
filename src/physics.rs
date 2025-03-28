use crate::object::{Object, ObjectPool};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct PhysicsHandler {
    grav_const: f32,
    timestep: f32,
    accumulator: f32,
}

impl PhysicsHandler {
    pub fn new(grav_const: f32, timestep: f32) -> Self {
        Self {
            grav_const,
            timestep,
            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, objects: &mut ObjectPool, dt: f32) {
        self.accumulator += dt;
        while self.accumulator > self.timestep {
            self.update_objects(objects);

            self.accumulator -= self.timestep;
        }
    }

    pub fn update_objects(&mut self, mut objects: &mut ObjectPool) {
        let mut clone = objects.clone();

        for obj in clone.iter_mut() {
            obj.velocity += self.get_obj_veloc(obj, objects);
        }

        for obj in clone.iter_mut() {
            obj.update_pos();
        }

        *objects = clone;

        for obj in objects.clone().iter_mut() {
            self.handle_collisions(&mut objects, obj);
        }
    }

    pub fn handle_collisions(&mut self, objects: &mut ObjectPool, object: &mut Object) {
        for other in objects
            .get_all_in_area(object.position, object.radius)
            .iter_mut()
        {
            if object.id == other.id {
                continue;
            }

            let object_momentum = object.mass * object.velocity;
            let other_momentum = other.mass * other.velocity;
            let combined_mass = object.mass + other.mass;
            let combined_radius = object.radius + other.radius;

            let new_veloc = (object_momentum + other_momentum) / combined_mass;
            let largest: &Object = if object.mass > other.mass {
                object
            } else {
                other
            };

            let smallest: &Object = if object.mass <= other.mass {
                object
            } else {
                other
            };

            objects.push(Object::new(
                largest.position,
                new_veloc,
                combined_mass,
                combined_radius,
                Self::mix_color(largest.color, smallest.color, smallest.mass / largest.mass),
            ));

            objects.remove(object.id);
            objects.remove(other.id);
        }
    }

    pub fn get_obj_veloc(&self, object: &Object, objects: &ObjectPool) -> Vec3 {
        let mut force = Vec3::ZERO;

        for other in objects.iter() {
            force += self.get_grav_force(object.mass, other.mass, other.position - object.position);
        }

        Self::get_veloc(force, object.mass, self.timestep)
    }

    pub fn get_grav_force(&self, m1: f32, m2: f32, dist: Vec3) -> Vec3 {
        if dist.length_squared() == 0. {
            return Vec3::ZERO;
        }

        let force = self.grav_const * m1 * m2 / dist.length_squared();
        let dir = dist.normalize();
        dir * force
    }

    pub fn get_veloc(force: Vec3, mass: f32, time: f32) -> Vec3 {
        if mass == 0. || time == 0. {
            return Vec3::ZERO;
        }

        force / mass * time
    }

    pub fn get_displ(force: Vec3, mass: f32, time: f32) -> Vec3 {
        if mass == 0. || time == 0. {
            return Vec3::ZERO;
        }

        force / mass * time * time
    }

    pub fn get_timestep(&self) -> f32 {
        self.timestep
    }

    fn mix_color(c1: Color, c2: Color, factor: f32) -> Color {
        Color {
            r: c1.r * (1. - factor) + c2.r * factor,
            g: c1.g * (1. - factor) + c2.g * factor,
            b: c1.b * (1. - factor) + c2.b * factor,
            a: c1.a * (1. - factor) + c2.a * factor,
        }
    }
}

impl Default for PhysicsHandler {
    fn default() -> PhysicsHandler {
        PhysicsHandler {
            grav_const: 1.0,
            timestep: 0.01,
            accumulator: 0.,
        }
    }
}
