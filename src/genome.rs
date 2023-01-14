use crate::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

/// All the genes coming only from input neurons
type InputGene = Gene;
/// All the genes that go from inner neurons to inner neurons
type InnerGene = Gene;
/// All the genes that go from inner neurons to output neurons
type OutputGene = Gene;

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
    /// (this is here to facilitate the neural net calculation at each tic)
    pub input_genes: Vec<InputGene>,
    /// (this is here to facilitate the neural net calculation at each tic)
    pub inner_genes: Vec<InnerGene>,
    /// (this is here to facilitate the neural net calculation at each tic)
    pub output_genes: Vec<OutputGene>,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let nnh = props.neural_net_helper;

        let mut input_genes = vec![];
        let mut inner_genes = vec![];
        let mut output_genes = vec![];

        for _ in 0..props.size {
            let from = nnh.random_from_neuron();
            let to = nnh.random_to_neuron();
            let weight = Genome::random_weight();

            let gene = Gene { from, to, weight };

            match Genome::classify_gene(nnh, &gene) {
                GeneType::InputGene => input_genes.push(gene),
                GeneType::InnerGene => inner_genes.push(gene),
                GeneType::OutputGene => output_genes.push(gene),
            }
        }

        Self {
            input_genes,
            inner_genes,
            output_genes,
        }
    }

    pub fn random_weight() -> f32 {
        thread_rng().gen_range(-4.0..=4.0)
    }

    pub fn classify_gene(nnh: &NeuralNetHelper, gene: &Gene) -> GeneType {
        let type_of = |id: &usize| nnh.neuron_type_map.get(id).unwrap();

        if let NeuronType::InputNeuron = type_of(&gene.from) {
            return GeneType::InputGene;
        } else if let NeuronType::InnerNeuron = type_of(&gene.from) {
            if let NeuronType::InnerNeuron = type_of(&gene.to) {
                return GeneType::InnerGene;
            }
        }

        return GeneType::OutputGene;
    }
}
