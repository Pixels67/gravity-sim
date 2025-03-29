use crate::physics::PhysicsHandler;
use crate::renderer::Renderer;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Object {
    pub id: usize,
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
}

impl Object {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32, radius: f32, color: Color) -> Self {
        Object {
            id: 0,
            position,
            velocity,
            mass,
            radius,
            color,
        }
    }

    pub fn with_pos(position: Vec3) -> Self {
        Object {
            position,
            ..Default::default()
        }
    }

    pub fn with_id(id: usize) -> Self {
        Object {
            id,
            ..Default::default()
        }
    }

    pub fn clone_with_id(&self, id: usize) -> Self {
        Object {
            id,
            position: self.position,
            velocity: self.velocity,
            mass: self.mass,
            radius: self.radius,
            color: self.color,
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.position += translation;
        self
    }

    pub fn add_velocity(&mut self, velocity: Vec3) -> &mut Self {
        self.velocity += velocity;
        self
    }

    pub fn update_pos(&mut self, time: f32) -> &mut Self {
        self.position += self.velocity * time;
        self
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_sphere(self.position, self.radius, Some(self.color));
        renderer.draw_arrow(self.position, self.position.with_y(0.0), Some(self.color));
    }

    pub fn calculate_trajectory(
        &self,
        objects: &ObjectPool,
        physics_handler: &PhysicsHandler,
        point_count: u32,
        step: u32,
    ) -> Trajectory {
        let mut objects = objects.clone();
        let mut id = self.id;
        if objects.get(id).is_none() {
            id = objects.push(self.clone());
        }

        let mut trajectory = Trajectory::new();

        for i in 0..point_count * step {
            let time = physics_handler.get_timestep();
            physics_handler.update_objects(&mut objects, time);

            if i % step != 0 {
                continue;
            }

            let obj = objects.get(id);

            if obj.is_none() {
                trajectory.end();
                return trajectory;
            }

            let obj = obj.unwrap().clone();
            trajectory.push(obj.position);
        }

        trajectory
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new(Vec3::ZERO, Vec3::ZERO, 1., 1., WHITE)
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let mut obj = Object::new(
            self.position,
            self.velocity,
            self.mass,
            self.radius,
            self.color,
        );

        obj.id = self.id;
        obj
    }
}

#[derive(Default)]
pub struct ObjectPool {
    objects: Vec<Object>,
    current_id: usize,
}

impl ObjectPool {
    pub fn new() -> Self {
        ObjectPool {
            objects: Vec::new(),
            current_id: 0,
        }
    }

    pub fn push(&mut self, object: Object) -> usize {
        self.current_id += 1;
        self.objects.push(object.clone_with_id(self.current_id));
        self.current_id
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.objects.pop()
    }

    pub fn remove(&mut self, id: usize) {
        for i in 0..self.objects.len() {
            if self.objects[i].id != id {
                continue;
            }

            self.objects.swap_remove(i);
            return;
        }
    }

    pub fn get(&self, id: usize) -> Option<&Object> {
        self.objects.iter().find(|obj| obj.id == id)
    }

    pub fn get_all_in_area(&self, pos: Vec3, radius: f32) -> ObjectPool {
        ObjectPool {
            objects: self
                .objects
                .iter()
                .filter(|obj| (obj.position - pos).length() - obj.radius <= radius)
                .cloned()
                .collect(),
            current_id: self.current_id,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.iter_mut()
    }

    pub fn calculate_trajectories(
        &self,
        physics_handler: &PhysicsHandler,
        point_count: u32,
        step: u32,
    ) -> HashMap<usize, Trajectory> {
        let mut ids: Vec<usize> = Vec::new();
        let mut objects = self.clone();
        let mut trajectories: HashMap<usize, Trajectory> = HashMap::new();

        for obj in objects.iter_mut() {
            ids.push(obj.id);
        }

        for i in 0..point_count * step {
            let time = physics_handler.get_timestep();
            physics_handler.update_objects(&mut objects, time);

            if i % step != 0 {
                continue;
            }

            for id in ids.clone().iter() {
                if objects.iter().find(|o| o.id == *id).is_some() {
                    continue;
                }

                if let Some(traj) = trajectories.get_mut(id) {
                    traj.end();
                }

                ids.remove(ids.iter().position(|id| *id == *id).unwrap());
            }

            for obj in objects.iter_mut() {
                if i % step != 0 {
                    continue;
                }

                trajectories
                    .entry(obj.id)
                    .or_insert(Trajectory::new())
                    .push(obj.position);
            }
        }

        trajectories
    }

    pub fn draw_all(&self, renderer: &Renderer) {
        self.iter().for_each(|obj| {
            obj.draw(renderer);
        });
    }
}

impl Clone for ObjectPool {
    fn clone(&self) -> Self {
        let mut vec = Vec::default();
        for obj in &self.objects {
            vec.push(obj.clone());
        }

        ObjectPool {
            objects: vec,
            current_id: self.current_id,
        }
    }
}

#[derive(Clone)]
pub struct Trajectory {
    points: Vec<Vec3>,
    has_end: bool,
}

impl Trajectory {
    pub fn new() -> Self {
        Trajectory {
            points: Vec::new(),
            has_end: false,
        }
    }

    pub fn push(&mut self, point: Vec3) -> &mut Trajectory {
        self.points.push(point);
        self
    }

    pub fn end(&mut self) -> &mut Trajectory {
        self.has_end = true;
        self
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn first(&self) -> Option<&Vec3> {
        self.points.first()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec3> {
        self.points.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec3> {
        self.points.iter_mut()
    }

    pub fn draw(&self, renderer: &Renderer, color: Option<Color>, end_sphere_radius: f32) {
        for (i, point) in self.points.iter().enumerate() {
            if i == 0 {
                continue;
            }

            renderer.draw_line(self.points[i - 1], *point, color);

            if i == self.points.len() - 1 && self.has_end {
                renderer.draw_halo(*point, end_sphere_radius, Some(RED));
            }
        }
    }
}
