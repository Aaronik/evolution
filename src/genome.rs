use std::collections::HashMap;

use crate::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

#[derive(Debug, Clone)]
pub struct GenomeProps<'a> {
    pub size: usize,
    pub neural_net_helper: &'a NeuralNetHelper,
}

#[derive(Debug, Clone)]
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
        let inner_output_neuron_ids = props.neural_net_helper.inner_output_neuron_ids.clone();
        let input_output_clone = props.neural_net_helper.input_output.clone();
        let mut input_output_iter = input_output_clone.iter();
        let input_inner_output_clone = props.neural_net_helper.input_inner_output.clone();
        let mut input_inner_output_iter = input_inner_output_clone.iter();

        // Ok, I'm a bit delerious but this thing here will help us not repeat AND not
        // run out of inner or ouput neuron ids to sample from. Hopefully the name is somewhat
        // descriptive!
        // map from inner neuron id -> Vec<inner neuron ids UNION output neuron ids>
        let mut inner_neuron_id_potential_to_pool: HashMap<usize, Vec<usize>> = HashMap::new();
        for inner_id in &props.neural_net_helper.inner_neuron_ids {
            for inner_output_id in &inner_output_neuron_ids {
                if inner_neuron_id_potential_to_pool.contains_key(&inner_id) {
                    inner_neuron_id_potential_to_pool
                        .get_mut(&inner_id)
                        .unwrap()
                        .push(*inner_output_id);
                } else {
                    inner_neuron_id_potential_to_pool.insert(*inner_id, vec![*inner_output_id]);
                }
            }
        }

        let NeuralNetHelper {
            neuron_type_map, ..
        } = props.neural_net_helper;

        let mut input_genes = vec![];
        let mut inner_genes = vec![];
        let mut output_genes = vec![];

        let mut starting_id: Option<usize> = None;

        fn num_genes_left(
            size: &usize,
            input_genes: &Vec<Gene>,
            inner_genes: &Vec<Gene>,
            output_genes: &Vec<Gene>,
        ) -> usize {
            size - (input_genes.len() + inner_genes.len() + output_genes.len())
        }

        // Get one id from output neurons
        let mut random_output_neuron_id = || {
            let idx = thread_rng().gen_range(0..output_neuron_ids.len());
            output_neuron_ids.remove(idx)
        };

        // Get one id from inner neurons UNION output neurons
        let mut random_inner_output_neuron_id = |from: &usize| {
            let potentials_pool_length = inner_neuron_id_potential_to_pool[from].len();

            // It can happen that there'll be no more inner neuron ids in this pool,
            // if the inner neuron in question is already connected to
            // itself and every single output neuron.
            // If this is the case, we need to simply pass on this connection.
            if potentials_pool_length == 0 {
                return None;
            }

            let idx = thread_rng().gen_range(0..potentials_pool_length);
            let to = inner_neuron_id_potential_to_pool
                .get_mut(from)
                .unwrap()
                .remove(idx);

            Some(to)
        };

        // This is a cleverly (hopefully) crafted loop that allows us to construct a genome that:
        // * Doesn't repeat itself (does this by exracting from a set of possible id pairs)
        // * Doesn't result in useless configurations (this is what most of the logic is about)
        // TODO I'm starting to wonder if we don't actually GAF about this rigamerole and simply
        // do random (although non repeating) assignment, gosh be darned if it's a dud. It'll
        // evolve away. B/c when we mutate, it's hard to be really random if we can't assign to any
        // old id. When we mate, we're just going to pick at random genomes. So some may overlap
        // and some may be dead ends. Or, maybe it's good to start them from a good position and
        // then let them evolve in whatever way they want. In fact you know what, even if we repeat
        // a gene, who cares? That'll have the effect of modifying the weight of the connection and
        // reducing the complexity of the brain.
        loop {
            if let Some(from) = starting_id {
                if num_genes_left(&props.size, &input_genes, &inner_genes, &output_genes) == 1 {
                    let to = random_output_neuron_id();
                    let weight = Genome::random_weight();
                    output_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    // TODO If we get to our final inner neuron, this will try to pool
                    // from an empty list and throw
                    let to = random_inner_output_neuron_id(&from);

                    // If this happens, we're already maxed out on connections for this inner
                    // neuron, so we abandon it and move on. This will result in one less gene for
                    // this lifeform.
                    if let None = to {
                        starting_id = None;
                        continue;
                    }

                    let to = to.unwrap();

                    let to_type = &neuron_type_map[&to];
                    let weight = Genome::random_weight();

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
                    let weight = Genome::random_weight();
                    input_genes.push(Gene { from, to, weight });
                    break;
                } else {
                    let (from, to) = *input_inner_output_iter.nth(0).unwrap();
                    let to_type = &neuron_type_map[&to];
                    let weight = Genome::random_weight();

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

    pub fn random_weight() -> f32 {
        thread_rng().gen_range(-4.0..=4.0)
    }
}
