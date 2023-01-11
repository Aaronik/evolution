use crate::*;

pub struct LifeForm {
    pub id: usize,
    pub health: f32, // 0 - 1
    pub genome: Vec<Gene>,
    pub neural_net: NeuralNet,
    pub hunger: f32, // 0 - 1
    pub thirst: f32, // 0 - 1
}
