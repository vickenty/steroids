use three;
use nphysics3d;
use ncollide;

use Ent;
use Entity;

pub struct Bullet {
    entity: Entity,
    ttl: u32,
    damage: u32,
}

impl Bullet {
    pub fn new(
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
            ttl: 300,
            damage: damage,
        }
    }
}

impl Ent for Bullet {
    fn update_logic(&mut self) {
        if self.ttl > 0 {
            self.ttl -= 1;
        }
    }

    fn update_mesh(&mut self) {
        self.entity.update_mesh();
    }

    fn set_entity_id(&mut self, id: u64) {
        self.entity.set_entity_id(id);
    }
}
