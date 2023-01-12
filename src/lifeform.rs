use crate::*;

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
    pub fn new(id: usize, genome_size: usize, num_inner_neurons: usize) -> Self {
        let neural_net = NeuralNet::new(num_inner_neurons);

        let genome_props = GenomeProps {
            size: genome_size,
            neural_net: &neural_net,
        };

        let mut genome = Genome::new(genome_props);

        // TODO This whole loop is gonna be gone and the bs processed_genome method
        // is kaputz and Genome::new is going to take care of all of that efficiently.
        loop {
            let (is_valid, processed_genome) = LifeForm::process_genome(genome);

            if is_valid {
                genome = processed_genome;
                break;
            }
        }

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

    // TODO This is also inefficient. If this could be moved to genome and proactively
    // built up instead of retroactively enforced we'd save statistically some time on lifeform
    // generation.
    // Returns a tuple of whether the genome is valid, and the genome itself.
    // Processing will happen on the genome to remove genes that will have no
    // effect.
    fn process_genome(neural_net: &NeuralNet, genome: Genome) -> (bool, Genome) {
        // * Remove all genes that come from an inner neuron that has no input.
        // * If there are no genes left, lifeform is invalid.

        // If there are no genes from input neurons OR no genes to
        // output neurons, the lifeform is invalid.
        if genome.input_genes.len() == 0 {
            return (false, genome);
        }

        if genome.output_genes.len() == 0 {
            return (false, genome);
        }

        // Remove all genes that lead to an inner neuron that has no output (no genes have a from
        // for that inner neuron)
        // Go through each inner neuron in the neural net:
        // * If there're no genes with a FROM value of the neuron's id:
        //  * Remove all genes with a TO value to that neuron's id
        genome.out

    }

    /// returns a list of probablities associated with output neuron types
    pub fn calculate_output_probabilities(&self) -> Vec<(OutputNeuronType, f32)> {
        // * let running_sums = HashMap<neuron_id, sum>
        // * HashMap<neuron_id, NeuronType> (calculated at lifeform birth)

        // TODO in Genome
        // First, we validate:
        //
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

        // This means we'll need the genes to be sorted into trhee pools, those from
        // input neurons, those from inner neurons to inner neurons, and those from
        // inner neurons to output neurons.
    }
}
