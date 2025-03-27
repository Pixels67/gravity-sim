pub mod control;
pub mod object;
pub mod physics;
pub mod renderer;
pub mod screen;
pub mod world;

#[cfg(test)]
mod tests {
    use crate::object::*;
    use macroquad::prelude::*;

    #[test]
    fn object_translate() {
        let mut obj = Object::default();
        obj.translate(vec3(1., 0., 0.));
        assert_eq!(obj.position, vec3(1., 0., 0.));

        let mut obj = Object::default();
        obj.translate(vec3(-2., 0., 0.));
        assert_eq!(obj.position, vec3(-2., 0., 0.));

        let mut obj = Object::default();
        obj.translate(vec3(4., 0., 3.));
        assert_eq!(obj.position, vec3(4., 0., 3.));

        let mut obj = Object::default();
        obj.translate(Vec3::ZERO);
        assert_eq!(obj.position, Vec3::ZERO);
    }

    #[test]
    fn object_clone_with_id() {
        let original = Object::default();
        let new = original.clone_with_id(4);

        assert_eq!(new.id, 4);
    }

    #[test]
    fn object_add_velocity() {
        let mut obj = Object::default();
        obj.add_velocity(vec3(1., 0., 0.));
        assert_eq!(obj.velocity, vec3(1., 0., 0.));

        let mut obj = Object::default();
        obj.add_velocity(vec3(-2., 0., 0.));
        assert_eq!(obj.velocity, vec3(-2., 0., 0.));

        let mut obj = Object::default();
        obj.add_velocity(vec3(4., 0., 3.));
        assert_eq!(obj.velocity, vec3(4., 0., 3.));

        let mut obj = Object::default();
        obj.add_velocity(Vec3::ZERO);
        assert_eq!(obj.velocity, Vec3::ZERO);
    }

    #[test]
    fn object_update_pos() {
        let mut obj = Object::default();
        obj.add_velocity(vec3(1., 0., 0.));
        obj.update_pos();
        assert_eq!(obj.position, vec3(1., 0., 0.));

        let mut obj = Object::default();
        obj.add_velocity(vec3(1., 2., 0.));
        obj.add_velocity(vec3(-3., 2., -1.));
        obj.update_pos();
        assert_eq!(obj.position, vec3(-2., 4., -1.));
    }

    #[test]
    fn object_clone() {
        let original = Object::default();
        let new = original.clone();

        assert_eq!(new, original);
    }

    use crate::screen::*;

    #[test]
    fn raycast() {
        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.));
        let pos = vec3(5., 0., 0.);

        assert!(ray.raycast(pos, 0.));

        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.));
        let pos = vec3(5., 1., 0.);

        assert!(ray.raycast(pos, 1.));

        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.));
        let pos = vec3(5., 2., 0.);

        assert!(!ray.raycast(pos, 1.));
    }
}
