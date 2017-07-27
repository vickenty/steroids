extern crate three;
extern crate cgmath;

use cgmath::One;
use cgmath::Rotation3;
use cgmath::Rotation;
use cgmath::InnerSpace;

struct Controller {
    pu: three::Button,
    pd: three::Button,
    yl: three::Button,
    yr: three::Button,
    rl: three::Button,
    rr: three::Button,
    fwd: three::Button,
    rev: three::Button,

    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Controller {
    fn update(&mut self, target: &mut three::Object, window: &three::Window) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        let mut sp = 0.0;

        if self.pu.is_hit(&window.input) {
            dz += 0.02;
        }
        if self.pd.is_hit(&window.input) {
            dz -= 0.02;
        }
        if self.yl.is_hit(&window.input) {
            dy += 0.02;
        }
        if self.yr.is_hit(&window.input) {
            dy -= 0.02;
        }
        if self.rl.is_hit(&window.input) {
            dx += 0.02;
        }
        if self.rr.is_hit(&window.input) {
            dx -= 0.02;
        }
        if self.fwd.is_hit(&window.input) {
            sp += 0.02;
        }
        if self.rev.is_hit(&window.input) {
            sp -= 0.02;
        }

        let rx = cgmath::Quaternion::from_angle_x(cgmath::Rad(dx));
        let ry = cgmath::Quaternion::from_angle_y(cgmath::Rad(dy));
        let rz = cgmath::Quaternion::from_angle_z(cgmath::Rad(dz));

        self.rotation = rx * ry * rz * self.rotation;

        let ax = self.rotation.invert() * cgmath::Vector3::unit_x();
        let az = cgmath::vec3(ax.z, ax.y, -ax.x);

        self.position += az * sp;

        let pf: [f32; 3] = self.position.into();
        let rf: [f32; 4] = self.rotation.into();

        target.set_transform(pf, rf, 1.0);
    }
}

fn main() {
    let mut window = three::Window::new("Steroids", "shaders");

    let mut camera = window.factory.perspective_camera(45.0, 0.1, 100.0);
    camera.look_at([3.0, 3.0, 1.0], [0.0, 0.0, 0.0], None);

    let cube_geom = three::Geometry::new_box(1.0, 1.0, 1.0);
    let mut cube_mesh = window.factory.mesh(cube_geom, three::Material::LineBasic { color: 0xFFFF_FFFF });
    window.scene.add(&cube_mesh);

    let mut control = Controller {
        pu: three::Button::Key(three::Key::Up),
        pd: three::Button::Key(three::Key::Down),
        yl: three::Button::Key(three::Key::Left),
        yr: three::Button::Key(three::Key::Right),
        rl: three::Button::Key(three::Key::Comma),
        rr: three::Button::Key(three::Key::Period),

        fwd: three::Button::Key(three::Key::A),
        rev: three::Button::Key(three::Key::Z),

        position: cgmath::vec3(0.0, 0.0, 0.0),
        rotation: cgmath::Quaternion::one(),
    };

    while window.update() {
        control.update(&mut cube_mesh, &window);
        window.render(&camera);
    }
}
