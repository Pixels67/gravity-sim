use crate::object::*;
use crate::physics;
use macroquad::prelude::*;
use std::vec;
use crate::screen::{get_mouse_pos, Ray};

pub struct World {
    pub grav_const: f32,
    pub cam: Camera3D,
    pub obj_mat: Material,
    pub update_interval: f32,
    accumulator: f32,
    objects: ObjectPool,
}

impl World {
    pub fn new(grav_const: f32, update_interval: f32) -> Self {
        let cam = Camera3D {
            position: vec3(0., 10., -15.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };

        let shader = ShaderSource::Glsl {
            vertex: DEFAULT_VERT_SHADER,
            fragment: DEFAULT_FRAG_SHADER,
        };

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let params = MaterialParams {
            pipeline_params,
            uniforms: vec![
                UniformDesc::new("light_pos", UniformType::Float3),
                UniformDesc::new("ambient_light", UniformType::Float1),
                UniformDesc::new("color", UniformType::Float4),
                UniformDesc::new("world_pos", UniformType::Float3),
            ],
            ..Default::default()
        };

        let obj_mat = load_material(shader, params).unwrap();
        obj_mat.set_uniform("light_pos", Vec3::ONE);
        obj_mat.set_uniform("ambient_light", 0.25f32);

        World {
            grav_const,
            cam,
            obj_mat,
            update_interval,
            accumulator: 0.,
            objects: ObjectPool::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn set_light_pos(&mut self, pos: Vec3) {
        self.obj_mat.set_uniform("light_pos", pos);
    }

    pub fn set_ambient_light(&mut self, ambient_light: f32) {
        self.obj_mat.set_uniform("ambient_light", ambient_light);
    }

    pub fn update(&mut self, dt: f32) {
        self.accumulator += dt;
        while self.accumulator > self.update_interval {
            let mut objects = self.objects.clone();

            for obj in objects.iter_mut() {
                obj.add_velocity(self.get_obj_veloc(obj, self.update_interval));
            }

            for obj in objects.iter_mut() {
                obj.update_pos();
            }

            self.objects = objects;

            self.accumulator -= self.update_interval;
        }

        self.handle_input();
    }

    pub fn draw_all(&self) {
        set_camera(&self.cam);

        draw_grid(
            1_000,
            4.,
            Color {
                r: 0.8,
                g: 1.,
                b: 1.,
                a: 0.2,
            },
            Color {
                r: 0.5,
                g: 1.,
                b: 1.,
                a: 0.1,
            },
        );

        self.objects.draw_all(&self.obj_mat);

        let ray = Ray::new_from_cam(&self.cam, get_mouse_pos(), 100.);

        for obj in self.objects.iter() {
            if ray.raycast(obj.position, 0.5) {
                draw_sphere(obj.position, 0.5, None, WHITE);
            }
        }

        #[cfg(debug_assertions)]
        for obj in self.objects.iter() {
            draw_line_3d(
                obj.position,
                obj.position + self.get_obj_veloc(obj, get_frame_time() * 1_000.),
                MAGENTA,
            );
        }

        set_default_camera();
    }

    fn get_obj_veloc(&self, object: &Object, dt: f32) -> Vec3 {
        let mut veloc: Vec3 = Vec3::default();

        for other in self.objects.iter() {
            if object == other {
                continue;
            }
            let dist = other.position - object.position;
            let force = physics::get_grav_force(self.grav_const, object.mass, other.mass, dist);
            veloc += physics::get_veloc(force, object.mass, dt);
        }

        veloc
    }

    fn handle_input(&mut self) {
        if get_keys_down().contains(&KeyCode::W) {
            self.cam.position.z += 0.4;
            self.cam.target.z += 0.4;
        }
        if get_keys_down().contains(&KeyCode::S) {
            self.cam.position.z -= 0.4;
            self.cam.target.z -= 0.4;
        }
        if get_keys_down().contains(&KeyCode::LeftControl) {
            self.cam.position.y -= 0.4;
            self.cam.target.y -= 0.4;
        }
        if get_keys_down().contains(&KeyCode::LeftShift) {
            self.cam.position.y += 0.4;
            self.cam.target.y += 0.4;
        }
        if get_keys_down().contains(&KeyCode::D) {
            self.cam.position.x -= 0.4;
            self.cam.target.x -= 0.4;
        }
        if get_keys_down().contains(&KeyCode::A) {
            self.cam.position.x += 0.4;
            self.cam.target.x += 0.4;
        }
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
