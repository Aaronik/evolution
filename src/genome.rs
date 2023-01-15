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

// /// All the genes coming only from input neurons
// type InputGene = Gene;
// /// All the genes that go from inner neurons to inner neurons
// type InnerGene = Gene;
// /// All the genes that go from inner neurons to output neurons
// type OutputGene = Gene;

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

#[derive(Debug, Clone)]
pub struct Genome {
    // /// (this is here to facilitate the neural net calculation at each tic)
    // pub input_genes: Vec<InputGene>,
    // /// (this is here to facilitate the neural net calculation at each tic)
    // pub inner_genes: Vec<InnerGene>,
    // /// (this is here to facilitate the neural net calculation at each tic)
    // pub output_genes: Vec<OutputGene>,

    /// The seed for a gene tree. This can be walked like a recursive tree.
    pub seed: HashMap<usize, Vec<Gene>>,

    /// List of neuron ids that are activated by this genome
    pub inputs: Vec<usize>,
    pub num_inner_genes: usize,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let nnh = props.neural_net_helper;

        // let mut input_genes = vec![];
        // let mut inner_genes = vec![];
        // let mut output_genes = vec![];

        let mut inputs: Vec<usize> = vec![];
        let mut num_inner_genes: usize = 0;

        // TODO Make function self.register_gene();
        // Will have to go through the whole process of recalculating
        // the ordered gene vec
        let mut seed: HashMap<usize, Vec<Gene>> = HashMap::new();

        for id in 0..props.size {
            let from = nnh.random_from_neuron();
            let to = nnh.random_to_neuron();
            let weight = Genome::random_weight();

            let gene = Gene { id, from, to, weight };

            seed.entry(from).or_insert(vec![]);

            match Genome::classify_gene(nnh, &gene) {
                GeneType::InputGene => inputs.push(from),
                GeneType::InnerGene => num_inner_genes += 1,
                GeneType::OutputGene => ()
            }


            let genes_vec = seed.get_mut(&from).unwrap();
            genes_vec.push(gene);

        }

        Self {
            seed,
            inputs,
            num_inner_genes,
            // input_genes,
            // inner_genes,
            // output_genes,
        }
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
}
