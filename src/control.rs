use crate::object::{Object, ObjectPool, Trajectory};
use crate::physics::PhysicsHandler;
use crate::renderer::Renderer;
use crate::screen::*;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct ControlHandler {
    move_speed: f32,
    scale_speed: f32,
    place_elevation: f32,
    control_state: ControlState,
    ghost_obj: Option<Object>,
    trajectories: HashMap<usize, Trajectory>,
}

impl ControlHandler {
    fn new(move_speed: f32, scale_speed: f32) -> Self {
        ControlHandler {
            move_speed,
            scale_speed,
            place_elevation: 0.0,
            control_state: ControlState::Idle,
            ghost_obj: None,
            trajectories: HashMap::new(),
        }
    }

    pub fn handle_input(
        &mut self,
        renderer: &mut Renderer,
        objects: &mut ObjectPool,
        physics_handler: &PhysicsHandler,
        dt: f32,
    ) {
        self.handle_movement(renderer, dt);
        self.handle_ghost_obj(dt);

        self.control_state = match self.control_state {
            ControlState::Idle => self.handle_idle(renderer, physics_handler, objects),
            ControlState::Place => self.handle_place(renderer),
            ControlState::Drag => self.handle_drag(renderer, physics_handler, objects),
        };
    }

    fn handle_idle(
        &mut self,
        renderer: &mut Renderer,
        physics_handler: &PhysicsHandler,
        objects: &mut ObjectPool,
    ) -> ControlState {
        if let Some(obj) = self.get_hovered_obj(renderer, objects) {
            let color = Color {
                a: 0.2,
                ..obj.color
            };

            renderer.draw_halo(obj.position, obj.radius * 1.1, Some(color));
            self.draw_obj_trajectory(physics_handler, renderer, objects, &obj);
        }

        if is_key_released(KeyCode::R) {
            if let Some(obj) = self.get_hovered_obj(renderer, objects) {
                objects.remove(obj.id);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            return ControlState::Place;
        }

        ControlState::Idle
    }

    fn handle_place(&mut self, renderer: &mut Renderer) -> ControlState {
        if self.ghost_obj.is_none() {
            self.ghost_obj = Some(Object::new(
                Vec3::ZERO,
                Vec3::ZERO,
                1.0,
                1.0,
                Self::random_color(),
            ));
        }

        let ray = Ray::new_from_mouse(renderer.get_cam());

        if let Some(obj) = &mut self.ghost_obj {
            obj.position = ray.plane_intersect(Some(self.place_elevation));
            obj.draw(renderer);
        }

        if is_mouse_button_released(MouseButton::Left) {
            return ControlState::Drag;
        }

        if is_key_released(KeyCode::Escape) {
            self.ghost_obj = None;
            return ControlState::Idle;
        }

        ControlState::Place
    }

    fn handle_drag(
        &mut self,
        renderer: &mut Renderer,
        physics_handler: &PhysicsHandler,
        objects: &mut ObjectPool,
    ) -> ControlState {
        let ray = Ray::new_from_mouse(renderer.get_cam());

        if let Some(obj) = &mut self.ghost_obj {
            let veloc = (ray.plane_intersect(Some(self.place_elevation)) - obj.position) / 10.0;

            let mut virtual_obj: Object = obj.clone();
            virtual_obj.add_velocity(veloc);

            renderer.draw_arrow(
                obj.position,
                ray.plane_intersect(Some(self.place_elevation)),
                Some(obj.color),
            );

            obj.draw(renderer);

            let mut clones = objects.clone().get_all_in_area(virtual_obj.position, 500.0);
            clones.push(virtual_obj.clone());

            self.draw_objects_trajectories(physics_handler, renderer, &clones);
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(obj) = &mut self.ghost_obj {
                let veloc = (ray.plane_intersect(Some(self.place_elevation)) - obj.position) / 10.0;
                obj.add_velocity(veloc);
                objects.push(obj.clone());
                self.ghost_obj = None;
            }

            return ControlState::Idle;
        }

        if is_key_released(KeyCode::Escape) {
            self.ghost_obj = None;
            return ControlState::Idle;
        }

        ControlState::Drag
    }

    fn handle_movement(&mut self, renderer: &mut Renderer, dt: f32) {
        renderer.move_cam(self.get_input_dir() * self.move_speed * dt);
    }

    fn handle_ghost_obj(&mut self, dt: f32) {
        if let Some(obj) = &mut self.ghost_obj {
            if is_key_down(KeyCode::Up) {
                obj.mass += self.scale_speed * dt;
                obj.radius += self.scale_speed * dt;
            }
            if is_key_down(KeyCode::Down) && obj.mass > 0.3 {
                obj.mass -= self.scale_speed * dt;
                obj.radius -= self.scale_speed * dt;
            }

            if is_key_down(KeyCode::E) {
                self.place_elevation += self.move_speed * dt;
            }
            if is_key_down(KeyCode::Q) {
                self.place_elevation -= self.move_speed * dt;
            }
        }
    }

    fn get_input_dir(&self) -> Vec3 {
        let mut dir = Vec3::ZERO;

        if is_key_down(KeyCode::W) {
            dir.z += 1.0;
        }
        if is_key_down(KeyCode::S) {
            dir.z -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::LeftShift) {
            dir.y += 1.0;
        }
        if is_key_down(KeyCode::LeftControl) {
            dir.y -= 1.0;
        }

        dir
    }

    fn get_hovered_obj(&mut self, renderer: &mut Renderer, objects: &ObjectPool) -> Option<Object> {
        let ray = Ray::new_from_mouse(renderer.get_cam());
        for obj in objects.iter() {
            if !ray.raycast(obj.position, obj.radius) {
                continue;
            }

            return Some(obj.clone());
        }

        None
    }

    fn draw_obj_trajectory(
        &mut self,
        physics_handler: &PhysicsHandler,
        renderer: &Renderer,
        objects: &ObjectPool,
        object: &Object,
    ) {
        let traj = object.calculate_trajectory(objects, physics_handler, 10_000, 2);
        traj.draw(renderer, Some(object.color), object.radius);
    }

    fn draw_objects_trajectories(
        &mut self,
        physics_handler: &PhysicsHandler,
        renderer: &Renderer,
        objects: &ObjectPool,
    ) {
        self.trajectories = objects.calculate_trajectories(physics_handler, 10_000, 2);

        for (id, traj) in &self.trajectories {
            let obj = match objects.get(*id) {
                Some(obj) => obj,
                None => continue,
            };

            traj.draw(renderer, Some(obj.color), obj.radius);
        }
    }

    fn random_color() -> Color {
        Color {
            r: rand::gen_range(0.0, 1.0),
            g: rand::gen_range(0.0, 1.0),
            b: rand::gen_range(0.0, 1.0),
            a: 1.0,
        }
    }
}

impl Default for ControlHandler {
    fn default() -> Self {
        ControlHandler::new(20.0, 5.0)
    }
}

#[derive(Debug, PartialEq)]
enum ControlState {
    Idle,
    Place,
    Drag,
}
