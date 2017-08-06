extern crate three;
extern crate nphysics3d;
extern crate ncollide;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Controller {
    pu: three::Button,
    pd: three::Button,
    yl: three::Button,
    yr: three::Button,
    rl: three::Button,
    rr: three::Button,
    fwd: three::Button,
    rev: three::Button,
    pew: three::Button,
}

const C: f32 = 1.0;

impl Controller {
    fn update(
        &self,
        target: &mut Ship,
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        registry: &mut Registry,
    ) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        let mut sp = 0.0;

        if self.pu.is_hit(&window.input) {
            dz += C;
        }
        if self.pd.is_hit(&window.input) {
            dz -= C;
        }
        if self.yl.is_hit(&window.input) {
            dy += C;
        }
        if self.yr.is_hit(&window.input) {
            dy -= C;
        }
        if self.rl.is_hit(&window.input) {
            dx += C;
        }
        if self.rr.is_hit(&window.input) {
            dx -= C;
        }
        if self.fwd.is_hit(&window.input) {
            sp += C;
        }
        if self.rev.is_hit(&window.input) {
            sp -= C;
        }

        let mut b = target.entity.body.borrow_mut();
        let r = b.position().rotation;
        b.append_lin_force(r * nphysics3d::math::Vector::new(sp, 0.0, 0.0));
        b.append_ang_force(r * nphysics3d::math::Vector::new(dx, dy, dz));

        if self.pew.is_hit(&window.input) {
            // TODO delay
            target.shoot(window, world, registry)
        }
    }
}

struct Entity {
    body: nphysics3d::object::RigidBodyHandle<f32>,
    mesh: three::Mesh,
}

struct Ship {
    entity: Entity,
    hitpoints: u32,
}

struct Bullet {
    entity: Entity,
    damage: u32,
}

type CollisionObject = ncollide::world::CollisionObject<
    nphysics3d::math::Point<f32>,
    nphysics3d::math::Isometry<f32>,
    nphysics3d::object::WorldObject<f32>,
>;

impl Ship {
    fn new(
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        x: f32,
        y: f32,
        z: f32,
        hp: u32,
    ) -> Ship {
        let shape = ncollide::shape::Cuboid::new(nphysics3d::math::Vector::new(0.5, 0.5, 0.5));
        let mut body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);

        body.set_translation(nphysics3d::math::Translation::new(x, y, z));
        let hndl = world.add_rigid_body(body);

        let geom = three::Geometry::new_box(1.0, 1.0, 1.0);
        let mesh = window.factory.mesh(
            geom,
            three::Material::MeshLambert {
                color: 0xabcdef,
                flat: true,
            },
        );
        window.scene.add(&mesh);

        Ship {
            entity: Entity {
                body: hndl,
                mesh: mesh,
            },
            hitpoints: hp,
        }
    }

    fn shoot(
        &self,
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        registry: &mut Registry,
    ) {
        registry.add(Bullet::new(window, world, 0.0, 0.0, 0.0, 1));
    }
}

impl Bullet {
    fn new(
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        x: f32,
        y: f32,
        z: f32,
        d: u32,
    ) -> Bullet {
        // FIXME these probably need to be adjusted according to the size of the
        // cylinder body
        let shape = ncollide::shape::Cone::new(0.5, 0.75);
        let mut body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);

        body.set_translation(nphysics3d::math::Translation::new(x, y, z));
        let hndl = world.add_rigid_body(body);

        let geom = three::Geometry::new_cylinder(0.01, 0.1, 0.25, 256);
        let mesh = window.factory.mesh(
            geom,
            three::Material::MeshLambert {
                color: 0xabcdef,
                flat: true,
            },
        );
        window.scene.add(&mesh);

        Bullet {
            entity: Entity {
                body: hndl,
                mesh: mesh,
            },
            damage: d,
        }
    }
}

impl Ent for Ship {
    fn get_body(&self) -> nphysics3d::object::RigidBodyHandle<f32> {
        self.entity.body.clone()
    }
    fn get_mesh(&mut self) -> &mut three::Mesh {
        &mut self.entity.mesh
    }
}

impl Ent for Bullet {
    fn get_body(&self) -> nphysics3d::object::RigidBodyHandle<f32> {
        self.entity.body.clone()
    }
    fn get_mesh(&mut self) -> &mut three::Mesh {
        &mut self.entity.mesh
    }
}

trait Ent {
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

struct RegistryData {
    counter: u64,
    entities: HashMap<u64, Box<Ent>>,
}

impl RegistryData {
    fn new() -> RegistryData {
        RegistryData {
            counter: 0,
            entities: HashMap::new(),
        }
    }

    fn add<T>(&mut self, entity: T) -> u64
    where
        T: Ent + 'static,
    {
        let id = self.counter;
        self.counter += 1;
        self.entities.insert(id, Box::new(entity));
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

#[derive(Clone)]
struct Registry(Rc<RefCell<RegistryData>>);

impl Registry {
    fn new() -> Self {
        Registry(Rc::new(RefCell::new(RegistryData::new())))
    }

    fn add<T: Ent + 'static>(&self, entity: T) -> u64 {
        self.0.borrow_mut().add(entity)
    }

    fn apply_one<F: FnMut(&mut Box<Ent>)>(&mut self, id: u64, mut f: F) {
        self.0.borrow_mut().apply_one(id, f)
    }

    fn apply_all<F: FnMut(&mut Box<Ent>)>(&mut self, mut f: F) {
        self.0.borrow_mut().apply_all(f)
    }
}

impl ncollide::narrow_phase::ContactHandler<
    nphysics3d::math::Point<f32>,
    nphysics3d::math::Isometry<f32>,
    nphysics3d::object::WorldObject<f32>,
> for Registry {
    fn handle_contact_started(
        &mut self,
        co1: &CollisionObject,
        _co2: &CollisionObject,
        contacts: &ncollide::narrow_phase::ContactAlgorithm<
            nphysics3d::math::Point<f32>,
            nphysics3d::math::Isometry<f32>,
        >,
    ) {
        println!("collision detected");
    }

    fn handle_contact_stopped(&mut self, _co1: &CollisionObject, _co2: &CollisionObject) {}
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

        pew: three::Button::Key(three::Key::Space),
    };

    let mut entities = Registry::new();

    world.register_contact_handler("entities", entities.clone());

    let mut player = Ship::new(&mut window, &mut world, -1.1, 0.1, 0.0, 100);

    entities.add(Ship::new(&mut window, &mut world, 1.1, -0.1, 0.0, 100));
    entities.add(Ship::new(&mut window, &mut world, 2.2, -0.1, 0.1, 100));
    entities.add(Ship::new(&mut window, &mut world, 3.3, -0.1, 0.2, 100));
    entities.add(Ship::new(&mut window, &mut world, 4.4, -0.1, 0.3, 100));

    while window.update() {
        entities.apply_all(|e| e.update_body());
        player.update_body();

        control.update(&mut player, &mut window, &mut world, &mut entities);

        world.step(0.017);

        entities.apply_all(|e| e.update_mesh());
        player.update_mesh();

        // entities.retain(|e| e.alive());

        player.look_at(&mut camera);

        window.render(&camera);
    }
}
