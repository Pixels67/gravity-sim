use crate::physics::PhysicsHandler;
use crate::renderer::Renderer;
use macroquad::prelude::*;

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

    pub fn update_pos(&mut self) -> &mut Self {
        self.position += self.velocity;
        self
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_sphere(self.position, self.radius, Some(self.color));
        renderer.draw_arrow(self.position, self.position.with_y(0.0), Some(self.color));
    }

    pub fn draw_path(
        &self,
        objects: &ObjectPool,
        renderer: &Renderer,
        physics_handler: &PhysicsHandler,
        point_count: u32,
        skip: u32,
    ) {
        let mut physics_handler = physics_handler.clone();

        let mut objects = objects.clone();
        let mut id = self.id;
        if objects.get(id).is_none() {
            id = objects.push(self.clone());
        }

        let initial_pos = self.position;
        let mut pos = self.position;

        for i in 0..point_count {
            physics_handler.update_objects(&mut objects);

            let obj = objects.get(id);

            if obj.is_none() {
                draw_sphere(pos, self.radius * 1.01, None, RED);
                break;
            }

            if i % skip == 0 {
                renderer.draw_line(pos, obj.unwrap().position, Some(self.color));
                pos = obj.unwrap().position;
            }

            if (pos - initial_pos).length() > 1000.0 {
                break;
            }
        }
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

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.iter_mut()
    }

    pub fn draw_all(&self, renderer: &Renderer) {
        for obj in &self.objects {
            obj.draw(renderer);
        }
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
