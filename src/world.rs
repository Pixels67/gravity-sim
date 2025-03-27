use crate::object::*;
use crate::physics;
use crate::screen::*;
use macroquad::prelude::*;
use ::rand::*;
use std::cmp::PartialEq;
use std::vec;

pub struct World {
    pub grav_const: f32,
    pub cam: Camera3D,
    pub obj_mat: Material,
    pub update_interval: f32,
    move_speed: f32,
    scale_speed: f32,
    accumulator: f32,
    pub objects: ObjectPool,
    control_state: ControlState,
    new_planet_pos: Vec3,
    place_elevation: f32,
    ghost_obj_id: usize,
    current_obj_mass: f32,
}

impl Clone for World {
    fn clone(&self) -> Self {
        World {
            grav_const: self.grav_const,
            cam: Camera3D::default(),
            obj_mat: self.obj_mat.clone(),
            update_interval: self.update_interval,
            move_speed: self.move_speed,
            scale_speed: self.scale_speed,
            accumulator: self.accumulator,
            objects: self.objects.clone(),
            control_state: ControlState::Idle,
            new_planet_pos: Vec3::ZERO,
            place_elevation: 0.0,
            ghost_obj_id: 0,
            current_obj_mass: 0.0,
        }
    }
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
            move_speed: 30.,
            scale_speed: 5.,
            objects: ObjectPool::new(),
            control_state: ControlState::Idle,
            new_planet_pos: Vec3::ZERO,
            place_elevation: 0.,
            ghost_obj_id: 0,
            current_obj_mass: 1.,
        }
    }

    pub fn add_object(&mut self, object: Object) -> usize {
        self.objects.push(object)
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
                obj.add_velocity(self.get_obj_veloc(&self.objects, obj, self.update_interval) * 5.);
            }

            for obj in objects.iter_mut() {
                obj.update_pos();
            }

            self.objects = objects;
            self.handle_collision();
            self.accumulator -= self.update_interval;
        }

        self.handle_input(dt);
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

        let ray = Ray::new_from_mouse(&self.cam, 100.);

        for obj in self.objects.iter() {
            if ray.raycast(obj.position, obj.radius) {
                let color = Color {
                    r: obj.color.r,
                    g: obj.color.g,
                    b: obj.color.b,
                    a: 0.2,
                };

                draw_sphere(obj.position, obj.radius, None, color);
            }
        }

        #[cfg(debug_assertions)]
        for obj in self.objects.iter() {
            draw_line_3d(
                obj.position,
                obj.position + self.get_obj_veloc(&self.objects, obj, 100.),
                RED,
            );
        }

        set_default_camera();
    }

    fn get_obj_veloc(&self, objects: &ObjectPool, object: &Object, dt: f32) -> Vec3 {
        let mut veloc: Vec3 = Vec3::default();

        for other in objects.iter() {
            if object == other {
                continue;
            }
            let dist = other.position - object.position;
            let force = physics::get_grav_force(self.grav_const, object.mass, other.mass, dist);
            veloc += physics::get_veloc(force, object.mass, dt);
        }

        veloc
    }

    fn handle_collision(&mut self) {
        for obj in self.objects.clone().iter_mut() {
            let other = self.is_obj_colliding(&self.objects, obj);
            if other.is_none() {
                continue;
            }

            let other = other.unwrap();
            let largest: &Object = if obj.mass > other.mass { &obj } else { &other };
            self.objects.push(Object::new(
                largest.position,
                (obj.velocity * obj.mass + other.velocity * other.mass) / (obj.mass + other.mass),
                obj.mass + other.mass,
                obj.radius + other.radius,
                largest.color,
            ));

            self.objects.remove(obj.id);
            self.objects.remove(other.id);
            break;
        }
    }

    fn is_obj_colliding(&self, objects: &ObjectPool, object: &Object) -> Option<Object> {
        for other in objects.iter() {
            if object.id == other.id {
                continue;
            }

            if other.id == self.ghost_obj_id || object.id == self.ghost_obj_id {
                continue;
            }

            let dist = other.position - object.position;
            let combined_radius = object.radius + other.radius;
            let surface_dist = dist.length() - combined_radius;

            if surface_dist < 0. {
                return Some(other.clone());
            }
        }

        None
    }

    fn handle_input(&mut self, dt: f32) {
        self.handle_movement(dt);
        self.handle_ghost_obj(dt);

        self.control_state = match self.control_state {
            ControlState::Idle => self.handle_idle(),
            ControlState::Place => self.handle_place(),
            ControlState::Drag => self.handle_drag(),
        };
    }

    fn handle_idle(&mut self) -> ControlState {
        if is_mouse_button_pressed(MouseButton::Left) {
            return ControlState::Place;
        }

        if is_mouse_button_released(MouseButton::Right) {
            let ray = Ray::new_from_mouse(&self.cam, 100.);
            for obj in self.objects.clone().iter() {
                if !ray.raycast(obj.position, obj.radius) {
                    continue;
                }
                self.objects.remove(obj.id);
                break;
            }
        }

        ControlState::Idle
    }

    fn handle_place(&mut self) -> ControlState {
        let ray = Ray::new_from_mouse(&self.cam, 15.);

        self.objects.remove(self.ghost_obj_id);
        self.ghost_obj_id = self.add_object(Object::new(
            ray.grid_intersect().with_y(self.place_elevation),
            Vec3::ZERO,
            0.,
            self.current_obj_mass / 2.,
            WHITE,
        ));

        if is_mouse_button_released(MouseButton::Left) {
            self.new_planet_pos = ray.grid_intersect().with_y(self.place_elevation);
            return ControlState::Drag;
        }

        if is_key_released(KeyCode::Escape) {
            self.objects.remove(self.ghost_obj_id);
            self.ghost_obj_id = 0;
            return ControlState::Idle;
        }

        ControlState::Place
    }

    fn handle_drag(&mut self) -> ControlState {
        let ray = Ray::new_from_mouse(&self.cam, 15.);

        set_camera(&self.cam);

        let mut obj = Object::new(
            self.new_planet_pos,
            (ray.grid_intersect().with_y(self.place_elevation) - self.new_planet_pos) / 100.,
            self.current_obj_mass,
            self.current_obj_mass / 2.,
            WHITE,
        );

        self.draw_obj_path(&mut obj, 100000, self.update_interval);

        draw_line_3d(
            self.new_planet_pos,
            ray.grid_intersect().with_y(self.place_elevation),
            GREEN,
        );

        set_default_camera();

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut rng = rng();

            let color = Color {
                r: rng.random_range(0f32..1f32),
                g: rng.random_range(0f32..1f32),
                b: rng.random_range(0f32..1f32),
                a: 1.,
            };

            self.objects.remove(self.ghost_obj_id);
            self.ghost_obj_id = 0;

            self.add_object(Object::new(
                self.new_planet_pos,
                (ray.grid_intersect().with_y(self.place_elevation) - self.new_planet_pos) / 100.,
                self.current_obj_mass,
                self.current_obj_mass / 2.,
                color,
            ));

            return ControlState::Idle;
        }

        if is_key_released(KeyCode::Escape) {
            self.objects.remove(self.ghost_obj_id);
            self.ghost_obj_id = 0;
            return ControlState::Idle;
        }

        ControlState::Drag
    }

    pub fn draw_obj_path(&self, object: &mut Object, length: u32, step: f32) {
        let mut objects: ObjectPool = self.objects.clone();
        if objects.iter().find(|obj| obj.id == object.id).is_none() {
            object.id = objects.push(object.clone());
        }

        let initial_pos = object.position;
        let mut pos = object.position;

        for i in 0..length {
            let mut clone = objects.clone();

            for obj in clone.iter_mut() {
                obj.add_velocity(self.get_obj_veloc(&objects.clone(), obj, step) * 5.);
            }

            for obj in clone.iter_mut() {
                obj.update_pos();
            }

            objects = clone;

            for obj in objects.clone().iter_mut() {
                let other = self.is_obj_colliding(&objects.clone(), obj);
                if other.is_none() {
                    continue;
                }

                let other = other.unwrap();
                let largest: &Object = if obj.mass > other.mass { &obj } else { &other };
                objects.push(Object::new(
                    largest.position,
                    (obj.velocity * obj.mass + other.velocity * other.mass)
                        / (obj.mass + other.mass),
                    obj.mass + other.mass,
                    obj.radius + other.radius,
                    largest.color,
                ));

                objects.remove(obj.id);
                objects.remove(other.id);
                break;
            }

            let obj = objects.iter().find(|obj| obj.id == object.id);

            if obj.is_none() {
                draw_sphere(pos, object.radius, None, RED);
                break;
            }

            if i % 10 == 0 {
                draw_line_3d(pos, obj.unwrap().position, object.color);
                pos = obj.unwrap().position;
            }

            if (pos - initial_pos).length() > 500. {
                break;
            }
        }
    }

    fn handle_movement(&mut self, dt: f32) {
        if get_keys_down().contains(&KeyCode::W) {
            self.cam.position.z += self.move_speed * dt;
            self.cam.target.z += self.move_speed * dt;
        }
        if get_keys_down().contains(&KeyCode::S) {
            self.cam.position.z -= self.move_speed * dt;
            self.cam.target.z -= self.move_speed * dt;
        }
        if get_keys_down().contains(&KeyCode::LeftControl) {
            self.cam.position.y -= self.move_speed * dt;
            self.cam.target.y -= self.move_speed * dt;
        }
        if get_keys_down().contains(&KeyCode::LeftShift) {
            self.cam.position.y += self.move_speed * dt;
            self.cam.target.y += self.move_speed * dt;
        }
        if get_keys_down().contains(&KeyCode::D) {
            self.cam.position.x -= self.move_speed * dt;
            self.cam.target.x -= self.move_speed * dt;
        }
        if get_keys_down().contains(&KeyCode::A) {
            self.cam.position.x += self.move_speed * dt;
            self.cam.target.x += self.move_speed * dt;
        }
    }

    fn handle_ghost_obj(&mut self, dt: f32) {
        if is_key_down(KeyCode::E) {
            self.place_elevation += self.scale_speed * dt;
        }
        if is_key_down(KeyCode::Q) {
            self.place_elevation -= self.scale_speed * dt;
        }

        if is_key_down(KeyCode::Up) {
            self.current_obj_mass += self.scale_speed * dt;
        }
        if is_key_down(KeyCode::Down) && self.current_obj_mass > 0.1 {
            self.current_obj_mass -= self.scale_speed * dt;
        }

        if self.ghost_obj_id != 0 {
            let obj: &mut Object = self
                .objects
                .iter_mut()
                .find(|obj| obj.id == self.ghost_obj_id)
                .unwrap();
            obj.radius = self.current_obj_mass / 2.
        }
    }
}

#[derive(Debug, PartialEq)]
enum ControlState {
    Idle,
    Place,
    Drag,
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
