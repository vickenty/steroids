extern crate three;
extern crate nphysics3d;
extern crate ncollide;

struct Controller {
    pu: three::Button,
    pd: three::Button,
    yl: three::Button,
    yr: three::Button,
    rl: three::Button,
    rr: three::Button,
    fwd: three::Button,
    rev: three::Button,
}

const C: f32 = 1.0;

impl Controller {
    fn update(&mut self, target: &nphysics3d::object::RigidBodyHandle<f32>, input: &three::Input) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        let mut sp = 0.0;

        if self.pu.is_hit(&input) {
            dz += C;
        }
        if self.pd.is_hit(&input) {
            dz -= C;
        }
        if self.yl.is_hit(&input) {
            dy += C;
        }
        if self.yr.is_hit(&input) {
            dy -= C;
        }
        if self.rl.is_hit(&input) {
            dx += C;
        }
        if self.rr.is_hit(&input) {
            dx -= C;
        }
        if self.fwd.is_hit(&input) {
            sp += C;
        }
        if self.rev.is_hit(&input) {
            sp -= C;
        }

        let mut b = target.borrow_mut();
        let r = b.position().rotation;
        b.append_lin_force(r * nphysics3d::math::Vector::new(sp, 0.0, 0.0));
        b.append_ang_force(r * nphysics3d::math::Vector::new(dx, dy, dz));
    }
}

struct Entity {
    body: nphysics3d::object::RigidBodyHandle<f32>,
    mesh: three::Mesh,
}

impl Entity {
    fn new(factory: &mut three::Factory, world: &mut nphysics3d::world::World<f32>) -> Self {
        let shape = ncollide::shape::Cuboid::new(nphysics3d::math::Vector::new(0.5, 0.5, 0.5));
        let body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 0.1, 0.1);
        let hndl = world.add_rigid_body(body);

        let geom = three::Geometry::new_box(1.0, 1.0, 1.0);
        let mesh = factory.mesh(geom, three::Material::LineBasic { color: 0xFFFF_FFFF });

        Entity {
            body: hndl,
            mesh: mesh,
        }
    }

    fn update_body(&mut self) {
        self.body.borrow_mut().clear_forces();
    }

    fn update_mesh(&mut self) {
        let body = self.body.borrow();
        let pos = body.position();

        let pf: [f32; 3] = pos.translation.vector.into();
        let rf: [f32; 4] = pos.rotation.as_ref().coords.into();

        self.mesh.set_transform(pf, rf, 1.0);
    }

    fn look_at<P>(&self, camera: &mut three::Camera<P>) {
        let pf: [f32; 3] = self.body.borrow().position().translation.vector.into();
        camera.look_at([3.0, 3.0, 1.0], pf, None);
    }
}

fn main() {
    let mut window = three::Window::new("Steroids", "shaders");

    let mut camera = window.factory.perspective_camera(45.0, 0.1, 100.0);
    let mut world = nphysics3d::world::World::new();

    let mut control = Controller {
        pu: three::Button::Key(three::Key::Up),
        pd: three::Button::Key(three::Key::Down),
        yl: three::Button::Key(three::Key::Left),
        yr: three::Button::Key(three::Key::Right),
        rl: three::Button::Key(three::Key::Comma),
        rr: three::Button::Key(three::Key::Period),

        fwd: three::Button::Key(three::Key::A),
        rev: three::Button::Key(three::Key::Z),
    };

    let mut c1 = Entity::new(&mut window.factory, &mut world);
    window.scene.add(&c1.mesh);

    let mut c2 = Entity::new(&mut window.factory, &mut world);
    window.scene.add(&c2.mesh);

    c1.body.borrow_mut().set_translation(nphysics3d::math::Translation::new(-1.1, 0.1, 0.0));
    c2.body.borrow_mut().set_translation(nphysics3d::math::Translation::new(1.1, -0.1, 0.0));

    while window.update() {
        c1.update_body();
        c2.update_body();

        control.update(&mut c1.body, &window.input);

        world.step(0.017);

        c1.update_mesh();
        c2.update_mesh();

        c1.look_at(&mut camera);

        window.render(&camera);
    }
}
