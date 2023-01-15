use std::collections::HashMap;

use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Builds AND houses data structures that help for speedy neural net related calculations
/// Meant to be a singleton that itself builds neural nets and houses these helpers.
/// Must be instantiated to generate data structures.
#[derive(Debug)]
pub struct NeuralNetHelper {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,

    neuron_type_map: HashMap<usize, NeuronType>,
    input_inner_neuron_ids: Vec<usize>,
    inner_output_neuron_ids: Vec<usize>,
}

impl NeuralNetHelper {
    pub fn new(num_inner_neurons: usize) -> Self {
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();
        let mut neuron_type_map = HashMap::new();

        // -- Generate Neurons

        for (idx, neuron_member) in InputNeuronType::iter().enumerate() {
            // Assuming there'll never be more than 100 input neuron types, we'll do this
            // to assure a different id from the output neurons
            let id = idx + 100;
            let neuron = InputNeuron { id, value: 0.0 };
            input_neurons.insert(id, (neuron_member, neuron));
            neuron_type_map.insert(id, NeuronType::InputNeuron);
        }

        for idx in 0..num_inner_neurons {
            let id = idx + 200;
            let neuron = InnerNeuron { id };
            inner_neurons.insert(id, neuron);
            neuron_type_map.insert(id, NeuronType::InnerNeuron);
        }

        for (idx, neuron_member) in OutputNeuronType::iter().enumerate() {
            let id = idx + 300;
            let neuron = OutputNeuron { id };
            output_neurons.insert(id, (neuron_member, neuron));
            neuron_type_map.insert(id, NeuronType::OutputNeuron);
        }

        // -- Generate Neuron Ids

        let input_neuron_ids: Vec<usize> = input_neurons.keys().map(|k| *k).collect();
        let inner_neuron_ids: Vec<usize> = inner_neurons.keys().map(|k| *k).collect();
        let output_neuron_ids: Vec<usize> = output_neurons.keys().map(|k| *k).collect();

        let mut input_inner_neuron_ids: Vec<usize> = input_neuron_ids.clone();
        input_inner_neuron_ids.append(&mut inner_neuron_ids.clone());
        let mut inner_output_neuron_ids: Vec<usize> = inner_neuron_ids.clone();
        inner_output_neuron_ids.append(&mut output_neuron_ids.clone());

        Self {
            input_neurons,
            output_neurons,
            inner_neurons,
            neuron_type_map,
            input_inner_neuron_ids,
            inner_output_neuron_ids,
        }
    }

    /// Spawn a new neural net based off the blueprint that was created at instantiation time.
    /// Cloning saves compute resources at the expense of memory, which is perfect for us.
    pub fn spawn(&self) -> NeuralNet {
        NeuralNet {
            input_neurons: self.input_neurons.clone(),
            inner_neurons: self.inner_neurons.clone(),
            output_neurons: self.output_neurons.clone(),
        }
    }

    /// Returns a neuron id randomly chosen from input neurons unioned with inner neurons.
    /// This is all the places where a gene can start from.
    pub fn random_from_neuron(&self) -> usize {
        let idx = thread_rng().gen_range(0..self.input_inner_neuron_ids.len());
        self.input_inner_neuron_ids[idx].clone()
    }

    /// Returns a neuron id randomly chosen from inner neurons unioned with output neurons.
    /// This is all the places where a gene can end, aka go to.
    pub fn random_to_neuron(&self) -> usize {
        let idx = thread_rng().gen_range(0..self.inner_output_neuron_ids.len());
        self.inner_output_neuron_ids[idx].clone()
    }

    pub fn neuron_type(&self, neuron_id: &usize) -> &NeuronType {
        &self.neuron_type_map[neuron_id]
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNet {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,
}

#[derive(Debug, EnumIter, Clone)]
pub enum InputNeuronType {
    DirectionToFood = 100,
    DistanceToFood = 101,
    DirectionToWater = 102,
    DistanceToWater = 103,
    DirectionToDanger = 104,
    DistanceToDanger = 105,
    DirectionToHealthiestLF = 106,
    DistanceToHealthiestLF = 107,
    HealthiestLFHealth = 108,
    DirectionToClosestLF = 109,
    DistanceToClosestLF = 110,
    ClosestLFHealth = 111,
    Health = 112,
    Hunger = 113,
    Thirst = 114,
    PopulationDensity = 115,
    NeighborhoodDensity = 116,
    Random = 117,
    Oscillator = 118,
}

#[derive(Debug, Default, Clone)]
pub struct InputNeuron {
    pub id: usize,
    pub value: f32,
}

#[derive(Debug, EnumIter, Clone)]
pub enum OutputNeuronType {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Attack,
    Mate,
    MoveRandom,
}

#[derive(Debug, Default, Clone)]
pub struct OutputNeuron {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct InnerNeuron {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub enum NeuronType {
    InputNeuron,
    InnerNeuron,
    OutputNeuron,
}
