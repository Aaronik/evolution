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
    pub fn calculate_output_probabilities(&self) -> Vec<(OutputNeuronType, f32)> {
        // * let running_sums = HashMap<neuron_id, sum>
        // * HashMap<neuron_id, NeuronType> (calculated at lifeform birth)

        // neuron id, running sum
        let mut running_sums: HashMap<usize, f32> = HashMap::new();

        let mut final_output_values: Vec<(OutputNeuronType, f32)> = vec![];

        // For each input, follow its TO chain up to a maximum number of times. If we went forever,
        // we'd get stuck in loops of one neuron to itself or to another which then points back to
        // the first. So we make that max number the number of inner neurons + 1 for the input gene
        // and +1 for an output, that way we're guaranteed to follow a chain for as long as it can
        // be (if it started with the input, went to the first inner, to the second, third, etc,
        // and ended at the output neuron), but not so long that it takes up more time than it
        // needs to. In a real biological neural net, I figure that's similar to how it works.
        // There can be recursive inputs from one neuron to another, but that signal will
        // eventually fade. I believe this is what gives the "frequency of consciousness", this
        // fading limit. Apparently our consciousness operates at around 40hz!
        let max_gene_follows = self.neural_net.inner_neurons.len() + 2;

        // * Start a loop of n times where n = the length of the longest gene
        // list.
        // * In each iteration i,
        //  * For each gene list starting at
        //  * get reference to the ith element

        // gene id, number of times seen
        let mut follow_count: HashMap<usize, usize> = HashMap::new();

        let mut ordered_genes: Vec<&Gene> = vec![];

        // * Append all genes from each input_id to ordered_genes, incrementing follow_count
        // for each TO.
        // * Then go through each member of ordered_genes, append all genes for each FROM
        // to ordered genes, again incrementing follow_count for each.
        // * If at any time follow_count is above the limit, don't add that gene again.
        // * Do this until the loop iterated the same number of times as the length of
        // ordered_genes, noting that that length will be growing as the loop proceeds.
        // I BELIEVE this can be done one time at startup actually instead of being done
        // here. Yes every new genome only has to do this one time.
        for input_id in &self.genome.inputs {
            for gene in &self.genome.seed[input_id] {
                ordered_genes.push(gene);
            }
        }

        let mut index = 0;
        while index < ordered_genes.len() {
            let gene = ordered_genes[index];
            // only if it hasn't exceeded its follow_count
            if let Some(count) = follow_count.get_mut(&gene.id) {
                if *count < max_gene_follows {
                    for gene in &self.genome.seed[&gene.from] {
                        ordered_genes.push(gene);
                    }

                    *count += 1;
                }
            } else {
                follow_count.insert(gene.id, 1);
                for gene in &self.genome.seed[&gene.from] {
                    ordered_genes.push(gene);
                }
            }

            index += 1;
        }

        println!("{} ordered_genes", ordered_genes.len());

        // // First calculate the running sums for all of the input neurons
        // for gene in &self.genome.input_genes {
        //     let extent_sum = running_sums.get(&gene.to);

        //     // The value of the input neuron
        //     let input_value = self.neural_net.input_neurons[&gene.from].1.value;

        //     let new_sum = match extent_sum {
        //         Some(sum) => sum + (gene.weight * input_value),
        //         None => gene.weight * input_value,
        //     };

        //     running_sums.insert(gene.to, new_sum);
        // }

        // // TODO This can be created at genome creation time instead of the three vecs

        // // Clone the inner genes
        // let mut inner_genes = self.genome.inner_genes.clone();

        // // Perform inner gene scheme: For every gene that goes inner neuron to inner neuron,
        // // as are called inner genes in here, use up the gene if its originator (from value)
        // // already has a sum, tanh that and add it to the to value's sum. If it doesn't yet
        // // have a sum, file it for later inspection, understanding that it may get a sum as
        // // we move through the others. Limit the number of times looping through it all to
        // // loop through every gene the same number of times as we have inner neurons. This should
        // // allow for completion of even the longest chains that have a completing path.
        // // TODO a better scheme here, which would respect more the recursive properties of
        // // neuron loops, would be to follow signal paths. So start with an input gene, follow
        // // the TO, add to the sum of that TO, then find all the genes that come from that TO
        // // and repeat the process. A nice data structure to help with this could be:
        // // HashMap<usize, Vec<Gene>>, where the usize is the neuron id, and the vec of genes
        // // is every gene that has a FROM value of that neuron id.
        // let mut count = 0;
        // while inner_genes.len() > 0 && count < self.neural_net.inner_neurons.len() * inner_genes.len() {
        //     let gene = inner_genes.remove(0); // Same as .shift()

        //     match running_sums.get(&gene.from) {
        //         Some(sum) => {
        //             running_sums.insert(gene.to, (gene.weight * sum).tanh());
        //         }
        //         None => {
        //             inner_genes.push(gene);
        //         }
        //     }

        //     count += 1;
        // }

        // for gene in &self.genome.output_genes {
        //     // Two passes, first add all the sums, then run a tanh on them
        //     // Add up all the sums that point to it, no biggy

        //     // Guaranteed to have a sum because we've already visited every input
        //     // for output genes. (the only genes we haven't visited are output genes,
        //     // and they don't ever lead to each other.)
        //     let to_be_added_opt = running_sums.get(&gene.from);

        //     // There may well be a connection from an inner neuron that has no inputs!
        //     if let None = to_be_added_opt {
        //         continue;
        //     }

        //     let to_be_added = *to_be_added_opt.unwrap();

        //     let extent_sum = running_sums.get(&gene.to);

        //     let new_sum: f32 = match extent_sum {
        //         Some(sum) => sum + to_be_added,
        //         None => to_be_added,
        //     };

        //     running_sums.insert(gene.to, new_sum);
        // }

        // let mut final_output_values: Vec<(OutputNeuronType, f32)> = vec![];

        // for gene in &self.genome.output_genes {
        //     // In this loop we have the value of every output neuron that has a value

        //     let (neuron_type, _) = &self.neural_net.output_neurons[&gene.to];

        //     match running_sums.get(&gene.to) {
        //         Some(sum) => final_output_values.push((neuron_type.clone(), sum.tanh())),
        //         None => (),
        //     }
        // }

        final_output_values
    }
}
