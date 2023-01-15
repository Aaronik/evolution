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
    /// An ordered list of genes constructed to enable fast and easy neural net calculations
    pub ordered_genes: Vec<Gene>,

    /// The seed for a gene tree. This can be walked like recursive tree.
    seed: Seed,

    /// How many times we're going to be repeat calculations on a specific gene.
    /// Some genes create recursive loops, so we can't be following those forever!
    max_gene_follows: usize,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let nnh = props.neural_net_helper;

        let mut inputs: Vec<usize> = vec![];
        let mut seed: Seed = HashMap::new();

        for id in 0..props.size {
            let from = nnh.random_from_neuron();
            let to = nnh.random_to_neuron();
            let weight = Genome::random_weight();

            let gene = Gene {
                id,
                from,
                to,
                weight,
            };

            if let GeneType::InputGene = Genome::classify_gene(nnh, &gene) {
                inputs.push(from);
            }

            seed.entry(from).or_insert(vec![]).push(gene);
        }

        // TODO I don't think this is necessary any more, we can make this an arbitrary number.
        // No matter what, all the genes that can be followed will get followed at least once,
        // since we're doing it in a more "follow the path" kind of stle.
        let max_gene_follows = nnh.inner_neurons.len() + 2;

        let ordered_genes =
            Genome::generate_ordered_from_seed_and_inputs(&seed, &inputs, max_gene_follows);

        Self { seed, ordered_genes, max_gene_follows }
    }

    /// Takes a gene and inserts it into the genome. Recomputes the ordered_genes value.
    /// TODO Might eventually turn this into register_genes() and have it take a vec of genes
    pub fn register_gene(&mut self, gene: Gene) {
        let mut inputs: Vec<usize> = vec![];
        let mut seed: Seed = HashMap::new();

        let mut genes = Genome::genes_from_seed(&self.seed);
        genes.push(gene);

        for gene in genes {
            Genome::insert_gene_to_seed_and_inputs(gene, &mut seed, &mut inputs);
        }

        self.ordered_genes = Genome::generate_ordered_from_seed_and_inputs(&seed, &inputs, self.max_gene_follows);
        self.seed = seed;
    }

    fn insert_gene_to_seed_and_inputs(gene: Gene, seed: &mut Seed, inputs: &mut Vec<usize>) {
        let from = gene.from.clone();
        seed.entry(gene.from).or_insert(vec![]).push(gene);
        inputs.push(from);

    }

    fn genes_from_seed(seed: &Seed) -> Vec<Gene> {
        let mut genes = vec![];

        for sub_genes in seed.values() {
            for gene in sub_genes {
                genes.push(gene.clone());
            }
        }

        genes
    }

    pub fn random_weight() -> f32 {
        thread_rng().gen_range(-4.0..=4.0)
    }

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
