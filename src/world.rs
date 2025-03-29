use crate::control::*;
use crate::object::*;
use crate::physics::*;
use crate::renderer::*;

pub const SIM_SPEED: f32 = 20.0;

pub struct World {
    pub objects: ObjectPool,
    pub physics_handler: PhysicsHandler,
    pub renderer: Renderer,
    pub input_handler: ControlHandler,
}

impl World {
    pub fn new(
        objects: ObjectPool,
        physics_handler: PhysicsHandler,
        renderer: Renderer,
        input_handler: ControlHandler,
    ) -> Self {
        World {
            objects,
            physics_handler,
            renderer,
            input_handler,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.renderer.begin_drawing();

        self.physics_handler
            .update(&mut self.objects, dt * SIM_SPEED);
        self.objects.draw_all(&self.renderer);
        self.input_handler.handle_input(
            &mut self.renderer,
            &mut self.objects,
            &self.physics_handler,
            dt,
        );

        self.renderer.end_drawing();
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: ObjectPool::default(),
            physics_handler: PhysicsHandler::default(),
            renderer: Renderer::default(),
            input_handler: ControlHandler::default(),
        }
    }
}
