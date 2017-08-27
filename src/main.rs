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
        target: &mut Box<Ent>,
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        registry: &Registry,
    ) {
        let mut roll = 0.0;
        let mut yaw = 0.0;
        let mut pitch = 0.0;
        let mut throttle = 0.0;

        if self.pu.is_hit(&window.input) {
            pitch += C;
        }
        if self.pd.is_hit(&window.input) {
            pitch -= C;
        }
        if self.yl.is_hit(&window.input) {
            yaw += C;
        }
        if self.yr.is_hit(&window.input) {
            yaw -= C;
        }
        if self.rl.is_hit(&window.input) {
            roll += C;
        }
        if self.rr.is_hit(&window.input) {
            roll -= C;
        }
        if self.fwd.is_hit(&window.input) {
            throttle += C;
        }
        if self.rev.is_hit(&window.input) {
            throttle -= C;
        }

        target.handle_controls(
            pitch,
            yaw,
            roll,
            throttle,
            self.pew.is_hit(&window.input),
            window,
            world,
            registry,
        );

    }
}

struct Entity {
    body: nphysics3d::object::RigidBodyHandle<f32>,
    mesh: three::Mesh,
}

struct Ship {
    entity: Entity,
    hitpoints: u32,

    pitch: f32,
    yaw: f32,
    roll: f32,

    fire_delay: u32,
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
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,

            fire_delay: 0,
        }
    }

    fn shoot(
        &mut self,
        position: &nphysics3d::math::Isometry<f32>,
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        registry: &Registry,
    ) {
        if self.fire_delay > 0 {
            return;
        }

        self.fire_delay = 60;

        let gunpoint = nphysics3d::math::Translation::new(0.0, 0.0, -2.0);
        let bullet_position = position * gunpoint;

        registry.add_deferred(Bullet::new(
            window,
            world,
            bullet_position,
            1,
        ));
    }


}

impl Bullet {
    fn new(
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        position: nphysics3d::math::Isometry<f32>,
        damage: u32,
    ) -> Bullet {
        // FIXME these probably need to be adjusted according to the size of the
        // cylinder body
        let shape = ncollide::shape::Cone::new(0.5, 0.75);
        let mut body = nphysics3d::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);

        let direction = position * nphysics3d::math::Vector::new(0.0, 0.0, -15.0);
        body.set_lin_vel(direction);

        body.set_transformation(position);
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
            damage: damage,
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

    fn handle_controls(
        &mut self,
        pitch: f32,
        yaw: f32,
        roll: f32,
        throttle: f32,
        shoot: bool,
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        registry: &Registry,
    ) {
        if self.fire_delay > 0 {
            self.fire_delay -= 1;
        }

        {
            let mut b = self.entity.body.borrow_mut();
            let r = b.position().rotation;

            fn fade_in(cur: f32, new: f32, alpha: f32) -> f32 {
                if new != 0.0 {
                    new * alpha + cur * (1.0 - alpha)
                } else {
                    0.0
                }
            }

            self.pitch = fade_in(self.pitch, -pitch, 0.1);
            self.roll = fade_in(self.roll, roll, 0.1);
            self.yaw = fade_in(self.yaw, yaw, 0.1);

            b.append_lin_force(r * nphysics3d::math::Vector::new(0.0, 0.0, -throttle));
            b.append_ang_force(
                r * nphysics3d::math::Vector::new(self.pitch, self.yaw, self.roll),
            );
        }
        if shoot {
            let position = self.entity.body.borrow().position().clone();
            self.shoot(&position, window, world, registry)
        }

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

    fn look_at(
        &self,
        camera: &mut three::Camera<three::Perspective>,
        background: &mut three::Mesh,
    ) {
        let body = self.get_body();
        let pf: [f32; 3] = body.borrow().position().translation.vector.into();
        let rf: [f32; 4] = body.borrow().position().rotation.as_ref().coords.into();
        camera.set_transform(pf, rf, 1.0);
        background.set_position(pf);
    }

    fn handle_controls(
        &mut self,
        _pitch: f32,
        _yaw: f32,
        _roll: f32,
        _throttle: f32,
        _shoot: bool,
        _window: &mut three::Window,
        _world: &mut nphysics3d::world::World<f32>,
        _registry: &Registry,
    ) {
        unimplemented!();
    }
}

struct RegistryData {
    counter: u64,
    entities: HashMap<u64, Box<Ent>>,
}

impl RegistryData {
    fn new() -> RegistryData {
        RegistryData {
            counter: 1,
            entities: HashMap::new(),
        }
    }

