use macroquad::prelude::*;

pub struct Object {
    id: u32,
    position: Vec3,
    mass: f32,
    radius: f32,
    color: Color,
}

pub struct ObjectPool {
    objects: Vec<Object>,
    current_id: u32,
}

impl Object {
    pub fn new(id: u32, position: Vec3, mass: f32, radius: f32, color: Color) -> Self {
        Object {
            id,
            position,
            mass,
            radius,
            color,
        }
    }

    pub fn clone_with_id(&self, id: u32) -> Self {
        Object {
            id,
            ..*self
        }
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

    pub fn draw(&self) {
        draw_sphere(self.position, self.radius, None, self.color);
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new(0, Vec3::default(), 1., 1., WHITE)
    }
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

    pub fn draw_all(&self) {
        for obj in &self.objects {
            obj.draw();
        }
    }
}
