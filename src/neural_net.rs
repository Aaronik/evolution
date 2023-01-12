use std::collections::HashMap;
use rand::{thread_rng, Rng};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct NeuralNet {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
}

impl NeuralNet {
    pub fn new(num_inner_neurons: usize) -> Self {
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();

        for (idx, neuron_member) in InputNeuronType::iter().enumerate() {
            let neuron = InputNeuron {
                // Assuming there'll never be more than 100 input neuron types, we'll do this
                // to assure a different id from the output neurons
                id: idx + 100,

                // TODO should this be random, or..?
                value: 0.0,
            };
            input_neurons.insert(neuron.id, (neuron_member, neuron));
        }

        for (idx, neuron_member) in OutputNeuronType::iter().enumerate() {
            let neuron = OutputNeuron { id: idx + 200 };
            output_neurons.insert(neuron.id, (neuron_member, neuron));
        }

        for idx in 0..num_inner_neurons {
            let neuron = InnerNeuron { id: idx + 300 };
            inner_neurons.insert(neuron.id, neuron);
        }

        Self {
            input_neurons,
            output_neurons,
            inner_neurons,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum InputNeuronType {
    DirectionToFood,
    DistanceToFood,
    DirectionToWater,
    DistanceToWater,
    DirectionToDanger,
    DistanceToDanger,
    DirectionToHealthiestLF,
    DistanceToHealthiestLF,
    HealthiestLFHealth,
    DirectionToClosestLF,
    DistanceToClosestLF,
    ClosestLFHealth,
    Health,
    Hunger,
    Thirst,
    PopulationDensity,
    NeighborhoodDensity,
    Random,
    Oscillator,
}

#[derive(Debug, Default)]
pub struct InputNeuron {
    // TODO One struct per type?
    pub id: usize,
    pub value: f32, // 0.0 - 1.0
}

#[derive(Debug, EnumIter)]
pub enum OutputNeuronType {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Attack,
    Mate,
    Eat,
    Drink,
}

#[derive(Debug, Default)]
pub struct OutputNeuron {
    pub id: usize,
}

#[derive(Debug)]
pub struct InnerNeuron {
    pub id: usize,
}

pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

// Issues:
// * You can't have two of the same from/to pair

// TODO
// * Maybe instead of Gene, this is a Genome that spawns Genes. Then easy track
// could be kept of which from/to pairs are used.
impl Gene {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        Self { from, to, weight }
    }

    pub fn new_random(
        input_ids: &Vec<usize>,
        inner_ids: &Vec<usize>,
        output_ids: &Vec<usize>,
    ) -> Self {
        // from: randomly from input_ids and inner_ids
        // to: randomly from inner_ids and output_ids
        // weight: randomly from -4 to 4

        let from_idx = thread_rng().gen_range(0..input_ids.len() + inner_ids.len());
        let to_idx = thread_rng().gen_range(0..inner_ids.len() + output_ids.len());

        let from: usize;
        if from_idx < input_ids.len() {
            from = input_ids.get(from_idx).unwrap().clone();
        } else {
            from = inner_ids.get(from_idx - input_ids.len()).unwrap().clone();
        }

        let to: usize;
        if to_idx < inner_ids.len() {
            to = inner_ids.get(to_idx).unwrap().clone();
        } else {
            to = output_ids.get(to_idx - inner_ids.len()).unwrap().clone();
        }

        let weight = thread_rng().gen_range(-4.0..4.0);

        Self { from, to, weight }
    }
}

