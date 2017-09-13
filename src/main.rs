extern crate three;
extern crate nphysics3d;
extern crate ncollide;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod ship;
mod bullet;
mod controller;

use ship::Ship;
use bullet::Bullet;
use controller::Controller;

struct Entity {
    body: nphysics3d::object::RigidBodyHandle<f32>,
    mesh: three::Mesh,
}

impl Entity {
    fn update_mesh(&mut self) {
        let body = self.body.borrow();
        let pos = body.position();

        let pf: [f32; 3] = pos.translation.vector.into();
        let rf: [f32; 4] = pos.rotation.as_ref().coords.into();

        self.mesh.set_transform(pf, rf, 1.0);
    }

    fn set_entity_id(&mut self, id: u64) {
        self.body.borrow_mut().set_user_data(Some(Box::new(id)));
    }
}


type CollisionObject = ncollide::world::CollisionObject<
    nphysics3d::math::Point<f32>,
    nphysics3d::math::Isometry<f32>,
    nphysics3d::object::WorldObject<f32>,
>;

trait Ent {
    fn update_logic(&mut self);

    fn update_mesh(&mut self);

    fn set_entity_id(&mut self, id: u64);

    fn look_at(
        &self,
        camera: &mut three::Camera<three::Perspective>,
        background: &mut three::Mesh,
    ) {}

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

    fn add_boxed(&mut self, mut entity: Box<Ent>) -> u64 {
        let id = self.counter;
        self.counter += 1;

        entity.set_entity_id(id);

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
pub struct Registry {
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
        entities.apply_all(|e| e.update_logic());


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
