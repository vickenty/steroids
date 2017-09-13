use nphysics3d;
use three;

use Registry;
use Ent;

pub struct Controller {
    pub pu: three::Button,
    pub pd: three::Button,
    pub yl: three::Button,
    pub yr: three::Button,
    pub rl: three::Button,
    pub rr: three::Button,
    pub fwd: three::Button,
    pub rev: three::Button,
    pub pew: three::Button,
}

const C: f32 = 1.0;

impl Controller {
    pub fn update(
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
