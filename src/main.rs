extern crate three;
extern crate nphysics3d;
extern crate ncollide;

use std::collections::HashMap;
//use std::borrow::BorrowMut;
//use std::borrow::Borrow;

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
    fn update(&self, target: &nphysics3d::object::RigidBodyHandle<f32>, input: &three::Input) {
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

struct Cube {
    //    body: nphysics3d::object::RigidBodyHandle<f32>,
    //    mesh: three::Mesh,
    entity: Entity,
}

impl Ent for Cube {
    fn new(window: &mut three::Window, world: &mut nphysics3d::world::World<f32>, properties: HashMap<&'static str, f32>) -> Box<Ent> {
        let shape = ncollide::shape::Cuboid::new(nphysics3d::math::Vector::new(0.5, 0.5, 0.5));
        let mut body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);
        let x = properties.get("x").cloned().unwrap();
        let y = properties.get("y").cloned().unwrap();
        let z = properties.get("z").cloned().unwrap();

        body.set_translation(nphysics3d::math::Translation::new(x, y, z));
        let hndl = world.add_rigid_body(body);

        let geom = three::Geometry::new_box(1.0, 1.0, 1.0);
        let mesh = window.factory.mesh(geom, three::Material::MeshLambert { color: 0xabcdef, flat: true });
        window.scene.add(&mesh);

        Box::new(
            Cube {
                entity:
                Entity {
                    body: hndl,
                    mesh: mesh,
                }
            })
    }
    fn get_body(&self) -> nphysics3d::object::RigidBodyHandle<f32> {
        self.entity.body.clone()
    }

    fn get_mesh(&mut self) -> &mut three::Mesh {
        &mut self.entity.mesh
    }
}

trait Ent {
    fn new(window: &mut three::Window, world: &mut nphysics3d::world::World<f32>, properties: HashMap<&'static str, f32>) -> Box<Ent>
        where Self: Sized;

    fn get_body(&self) -> nphysics3d::object::RigidBodyHandle<f32>;

    fn get_mesh(&mut self) -> &mut three::Mesh;

    fn update_body(&self) {
        let body = self.get_body();
        body.borrow_mut().clear_forces();
    }

    fn update_mesh(&mut self) {
        let body = self.get_body();
        let body = body.borrow();
        let pos = body.position();

        let pf: [f32; 3] = pos.translation.vector.into();
        let rf: [f32; 4] = pos.rotation.as_ref().coords.into();

        self.get_mesh().set_transform(pf, rf, 1.0);
    }

    fn look_at(&self, camera: &mut three::Camera<three::Perspective>) {
        let body = self.get_body();
        let pf: [f32; 3] = body.borrow().position().translation.vector.into();
        camera.look_at([5.0, 5.0, 5.0], pf, None);
    }
}

//impl Entity {
//    fn new(window: &mut three::Window, world: &mut nphysics3d::world::World<f32>, x: f32, y: f32, z: f32) -> Self {
//        let shape = ncollide::shape::Cuboid::new(nphysics3d::math::Vector::new(0.5, 0.5, 0.5));
//        let mut body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);
//        body.set_translation(nphysics3d::math::Translation::new(x, y, z));
//        let hndl = world.add_rigid_body(body);
//
//        let geom = three::Geometry::new_box(1.0, 1.0, 1.0);
//        let mesh = window.factory.mesh(geom, three::Material::MeshLambert { color: 0xabcdef, flat: true });
//        window.scene.add(&mesh);
//
//        Entity {
//            body: hndl,
//            mesh: mesh,
//        }
//    }
//
//    fn update_body(&self) {
//        self.body.borrow_mut().clear_forces();
//    }
//
//    fn update_mesh(&mut self) {
//        let body = self.body.borrow();
//        let pos = body.position();
//
//        let pf: [f32; 3] = pos.translation.vector.into();
//        let rf: [f32; 4] = pos.rotation.as_ref().coords.into();
//
//        self.mesh.set_transform(pf, rf, 1.0);
//    }
//
//    fn look_at<P>(&self, camera: &mut three::Camera<P>) {
//        let pf: [f32; 3] = self.body.borrow().position().translation.vector.into();
//        camera.look_at([5.0, 5.0, 5.0], pf, None);
//    }
//}

struct Registry {
    counter: u64,
    entities: HashMap<u64, Box<Ent>>,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            counter: 0,
            entities: HashMap::new(),
        }
    }

    fn add(&mut self, window: &mut three::Window, world: &mut nphysics3d::world::World<f32>, x: f32, y: f32, z: f32) -> u64 {
        let id = self.counter;
        self.counter += 1;

        let mut properties: HashMap<&'static str, f32> = HashMap::new();
        properties.insert("x", x);
        properties.insert("y", y);
        properties.insert("z", z);

        let ent = Cube::new(window, world, properties);
        self.entities.insert(id, ent);

        id
    }

    fn apply_one<F: FnMut(&mut Box<Ent>)>(&mut self, id: u64, mut f: F) {
        if let Some(ent_ref) = self.entities.get_mut(&id) {
            f(ent_ref)
        }
    }

    fn apply_all<F: FnMut(&mut Box<Ent>)>(&mut self, mut f: F) {
        for e in self.entities.values_mut() {
            f(e)
        }
    }
}

fn main() {
    let mut window = three::Window::new("Steroids", "shaders");

    let mut lamp0 = window.factory.directional_light(0xddddffff, 0.5);
    lamp0.look_at([0.0, 0.0, 0.0], [-1.0, -1.0, 1.0], None);
    window.scene.add(&lamp0);

    let lamp1 = window.factory.directional_light(0xffffffff, 0.4);
    window.scene.add(&lamp1);

    let lamp2 = window.factory.ambient_light(0xddffffff, 0.01);
    window.scene.add(&lamp2);

    let mut camera = window.factory.perspective_camera(45.0, 0.1, 100.0);
    let mut world = nphysics3d::world::World::new();

    let control = Controller {
        pu: three::Button::Key(three::Key::Up),
        pd: three::Button::Key(three::Key::Down),
        yl: three::Button::Key(three::Key::Left),
        yr: three::Button::Key(three::Key::Right),
        rl: three::Button::Key(three::Key::Comma),
        rr: three::Button::Key(three::Key::Period),

        fwd: three::Button::Key(three::Key::A),
        rev: three::Button::Key(three::Key::Z),
    };

    let mut entities = Registry::new();

    let player_id = entities.add(&mut window, &mut world, -1.1, 0.1, 0.0);
    entities.add(&mut window, &mut world, 1.1, -0.1, 0.0);
    entities.add(&mut window, &mut world, 2.2, -0.1, 0.1);
    entities.add(&mut window, &mut world, 3.3, -0.1, 0.2);
    entities.add(&mut window, &mut world, 4.4, -0.1, 0.3);

    while window.update() {
        entities.apply_all(|e| e.update_body());
        entities.apply_one(player_id, |e| control.update(&e.get_body(), &window.input));

        world.step(0.017);

        entities.apply_all(|e| e.update_mesh());
        entities.apply_one(player_id, |e| e.look_at(&mut camera));

        window.render(&camera);
    }
}
