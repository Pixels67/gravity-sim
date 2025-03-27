use macroquad::prelude::*;

pub struct Renderer {
    cam: Camera3D,
    obj_mat: Material,
}

impl Renderer {
    pub fn new(cam: Camera3D, shader: ShaderSource, mat_params: MaterialParams) -> Self {
        let obj_mat = load_material(shader, mat_params).unwrap();

        obj_mat.set_uniform("light_pos", Vec3::ONE);
        obj_mat.set_uniform("ambient_light", 0.25f32);

        Renderer { cam, obj_mat }
    }

    pub fn set_light_pos(&self, pos: Vec3) {
        self.obj_mat.set_uniform("light_pos", pos);
    }

    pub fn set_ambient_light(&mut self, ambient_light: f32) {
        self.obj_mat.set_uniform("ambient_light", ambient_light);
    }

    pub fn set_color(&self, color: Color) {
        self.obj_mat.set_uniform("color", color);
    }

    pub fn set_world_pos(&self, pos: Vec3) {
        self.obj_mat.set_uniform("world_pos", pos);
    }

    pub fn move_cam(&mut self, translation: Vec3) {
        self.cam.position += translation.with_x(-translation.x);
        self.cam.target += translation.with_x(-translation.x);
    }

    pub fn get_cam(&self) -> &Camera3D {
        &self.cam
    }

    pub fn begin_drawing(&self) {
        set_camera(&self.cam);

        draw_grid(1000, 5.0, WHITE, GRAY);
    }

    pub fn end_drawing(&self) {
        gl_use_default_material();
        set_default_camera()
    }

    pub fn draw_sphere(&self, pos: Vec3, radius: f32, color: Option<Color>) {
        let color = color.unwrap_or(WHITE);
        gl_use_material(&self.obj_mat);

        self.set_color(color);
        self.set_world_pos(pos);

        draw_sphere(pos, radius, None, color);
        gl_use_default_material();
    }

    pub fn draw_halo(&self, pos: Vec3, radius: f32, color: Option<Color>) {
        let color = color.unwrap_or(WHITE);
        gl_use_default_material();
        draw_sphere(pos, radius, None, color);
    }

    pub fn draw_line(&self, start: Vec3, end: Vec3, color: Option<Color>) {
        let color = color.unwrap_or(WHITE);
        draw_line_3d(start, end, color);
    }

    pub fn draw_arrow(&self, start: Vec3, end: Vec3, color: Option<Color>) {
        let color = color.unwrap_or(WHITE);
        gl_use_default_material();
        draw_line_3d(start, end, color);
        draw_sphere(end, 0.2, None, color);
    }
}

impl Default for Renderer {
    fn default() -> Self {
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

        let mat_params = MaterialParams {
            pipeline_params,
            uniforms: vec![
                UniformDesc::new("light_pos", UniformType::Float3),
                UniformDesc::new("ambient_light", UniformType::Float1),
                UniformDesc::new("color", UniformType::Float4),
                UniformDesc::new("world_pos", UniformType::Float3),
            ],
            ..Default::default()
        };

        Self::new(cam, shader, mat_params)
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
