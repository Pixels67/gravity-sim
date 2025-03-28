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
    traj_update_interval: u64,
    control_state: ControlState,
    ghost_obj: Option<Object>,
    traj_update_counter: u64,
    trajectories: HashMap<usize, Trajectory>,
}

impl ControlHandler {
    fn new(move_speed: f32, scale_speed: f32, traj_update_interval: u64) -> Self {
        ControlHandler {
            move_speed,
            scale_speed,
            traj_update_interval,
            place_elevation: 0.0,
            control_state: ControlState::Idle,
            ghost_obj: None,
            traj_update_counter: 0,
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
            let traj = obj.calculate_trajectory(objects, physics_handler, 10_000, 10);
            traj.draw(renderer, Some(obj.color), obj.radius);
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
            let veloc = (ray.plane_intersect(Some(self.place_elevation)) - obj.position) / 100.0;

            let mut virtual_obj: Object = obj.clone();
            virtual_obj.add_velocity(veloc);

            renderer.draw_arrow(
                obj.position,
                ray.plane_intersect(Some(self.place_elevation)),
                Some(obj.color),
            );

            obj.draw(renderer);
            let traj = virtual_obj.calculate_trajectory(objects, physics_handler, 10_000, 20);
            traj.draw(renderer, Some(obj.color), obj.radius);

            let mut clones = objects.get_all_in_area(obj.position, 100.0);
            let id = clones.push(virtual_obj.clone());

            for (id, traj) in &self.trajectories {
                let obj = match clones.get(*id) {
                    Some(obj) => obj,
                    None => continue,
                };

                traj.draw(renderer, Some(obj.color), obj.radius);
            }

            if self.traj_update_counter % self.traj_update_interval == 0 {
                self.trajectories =
                    clones.calculate_trajectories_except(id, physics_handler, 5_000, 20);
            }

            self.traj_update_counter += 1;

            if self.traj_update_counter == (2u128.pow(64) - 1) as u64 {
                self.traj_update_counter = 0;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(obj) = &mut self.ghost_obj {
                let veloc =
                    (ray.plane_intersect(Some(self.place_elevation)) - obj.position) / 100.0;
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
        ControlHandler::new(20.0, 5.0, 5)
    }
}

#[derive(Debug, PartialEq)]
enum ControlState {
    Idle,
    Place,
    Drag,
}
