use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct NeuralNet {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,
    pub neuron_type_map: HashMap<usize, NeuronType>,
}

impl NeuralNet {
    pub fn new(num_inner_neurons: usize) -> Self {
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();
        let mut neuron_type_map = HashMap::new();

        for (idx, neuron_member) in InputNeuronType::iter().enumerate() {
            let neuron = InputNeuron {
                // Assuming there'll never be more than 100 input neuron types, we'll do this
                // to assure a different id from the output neurons
                id: idx + 100,

                // TODO should this be random, or..?
                value: 0.0,
            };
            input_neurons.insert(neuron.id, (neuron_member, neuron));
            neuron_type_map.insert(neuron.id, NeuronType::InputNeuron);
        }

        for idx in 0..num_inner_neurons {
            let neuron = InnerNeuron { id: idx + 300 };
            inner_neurons.insert(neuron.id, neuron);
            neuron_type_map.insert(neuron.id, NeuronType::InnerNeuron);
        }

        for (idx, neuron_member) in OutputNeuronType::iter().enumerate() {
            let neuron = OutputNeuron { id: idx + 200 };
            output_neurons.insert(neuron.id, (neuron_member, neuron));
            neuron_type_map.insert(neuron.id, NeuronType::OutputNeuron);
        }

        Self {
            input_neurons,
            output_neurons,
            inner_neurons,
            neuron_type_map,
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

#[derive(Debug)]
pub enum NeuronType {
    InputNeuron,
    InnerNeuron,
    OutputNeuron,
}

