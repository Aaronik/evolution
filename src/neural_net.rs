use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Builds AND houses data structures that help for speedy neural net related calculations
/// Meant to be a singleton that itself builds neural nets and houses these helpers.
/// Must be instantiated to generate data structures.
pub struct NeuralNetHelper {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,

    /// A mapping of neuron id to its type. Handy in a number of situations.
    pub neuron_type_map: HashMap<usize, NeuronType>,

    /// A list of ids to facilitate ease of genome creation
    pub input_neuron_ids: Vec<usize>,
    /// A list of ids to facilitate ease of genome creation
    pub inner_neuron_ids: Vec<usize>,
    /// A list of ids to facilitate ease of genome creation
    pub output_neuron_ids: Vec<usize>,
    /// A list of ids to facilitate ease of genome creation. This is a union
    /// of inner neuron and output neuron ids.
    pub inner_output_neuron_ids: Vec<usize>,

    /// Helper to facilitate rapid and accurate creation of Genomes
    /// This is a hashset of neuron ids that is meant to have elements
    /// removed from it when a genome is created so the genome can safely
    /// pick random neuron ids without duplicating pairs. This way no two
    /// genes will have the same from and to values.
    /// HashSet<(input_neuron_id, output_neuron_id)>
    pub input_output: HashSet<(usize, usize)>,

    /// Helper to facilitate rapid and accurate creation of Genomes
    /// This is a hashset of neuron ids that is meant to have elements
    /// removed from it when a genome is created so the genome can safely
    /// pick random neuron ids without duplicating pairs. This way no two
    /// genes will have the same from and to values.
    /// HashSet<(inner_neuron_id, output_neuron_id)>
    pub inner_output: HashSet<(usize, usize)>,

    /// Helper to facilitate rapid and accurate creation of Genomes
    /// This is a hashset of neuron ids that is meant to have elements
    /// removed from it when a genome is created so the genome can safely
    /// pick random neuron ids without duplicating pairs. This way no two
    /// genes will have the same from and to values.
    /// HashSet<(input_neuron_id, inner_neuron_id AND output_neuron_id)>
    pub input_inner_output: HashSet<(usize, usize)>,

    /// Helper to facilitate rapid and accurate creation of Genomes
    /// This is a hashset of neuron ids that is meant to have elements
    /// removed from it when a genome is created so the genome can safely
    /// pick random neuron ids without duplicating pairs. This way no two
    /// genes will have the same from and to values.
    /// HashSet<(inner_neuron_id, inner_neuron_id AND output_neuron_id)>
    pub inner_inner_output: HashSet<(usize, usize)>,
}

impl NeuralNetHelper {
    pub fn new(num_inner_neurons: usize) -> Self {
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();
        let mut neuron_type_map = HashMap::new();

        // -- Generate Neurons

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
            let neuron = InnerNeuron { id: idx + 200 };
            inner_neurons.insert(neuron.id, neuron);
            neuron_type_map.insert(neuron.id, NeuronType::InnerNeuron);
        }

        for (idx, neuron_member) in OutputNeuronType::iter().enumerate() {
            let neuron = OutputNeuron { id: idx + 300 };
            output_neurons.insert(neuron.id, (neuron_member, neuron));
            neuron_type_map.insert(neuron.id, NeuronType::OutputNeuron);
        }

        // -- Generate Neuron Ids

        let input_neuron_ids: Vec<usize> = input_neurons.keys().map(|k| *k).collect();
        let inner_neuron_ids: Vec<usize> = inner_neurons.keys().map(|k| *k).collect();
        let output_neuron_ids: Vec<usize> = output_neurons.keys().map(|k| *k).collect();

        // -- Generate Neuron Id HashSets

        // (input_neuron_id, output_neuron_id)
        let input_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in input_neuron_ids {
            for output_neuron_id in output_neuron_ids {
                input_output.insert((input_neuron_id, output_neuron_id));
            }
        }

        // (inner_neuron_id, output_neuron_id)
        let inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in inner_neuron_ids {
            for output_neuron_id in output_neuron_ids {
                inner_output.insert((inner_neuron_id, output_neuron_id));
            }
        }

        let inner_output_neuron_ids = inner_neuron_ids.clone();
        inner_output_neuron_ids.append(&mut output_neuron_ids.clone());

        // (input_neuron_id, inner_neuron_id AND output_neuron_id)
        let input_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in input_neuron_ids {
            for inner_output_neuron_id in inner_output_neuron_ids {
                input_inner_output.insert((input_neuron_id, inner_output_neuron_id));
            }
        }

        // (inner_neuron_id, inner_neuron_id AND output_neuron_id)
        let inner_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in inner_neuron_ids {
            for inner_output_neuron_id in inner_output_neuron_ids {
                inner_inner_output.insert((inner_neuron_id, inner_output_neuron_id));
            }
        }

        Self {
            input_neurons,
            output_neurons,
            inner_neurons,
            neuron_type_map,
            input_neuron_ids,
            inner_neuron_ids,
            output_neuron_ids,
            input_output,
            input_inner_output,
            inner_output,
            inner_inner_output,
            inner_output_neuron_ids,
        }
    }

    pub fn spawn(&self) -> NeuralNet {

        // Clone HashMaps
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();

        for (key, val) in self.input_neurons.iter() {
            input_neurons.insert(*key, *val);
        }

        for (key, val) in self.inner_neurons.iter() {
            inner_neurons.insert(*key, *val);
        }

        for (key, val) in self.output_neurons.iter() {
            output_neurons.insert(*key, *val);
        }

        NeuralNet {
            input_neurons,
            inner_neurons,
            output_neurons,
        }
    }
}

#[derive(Debug)]
pub struct NeuralNet {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,
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
