use three;
use nphysics3d;
use ncollide;

use Ent;
use Entity;
use Registry;
use bullet::Bullet;

pub struct Ship {
    entity: Entity,
    hitpoints: u32,

    pitch: f32,
    yaw: f32,
    roll: f32,

    fire_delay: u32,
}

impl Ship {
    pub fn new(
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

        registry.add_deferred(Bullet::new(window, world, bullet_position, 1));
    }
}

impl Ent for Ship {
    fn look_at(
        &self,
        camera: &mut three::Camera<three::Perspective>,
        background: &mut three::Mesh,
    ) {
        let body = self.entity.body.borrow();
        let pf: [f32; 3] = body.position().translation.vector.into();
        let rf: [f32; 4] = body.position().rotation.as_ref().coords.into();
        camera.set_transform(pf, rf, 1.0);
        background.set_position(pf);
    }

    fn update_logic(&mut self) {
        if self.fire_delay > 0 {
            self.fire_delay -= 1;
        }
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

            b.clear_forces();
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

    fn update_mesh(&mut self) {
        self.entity.update_mesh();
    }

    fn set_entity_id(&mut self, id: u64) {
        self.entity.set_entity_id(id);
    }
}
