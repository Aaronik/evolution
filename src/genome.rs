use crate::*;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

pub struct GenomeProps<'a> {
    pub size: usize,
    pub neural_net_helper: &'a NeuralNetHelper,
}

#[derive(Debug)]
pub struct Genome {
    /// All the genes coming only from input neurons
    /// (this is here to facilitate the neural net calculation at each tic)
    pub input_genes: Vec<Gene>,
    /// All the genes that go from inner neurons to inner neurons
    /// (this is here to facilitate the neural net calculation at each tic)
    pub inner_genes: Vec<Gene>,
    /// All the genes that go from inner neurons to output neurons
    /// (this is here to facilitate the neural net calculation at each tic)
    pub output_genes: Vec<Gene>,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let mut output_neuron_ids = props.neural_net_helper.output_neuron_ids.clone();
        let mut inner_output_neuron_ids = props.neural_net_helper.inner_output_neuron_ids.clone();
        let input_output_clone = props.neural_net_helper.input_output.clone();
        let mut input_output_iter = input_output_clone.iter();
        let input_inner_output_clone = props.neural_net_helper.input_inner_output.clone();
        let mut input_inner_output_iter = input_inner_output_clone.iter();

        let NeuralNetHelper {
            neuron_type_map, ..
        } = props.neural_net_helper;

        let mut input_genes = vec![];
        let mut inner_genes = vec![];
        let mut output_genes = vec![];

        let mut starting_id: Option<usize> = None;

        fn num_genes_left(size: &usize, input_genes: &Vec<Gene>, inner_genes: &Vec<Gene>, output_genes: &Vec<Gene>) -> usize {
            size - (input_genes.len() + inner_genes.len() + output_genes.len())
        }

        // Get one id from output neurons
        let mut random_output_neuron_id = || {
            let idx = thread_rng().gen_range(0..output_neuron_ids.len());
            output_neuron_ids.remove(idx)
        };

        // Get one id from inner neurons UNION output neurons
        let mut random_inner_output_neuron_id = || {
            let idx = thread_rng().gen_range(0..inner_output_neuron_ids.len());
            inner_output_neuron_ids.remove(idx)
        };

        let random_weight = || thread_rng().gen_range(-4.0..=4.0);

        // TODO Need to ensure we don't have more inner genes than is allowed

        // This is a cleverly (hopefully) crafted loop that allows us to construct a genome that:
        // * Doesn't repeat itself (does this by exracting from a set of possible id pairs)
        // * Doesn't result in useless configurations (this is what most of the logic is about)
        loop {
            if let Some(from) = starting_id {
                if num_genes_left(&props.size, &input_genes, &inner_genes, &output_genes) == 1 {
                    let to = random_output_neuron_id();
                    let weight = random_weight();
                    output_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    let to = random_inner_output_neuron_id();
                    let to_type = &neuron_type_map[&to];
                    let weight = random_weight();

                    if let NeuronType::InnerNeuron = to_type {
                        inner_genes.push(Gene { from, to, weight });
                        starting_id = Some(to);
                        continue;
                    } else if let NeuronType::OutputNeuron = to_type {
                        output_genes.push(Gene { from, to, weight });
                        starting_id = None;
                        continue;
                    }
                }
            } else {
                if num_genes_left(&props.size, &input_genes, &inner_genes, &output_genes) == 1 {
                    let (from, to) = *input_output_iter.nth(0).unwrap();
                    let weight = random_weight();
                    input_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    let (from, to) = *input_inner_output_iter.nth(0).unwrap();
                    let to_type = &neuron_type_map[&to];
                    let weight = random_weight();

                    if let NeuronType::InnerNeuron = to_type {
                        input_genes.push(Gene { from, to, weight });
                        starting_id = Some(to);
                        continue;
                    } else if let NeuronType::OutputNeuron = to_type {
                        input_genes.push(Gene { from, to, weight });
                        starting_id = None;
                        continue;
                    }
                }
            }
        }

        Self {
            input_genes,
            inner_genes,
            output_genes,
        }
    }
}
