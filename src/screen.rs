use macroquad::prelude::*;

pub fn get_mouse_pos() -> Vec2 {
    vec2(
        (mouse_position().0 - screen_width() / 2.) / screen_width(),
        (-mouse_position().1 + screen_height() / 2.) / screen_height(),
    ) * 2.
}

pub fn get_aspect_ratio() -> f32 {
    screen_width() / screen_height()
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub len: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, len: f32) -> Self {
        let dir = dir.normalize();
        Self { origin, dir, len }
    }

    pub fn new_from_cam(cam: &Camera3D, pos: Vec2, len: f32) -> Self {
        let cam_forward = (cam.target - cam.position).normalize();
        let cam_right = Vec3::cross(cam_forward, cam.up).normalize();
        let cam_up = Vec3::cross(cam_right, cam_forward).normalize();

        let aspect_ratio = get_aspect_ratio();
        let fov_scale = f32::tan(cam.fovy / 2.);

        let offset = cam_right * (pos.x * aspect_ratio * fov_scale) + cam_up * (pos.y * fov_scale);
        let dir = (offset + cam_forward).normalize();

        Self {
            origin: cam.position,
            dir,
            len,
        }
    }

    pub fn new_from_mouse(cam: &Camera3D, len: f32) -> Self {
        Self::new_from_cam(cam, get_mouse_pos(), len)
    }

    pub fn raycast(&self, target_pos: Vec3, margin_of_err: f32) -> bool {
        if self.dir == Vec3::ZERO {
            return false;
        }

        let closest_point_approx = self.origin + self.dir * (target_pos - self.origin).length();
        if target_pos == closest_point_approx {
            return true;
        }

        let point_to_target = target_pos - closest_point_approx;
        let plane_normal = self.dir.cross(point_to_target).normalize();
        let mirror_axis = self.dir.cross(plane_normal);
        let mirror_target = (target_pos - closest_point_approx) * mirror_axis;

        let closest_point = target_pos + mirror_target;

        (closest_point - target_pos).length() <= margin_of_err
    }

    pub fn end(&self) -> Vec3 {
        self.origin + self.dir * self.len
    }

    pub fn grid_intersect(&self) -> Vec3 {
        if self.dir.y >= 0. {
            return Vec3::ZERO;
        }

        self.origin + (self.origin.y / self.dir.y.abs()) * self.dir
    }

    #[cfg(debug_assertions)]
    pub fn draw(&self, color: Color) {
        draw_line_3d(self.origin, self.origin + self.dir * self.len, color);
    }
}
