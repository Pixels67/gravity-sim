pub mod object;
pub mod physics;
pub mod screen;
pub mod world;

#[cfg(test)]
mod tests {
    use crate::physics::*;
    use macroquad::prelude::*;

    #[test]
    fn physics_get_grav_force() {
        assert_eq!(
            get_grav_force(1., 1., 1., vec3(1., 0., 0.)),
            vec3(1., 0., 0.)
        );
        assert_eq!(
            get_grav_force(1., 2., 3., vec3(2., 0., 0.)),
            vec3(1.5, 0., 0.)
        );
        assert_eq!(get_grav_force(1., 1., 1., Vec3::ZERO), Vec3::ZERO);
    }

    #[test]
    fn physics_get_veloc() {
        assert_eq!(get_veloc(Vec3::ONE, 1., 1.), Vec3::ONE);
        assert_eq!(get_veloc(Vec3::ZERO, 1., 1.), Vec3::ZERO);
        assert_eq!(get_veloc(vec3(1., 0., 2.), 2., 1.), vec3(0.5, 0., 1.));
    }

    #[test]
    fn physics_get_displ() {
        assert_eq!(get_displ(Vec3::ONE, 1., 1.), Vec3::ONE);
        assert_eq!(get_displ(Vec3::ZERO, 1., 1.), Vec3::ZERO);
        assert_eq!(get_displ(vec3(1., 0., 2.), 2., 1.), vec3(0.5, 0., 1.));
    }

    use crate::object::*;

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
    fn object_new_with_pos() {
        let obj = Object::new_with_pos(vec3(1., 0., 0.));
        assert_eq!(obj.position, vec3(1., 0., 0.));

        let obj = Object::new_with_pos(vec3(4., 0., 3.));
        assert_eq!(obj.position, vec3(4., 0., 3.));

        let obj = Object::new_with_pos(vec3(-2., 0., -1.));
        assert_eq!(obj.position, vec3(-2., 0., -1.));

        let obj = Object::new_with_pos(Vec3::ZERO);
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
        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.), 10.);
        let pos = vec3(5., 0., 0.);

        assert!(ray.raycast(pos, 0.));

        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.), 10.);
        let pos = vec3(5., 1., 0.);

        assert!(ray.raycast(pos, 1.));

        let ray = Ray::new(vec3(0., 0., 0.), vec3(1., 0., 0.), 10.);
        let pos = vec3(5., 2., 0.);

        assert!(!ray.raycast(pos, 1.));
    }
}
