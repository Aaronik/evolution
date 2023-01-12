use std::collections::HashSet;

use crate::*;
use rand::{thread_rng, Rng};

pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

pub struct GenomeProps<'a> {
    size: usize,
    neural_net_helper: &'a NeuralNetHelper,
}

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
        let output_neuron_ids = props.neural_net_helper.output_neuron_ids.clone();
        let inner_output_neuron_ids = props.neural_net_helper.inner_output_neuron_ids.clone();
        let input_output = props.neural_net_helper.input_output.clone();
        let input_inner_output = props.neural_net_helper.input_inner_output.clone();

        let NeuralNetHelper {
            neuron_type_map, ..
        } = props.neural_net_helper;

        let input_genes = vec![];
        let inner_genes = vec![];
        let output_genes = vec![];

        // from id, to id
        let duplicate_prevention_set: HashSet<(usize, usize)> = HashSet::new();

        let mut starting_id: Option<usize> = None;

        let num_genes_left =
            || props.size - (&input_genes.len() + &inner_genes.len() + &output_genes.len());

        let random_output_neuron_id = || {
            let idx = thread_rng().gen_range(0..output_neuron_ids.len());
            output_neuron_ids[idx]
        };

        let random_inner_output_neuron_id = || {
            let idx = thread_rng().gen_range(0..inner_output_neuron_ids.len());
            inner_output_neuron_ids[idx]
        };

        let random_weight = || thread_rng().gen_range(-4.0..=4.0);

        // TODO Right now we're validating the genes heavily within the lifeform.
        // Instead, I'd much rather proactively create a valid structure here.
        //
        // To do that:
        // * If we have a starting id (it's an inner neuron):
        //  * If there's only one gene left:
        //   * Pick randomly from output neurons and make that TO.
        //   * Add that to output_genes;
        //   * break;
        //  * If there're more genes left to make:
        //   * Pick randomly from inner neurons AND output neurons
        //   * If it's an inner neuron
        //    * Add gene to inner_genes
        //    * assign neuron's id to starting id
        //    * continue;
        //   * If it's an output neuron
        //    * Add gene to output_genes
        //    * continue;
        // * If we don't have a starting id:
        //  * If there's only one gene left:
        //   * Pick random pair from input_output
        //   * Add that to output_genes;
        //   * break;
        //  * If there're more genes to make:
        //   * Pick random pair from input_inner_output.
        //   * If TO is an inner neuron:
        //    * Add it to inner_genes
        //    * Set starting id to TO (inner neuron)
        //    * continue;
        //   * If TO is an output neuron:
        //    * Add it to output_genes
        //    * continue;

        loop {
            if let Some(from) = starting_id {
                if num_genes_left() == 1 {
                    let to = random_output_neuron_id();
                    let weight = random_weight();
                    output_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    let to = random_inner_output_neuron_id();
                    let to_type = neuron_type_map[&to];
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
                if num_genes_left() == 1 {
                    let (from, to) = *input_output.iter().nth(0).unwrap();
                    let weight = random_weight();
                    output_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    let (from, to) = *input_inner_output.iter().nth(0).unwrap();
                    let to_type = neuron_type_map[&to];
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
            }
        }

        Self {
            input_genes,
            inner_genes,
            output_genes,
        }
    }
}
