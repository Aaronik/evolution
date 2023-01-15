use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone)]
pub struct LifeForm {
    pub id: usize,
    pub health: f32, // 0 - 1
    pub genome: Genome,
    pub neural_net: NeuralNet,
    pub hunger: f32, // 0 - 1
    pub thirst: f32, // 0 - 1
    pub location: (usize, usize),
    pub lifespan: usize, // How many tics this one has lived for
}

impl LifeForm {
    pub fn new(id: usize, genome_size: usize, neural_net_helper: &NeuralNetHelper) -> Self {
        let neural_net = neural_net_helper.spawn();

        let genome_props = GenomeProps {
            size: genome_size,
            neural_net_helper,
        };

        let genome = Genome::new(genome_props);

        Self {
            id,
            genome,
            neural_net,
            health: 1.0,
            hunger: 0.0,
            thirst: 0.0,
            lifespan: 0,
            location: (id + 10, id + 10),
        }
    }

    /// returns a list of probablities associated with output neuron types
    pub fn calculate_output_probabilities(
        &self,
        nnh: &NeuralNetHelper,
    ) -> Vec<(OutputNeuronType, f32)> {

        // neuron id, running sum
        let mut running_sums: HashMap<usize, f32> = HashMap::new();

        // Idea here is to go through each gene in the ordered genes here and if there's an entry
        // in the running sums map, add that... Trailing off b/c I have another idea.
        for gene in &self.genome.ordered_genes {

            if let NeuronType::InputNeuron = nnh.neuron_type(&gene.from) {
                running_sums.insert(gene.from, self.neural_net.input_neurons[&gene.from].1.value);
            }

            if let Some(sum) = running_sums.get(&gene.from) {
                *running_sums.entry(gene.to).or_insert(0.0) += sum.tanh() * gene.weight;
            }

        }

        let mut final_output_values: Vec<(OutputNeuronType, f32)> = vec![];

        for (neuron_id, sum) in running_sums {
            if let NeuronType::OutputNeuron = nnh.neuron_type(&neuron_id) {

            }

            if let Some((neuron_type, _)) = nnh.output_neurons.get(&neuron_id) {
                final_output_values.push((neuron_type.clone(), sum.tanh()));
            }
        }

        final_output_values
    }
}