    fn add_boxed(&mut self, entity: Box<Ent>) -> u64 {
        let id = self.counter;
        self.counter += 1;

        let body = entity.get_body();
        body.borrow_mut().set_user_data(Some(Box::new(id)));

        self.entities.insert(id, entity);

        id
    }

    fn add<T>(&mut self, entity: T) -> u64
    where
        T: Ent + 'static,
    {
        self.add_boxed(Box::new(entity))
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
struct Registry {
    data: Rc<RefCell<RegistryData>>,
    buffer: Rc<RefCell<Vec<Box<Ent>>>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            data: Rc::new(RefCell::new(RegistryData::new())),
            buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add<T: Ent + 'static>(&self, entity: T) -> u64 {
        self.data.borrow_mut().add(entity)
    }

    fn apply_one<F: FnMut(&mut Box<Ent>)>(&self, id: u64, f: F) {
        self.data.borrow_mut().apply_one(id, f)
    }

    fn apply_all<F: FnMut(&mut Box<Ent>)>(&self, f: F) {
        self.data.borrow_mut().apply_all(f)
    }

    fn add_deferred<T: Ent + 'static>(&self, entity: T) {
        self.buffer.borrow_mut().push(Box::new(entity));
    }

    fn execute_defferred(&self) {
        let mut buf = self.buffer.borrow_mut();
        for e in buf.drain(0..) {
            self.data.borrow_mut().add_boxed(e);
        }
    }
}

fn unpack_user_data(co: &CollisionObject) -> Option<u64> {
    if let nphysics3d::object::WorldObject::RigidBody(ref body_hndl) = co.data {
        if let Some(user_data) = body_hndl.borrow().user_data() {
            if let Some(id) = user_data.downcast_ref::<u64>() {
                return Some(*id);
            }
        }
    }

    None
}

impl ncollide::narrow_phase::ContactHandler<
    nphysics3d::math::Point<f32>,
    nphysics3d::math::Isometry<f32>,
    nphysics3d::object::WorldObject<f32>,
> for Registry {
    fn handle_contact_started(
        &mut self,
        co1: &CollisionObject,
        co2: &CollisionObject,
        _contacts: &ncollide::narrow_phase::ContactAlgorithm<
            nphysics3d::math::Point<f32>,
            nphysics3d::math::Isometry<f32>,
        >,
    ) {
        let id1 = match unpack_user_data(co1) {
            Some(id) => id,
            None => return,
        };
        let id2 = match unpack_user_data(co2) {
            Some(id) => id,
            None => return,
        };
        println!("collision detected: {}:{}", id1, id2);
    }

    fn handle_contact_stopped(&mut self, _co1: &CollisionObject, _co2: &CollisionObject) {}
}

fn make_background(factory: &mut three::Factory) -> three::Mesh {
    let geo = three::Geometry::new_sphere(90.0, 32, 32);
    factory.mesh(geo, three::Material::LineBasic { color: 0x204060 })
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

    let entities = Registry::new();

    world.register_contact_handler("entities", entities.clone());

    let player_id = entities.add(Ship::new(&mut window, &mut world, -1.1, 0.1, 0.0, 100));

    entities.add(Ship::new(&mut window, &mut world, 1.1, -0.1, 0.0, 100));
    entities.add(Ship::new(&mut window, &mut world, 2.2, -0.1, 0.1, 100));
    entities.add(Ship::new(&mut window, &mut world, 3.3, -0.1, 0.2, 100));
    entities.add(Ship::new(&mut window, &mut world, 4.4, -0.1, 0.3, 100));

    let mut background = make_background(&mut window.factory);
    background.set_position([5.0, 5.0, 5.0]); // TODO: set to pos of camera
    window.scene.add(&background);

    while window.update() {
        entities.apply_all(|e| e.update_body());

        entities.apply_one(player_id, |e| {
            control.update(e, &mut window, &mut world, &entities)
        });
        entities.execute_defferred();

        world.step(0.017);

        entities.apply_all(|e| e.update_mesh());

        // entities.retain(|e| e.alive());

        entities.apply_one(player_id, |e| e.look_at(&mut camera, &mut background));

        window.render(&camera);
    }
}
