use std::collections::HashMap;

use crate::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Gene {
    pub id: usize,
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

pub enum GeneType {
    InputGene,
    InnerGene,
    OutputGene,
}

#[derive(Debug, Clone)]
pub struct GenomeProps<'a> {
    pub size: usize,
    pub neural_net_helper: &'a NeuralNetHelper,
}

type Seed = HashMap<usize, Vec<Gene>>;

#[derive(Debug, Clone)]
pub struct Genome {
    /// An ordered list of duplicated genes constructed to enable fast and easy neural net calculations.
    /// Overwritten by calling genome.recompute_ordered_genes.
    pub ordered_genes: Vec<Gene>,

    /// An unordered unique list of genes representing one of each gene the genome has. Use this
    /// list to modify the genes in a genome, then call genome.recompute_ordered_genes().
    pub genes: Vec<Gene>,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let mut genes: Vec<Gene> = vec![];

        for id in 0..props.size {
            genes.push(Gene {
                id,
                from: props.neural_net_helper.random_from_neuron(),
                to: props.neural_net_helper.random_to_neuron(),
                weight: Genome::random_weight()
            });
        }

        Self {
            ordered_genes: Genome::compute_ordered_genes(&genes, props.neural_net_helper),
            genes,
        }
    }

    /// Takes a vector of unique genes, returns a vector of genes duplicated in the correct order
    /// to walk them to do the neural net calculation. Going from one end of the returned vector to
    /// the other adding the sums of the neurons for the genes already seen and taking the tanh of
    /// them, etc, will be like starting at the input neurons and following their connections, then
    /// following the connection of the next neuron, etc, recursively, for a specified maximum
    /// number sf times per gene, in case there is a loop. This is the best way I could think of to
    /// approximate biological neural nets.
    pub fn compute_ordered_genes(genes: &Vec<Gene>, nnh: &NeuralNetHelper) -> Vec<Gene> {
        // TODO I don't think this is necessary any more, we can make this an arbitrary number.
        // No matter what, all the genes that can be followed will get followed at least once.
        let max_gene_follows = nnh.inner_neurons.len() + 2;

        let mut seed: Seed = HashMap::new();
        let mut inputs: Vec<usize> = vec![];
        for gene in genes {
            if let GeneType::InputGene = Genome::classify_gene(nnh, &gene) {
                inputs.push(gene.from);
            }

            seed.entry(gene.from).or_insert(vec![]).push(gene.clone());
        }

        Genome::generate_ordered_from_seed_and_inputs(&seed, &inputs, max_gene_follows)
    }

    /// After the genome.genes vector has been messed with, call this to rebuild the data
    /// structures necessary to efficiently calculate this genome's neural net probabilities.
    pub fn recompute_ordered_genes(&mut self, nnh: &NeuralNetHelper) {
        self.ordered_genes = Genome::compute_ordered_genes(&self.genes, nnh);
    }

    pub fn random_weight() -> f32 {
        thread_rng().gen_range(-4.0..=4.0)
    }

    /// An InputGene is one that comes from an input. These are where you start when you do a
    /// recursive calculation to find out the final output neuron probabilities. An InnerGene is
    /// one that goes from an inner neuron to another inner neuron. An OutputGene is from an
    /// InnerNeuron to an OutputNeuron.
    pub fn classify_gene(nnh: &NeuralNetHelper, gene: &Gene) -> GeneType {
        if let NeuronType::InputNeuron = nnh.neuron_type(&gene.from) {
            return GeneType::InputGene;
        } else if let NeuronType::InnerNeuron = nnh.neuron_type(&gene.from) {
            if let NeuronType::InnerNeuron = nnh.neuron_type(&gene.to) {
                return GeneType::InnerGene;
            }
        }

        return GeneType::OutputGene;
    }

    /// For each input, follow its TO chain up to a maximum number of times. If we went forever,
    /// we'd get stuck in loops of one neuron to itself or to another which then points back to
    /// the first. So we make that max number the number of inner neurons + 1 for the input gene
    /// and +1 for an output, that way we're guaranteed to follow a chain for as long as it can
    /// be (if it started with the input, went to the first inner, to the second, third, etc,
    /// and ended at the output neuron), but not so long that it takes up more time than it
    /// needs to. In a real biological neural net, I figure that's similar to how it works.
    /// There can be recursive inputs from one neuron to another, but that signal will
    /// eventually fade. I believe this is what gives the "frequency of consciousness", this
    /// fading limit. Apparently our consciousness operates at around 40hz!
    fn generate_ordered_from_seed_and_inputs(
        seed: &Seed,
        inputs: &Vec<usize>,
        max_gene_follows: usize,
    ) -> Vec<Gene> {
        // gene id, number of times seen
        let mut follow_count: HashMap<usize, usize> = HashMap::new();

        let mut ordered_genes: Vec<Gene> = vec![];

        // * Append all genes from each input_id to ordered_genes, incrementing follow_count
        // for each TO.
        for input_id in inputs {
            for gene in &seed[&input_id] {
                ordered_genes.push(gene.clone());
            }
        }

        // * Then go through each member of ordered_genes, append all genes for each FROM
        // to ordered genes, again incrementing follow_count for each.
        // * If at any time follow_count is above the limit, don't add that gene again.
        // * Do this until the loop iterated the same number of times as the length of
        // ordered_genes, noting that that length will be growing as the loop proceeds.
        let mut index = 0;
        while index < ordered_genes.len() {
            let gene = &ordered_genes[index];
            // only if it hasn't exceeded its follow_count
            if let Some(count) = follow_count.get_mut(&gene.id) {
                if *count < max_gene_follows {
                    for gene in &seed[&gene.from] {
                        ordered_genes.push(gene.clone());
                    }

                    *count += 1;
                }
            } else {
                follow_count.insert(gene.id, 1);
                for gene in &seed[&gene.from] {
                    ordered_genes.push(gene.clone());
                }
            }

            index += 1;
        }

        ordered_genes
    }
}

// TODO Test ensure regenerating ordered_genes has the same thing each time
