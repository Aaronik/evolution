use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct LifeForm {
    pub id: usize,
    pub health: f32, // 0 - 1
    pub genome: Genome,
    pub neural_net: NeuralNet,
    pub hunger: f32, // 0 - 1
    pub thirst: f32, // 0 - 1
    pub location: (usize, usize),
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
            location: (0, id),
        }
    }

    /// returns a list of probablities associated with output neuron types
    pub fn calculate_output_probabilities(&self) -> Vec<(OutputNeuronType, f32)> {
        // * let running_sums = HashMap<neuron_id, sum>
        // * HashMap<neuron_id, NeuronType> (calculated at lifeform birth)

        // neuron id, running sum
        let mut running_sums: HashMap<usize, f32> = HashMap::new();

        // Game plan:
        // * For each gene from an input neuron (no matter its destination is inner or output):
        //  * record up its sums in running_sums[to]
        // * For each gene from an inner neuron to an inner neuron:
        //  * shift the gene out of the vec
        //  * If gene's FROM has no value:
        //      * push it back onto the end of the vec
        //  * else
        //      * add its sum to running_sums[to]
        //  (do this until the vec is empty)
        // * For each remaining gene, genes from inner neurons to output neurons:
        //  * Do the sums

        // TODO
        // So like, each neuron needs to have a sum, right?
        //  * For input neurons, that's its value * the genome weight.
        //  * For inner neurons, that's all its input neuron's values * their genome weights,
        //  plus the tanh of its inner neurons' existing sums.
        //
        //  So, we need to:
        //  * Get the tanh(sum(value * weight))

        // First calculate the running sums for all of the input neurons
        for gene in &self.genome.input_genes {

            // TODO We can't just add to this, right?
            // Like, what if the extent sum is to an output neuron
            // and has already been tanh'd?
            let extent_sum = running_sums.get(&gene.to);

            // The value of the input neuron
            let input_value = self
                .neural_net
                .input_neurons
                .get(&gene.from)
                .unwrap()
                .1
                .value;

            let new_sum = match extent_sum {
                Some(sum) => sum + (gene.weight * input_value),
                None => gene.weight * input_value,
            };

            running_sums.insert(gene.to, new_sum);

        }

        // Clone the inner genes
        let mut inner_genes: Vec<Gene> = vec![];
        for gene in &self.genome.inner_genes {
            inner_genes.push(gene.clone());
        }

        // Perform inner gene scheme
        while inner_genes.len() > 0 {
            let gene = inner_genes.remove(0); // Same as .shift()

            match running_sums.get(&gene.from) {
                Some(sum) => {
                    running_sums.insert(gene.to, (gene.weight * sum).tanh());
                }
                None => {
                    inner_genes.push(gene);
                }
            }
        }

        for gene in &self.genome.output_genes {
            // Two passes, first add all the sums, then run a tanh on them
            // Add up all the sums that point to it, no biggy

            // Guaranteed to have a sum because we've already visited every input
            // for output genes. (the only genes we haven't visited are output genes,
            // and they don't ever lead to each other.)
            let to_be_added = running_sums[&gene.from];
            let extent_sum = running_sums.get(&gene.to);

            let new_sum = match extent_sum {
                Some(sum) => sum + to_be_added,
                None => to_be_added,
            };

            running_sums.insert(gene.to, new_sum);
        }

        let mut final_output_values: Vec<(OutputNeuronType, f32)> = vec![];

        for gene in &self.genome.output_genes {
            // In this loop we have the value of every output neuron that has a value

            let (neuron_type, _) = &self.neural_net.output_neurons[&gene.to];
            let value = running_sums[&gene.to].tanh();

            final_output_values.push((neuron_type.clone(), value));
        }

        final_output_values
    }
}
