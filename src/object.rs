use macroquad::prelude::*;

#[derive(PartialEq)]
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

    pub fn new_with_pos(position: Vec3) -> Self {
        let mut obj = Object::default();
        obj.position = position;
        obj
    }

    pub fn clone_with_id(&self, id: usize) -> Self {
        Object { id, ..*self }
    }

    // Getters
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_color(&self) -> Color {
        self.color
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

    pub fn draw(&self, material: &Material) {
        gl_use_material(&material);

        material.set_uniform("color", self.color);
        material.set_uniform("world_pos", self.position);

        draw_sphere(self.position, self.radius, None, self.color);

        gl_use_default_material();

        #[cfg(debug_assertions)]
        draw_line_3d(self.position, self.position + (self.velocity * 100.), GREEN);

        draw_line_3d(
            self.position,
            vec3(self.position.x, 0., self.position.z),
            Color {
                r: 1.,
                g: 1.,
                b: 1.,
                a: 0.5,
            },
        );

        draw_sphere(
            vec3(self.position.x, 0., self.position.z),
            0.05,
            None,
            Color {
                r: 1.,
                g: 1.,
                b: 1.,
                a: 0.5,
            },
        );
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

    pub fn push(&mut self, object: Object) {
        self.current_id += 1;
        self.objects.push(object.clone_with_id(self.current_id));
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.objects.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.iter_mut()
    }

    pub fn draw_all(&self, material: &Material) {
        for obj in &self.objects {
            obj.draw(material);
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
