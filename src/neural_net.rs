use std::collections::{HashMap, HashSet};

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

    /// A mapping of neuron id to its type (Input, Inner, or Output). Handy in a number of situations.
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

        // -- Generate Neuron Id HashSets

        // (input_neuron_id, output_neuron_id)
        let mut input_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in &input_neuron_ids {
            for output_neuron_id in &output_neuron_ids {
                input_output.insert((input_neuron_id.clone(), output_neuron_id.clone()));
            }
        }

        // (inner_neuron_id, output_neuron_id)
        let mut inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in &inner_neuron_ids {
            for output_neuron_id in &output_neuron_ids {
                inner_output.insert((inner_neuron_id.clone(), output_neuron_id.clone()));
            }
        }

        let mut inner_output_neuron_ids = inner_neuron_ids.clone();
        inner_output_neuron_ids.append(&mut output_neuron_ids.clone());

        // (input_neuron_id, inner_neuron_id AND output_neuron_id)
        let mut input_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in &input_neuron_ids {
            for inner_output_neuron_id in &inner_output_neuron_ids {
                input_inner_output.insert((input_neuron_id.clone(), inner_output_neuron_id.clone()));
            }
        }

        // (inner_neuron_id, inner_neuron_id AND output_neuron_id)
        let mut inner_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in &inner_neuron_ids {
            for inner_output_neuron_id in &inner_output_neuron_ids {
                inner_inner_output.insert((inner_neuron_id.clone(), inner_output_neuron_id.clone()));
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
        NeuralNet {
            input_neurons: self.input_neurons.clone(),
            inner_neurons: self.inner_neurons.clone(),
            output_neurons: self.output_neurons.clone(),
        }
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
    Eat,
    Drink,
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
