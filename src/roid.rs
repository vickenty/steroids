use three;
use nphysics3d;
use ncollide;

use Ent;
use Entity;

pub struct Roid {
    entity: Entity,
    hp: u32,
}

impl Roid {
    pub fn new(
        window: &mut three::Window,
        world: &mut nphysics3d::world::World<f32>,
        x: f32,
        y: f32,
        z: f32,
        hp: u32,
    ) -> Roid {
        // FIXME these probably need to be adjusted according to the size of the
        // cylinder body
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

        Roid {
            entity: Entity {
                body: hndl,
                mesh: mesh,
            },

            hp,
        }
    }
}

impl Ent for Roid {
    fn update_logic(&mut self) {}

    fn update_mesh(&mut self) {
        self.entity.update_mesh();
    }

    fn set_entity_id(&mut self, id: u64) {
        self.entity.set_entity_id(id);
    }
}
