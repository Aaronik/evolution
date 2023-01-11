use crate::*;

pub struct Runner {
    pub world: World,
}

impl Runner {
    pub fn mate(_lf1: LifeForm, _lf2: LifeForm) -> LifeForm {
        todo!()
    }

    // with a chance of `rate`(Neuron), flip a single bit
    pub fn mutate(_lf: LifeForm, _rate: f32) {
        todo!()
    }

    // triggers the effect on the world
    pub fn trigger_effect() {
        todo!()
    }

    // does the neuron calculations and runs the output effects
    pub fn step_lifeform(_lf: LifeForm) {
        todo!()
    }
}

