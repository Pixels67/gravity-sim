use super::object::*;
use super::physics;
use macroquad::prelude::*;
use std::vec;

pub struct World {
    pub grav_const: f32,
    pub cam: Camera3D,
    pub obj_mat: Material,
    objects: ObjectPool,
}

impl World {
    pub fn new(grav_const: f32) -> Self {
        let cam = Camera3D {
            position: vec3(15., 10., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };

        let shader = ShaderSource::Glsl {
            vertex: DEFAULT_VERT_SHADER,
            fragment: DEFAULT_FRAG_SHADER,
        };

        let params = MaterialParams {
            uniforms: vec![
                UniformDesc::new("light_pos", UniformType::Float3),
                UniformDesc::new("ambient_light", UniformType::Float1),
                UniformDesc::new("color", UniformType::Float4),
                UniformDesc::new("world_pos", UniformType::Float3),
            ],
            ..Default::default()
        };

        let obj_mat = load_material(shader, params).unwrap();
        obj_mat.set_uniform("light_pos", vec3(0., 10., 10.));
        obj_mat.set_uniform("ambient_light", 0.3f32);
        obj_mat.set_uniform("ambient_color", BLACK);

        World {
            grav_const,
            cam,
            obj_mat,
            objects: ObjectPool::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_object(&mut self, id: usize) -> Result<&mut Object, String> {
        self.objects
            .iter_mut()
            .find(|obj| obj.id == id)
            .ok_or(format!("Object with ID {} not found", id))
    }

    pub fn set_light_pos(&mut self, pos: Vec3) {
        self.obj_mat.set_uniform("light_pos", pos);
    }

    pub fn set_ambient_light(&mut self, ambient_light: f32) {
        self.obj_mat.set_uniform("ambient_light", ambient_light);
    }

    pub fn update(&mut self, dt: f32) {
        let mut objects = self.objects.clone();
        for obj in objects.iter_mut() {
            obj.add_velocity(self.get_obj_veloc(obj, dt));
        }

        self.objects = objects;
        self.objects.iter_mut().for_each(|obj| { obj.update_pos(); })
    }

    pub fn draw_all(&self) {
        set_camera(&self.cam);
        self.objects.draw_all(&self.obj_mat);
        set_camera(&self.cam);
    }

    fn get_obj_veloc(&self, object: &Object, dt: f32) -> Vec3 {
        let mut veloc: Vec3 = Vec3::default();

        for other in self.objects.iter() {
            let dist = other.position - object.position;
            let force = physics::get_grav_force(self.grav_const, object.mass, other.mass, dist);
            veloc += physics::get_veloc(force, object.mass, dt);
        }

        veloc
    }
}

const DEFAULT_VERT_SHADER: &str = "#version 100
precision mediump float;
attribute vec3 position;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec3 world_pos;
varying vec3 v_normal;

void main() {
    vec4 world_space = Model * vec4(position, 1.0);
    gl_Position = Projection * world_space;

    v_normal = normalize(world_space.xyz - world_pos);
}";

const DEFAULT_FRAG_SHADER: &str = "#version 100
precision mediump float;
uniform vec3 light_pos;
uniform vec4 color;
uniform float ambient_light;
varying vec3 v_normal;

void main() {
    vec3 lightDir = normalize(light_pos);

    float diff = max(dot(v_normal, lightDir), 0.0);
    vec4 diffuse = min(diff + ambient_light, 1.0) * color;

    gl_FragColor = diffuse;
}";
