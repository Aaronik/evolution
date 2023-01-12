use std::collections::HashSet;

use crate::*;
use rand::{thread_rng, Rng};

pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

// Issues:
// * You can't have two of the same from/to pair

// TODO
// * Maybe instead of Gene, this is a Genome that spawns Genes. Then easy track
// could be kept of which from/to pairs are used.
impl Gene {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        Self { from, to, weight }
    }

    pub fn new_random(
        input_ids: Vec<usize>,
        inner_ids: Vec<usize>,
        output_ids: Vec<usize>,
    ) -> Self {
        // from: randomly from input_ids and inner_ids
        // to: randomly from inner_ids and output_ids
        // weight: randomly from -4 to 4

        let from_idx = thread_rng().gen_range(0..input_ids.len() + inner_ids.len());
        let to_idx = thread_rng().gen_range(0..inner_ids.len() + output_ids.len());

        let from: usize;
        if from_idx < input_ids.len() {
            from = input_ids.get(from_idx).unwrap().clone();
        } else {
            from = inner_ids.get(from_idx - input_ids.len()).unwrap().clone();
        }

        let to: usize;
        if to_idx < inner_ids.len() {
            to = inner_ids.get(to_idx).unwrap().clone();
        } else {
            to = output_ids.get(to_idx - inner_ids.len()).unwrap().clone();
        }

        let weight = thread_rng().gen_range(-4.0..4.0);

        Self { from, to, weight }
    }
}

pub struct GenomeProps<'a> {
    size: usize,
    neural_net: &'a NeuralNet,
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
        let input_neuron_ids: Vec<usize> =
            props.neural_net.input_neurons.keys().map(|k| *k).collect();
        let inner_neuron_ids: Vec<usize> =
            props.neural_net.inner_neurons.keys().map(|k| *k).collect();
        let output_neuron_ids: Vec<usize> =
            props.neural_net.output_neurons.keys().map(|k| *k).collect();

        // These will allow us to ensure we're picking unique pairs
        // Can turn each into iter(), then can call nth(0) on the iterator, which is like
        // shifting a value out of the front. It's in an arbitrary order, so this is akin
        // to taking out one random value from the hashset.
        // TODO These have to be made one time and when the neural net is first made and cloned or something
        // AND The neural net should be made one time when the program starts and THAT should be
        // cloned as well. It can live on NeuralNet::net and NeuralNet::hash_sets or something.

        // (input_neuron_id, output_neuron_id)
        let input_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in input_neuron_ids {
            for output_neuron_id in output_neuron_ids {
                input_output.insert((input_neuron_id, output_neuron_id));
            }
        }

        // (inner_neuron_id, output_neuron_id)
        let inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in inner_neuron_ids {
            for output_neuron_id in output_neuron_ids {
                inner_output.insert((inner_neuron_id, output_neuron_id));
            }
        }

        let inner_output_neuron_ids = inner_neuron_ids.clone();
        inner_output_neuron_ids.append(&mut output_neuron_ids.clone());

        // (input_neuron_id, inner_neuron_id AND output_neuron_id)
        let input_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for input_neuron_id in input_neuron_ids {
            for inner_output_neuron_id in inner_output_neuron_ids {
                input_inner_output.insert((input_neuron_id, inner_output_neuron_id));
            }
        }

        // (inner_neuron_id, inner_neuron_id AND output_neuron_id)
        let inner_inner_output: HashSet<(usize, usize)> = HashSet::new();

        for inner_neuron_id in inner_neuron_ids {
            for inner_output_neuron_id in inner_output_neuron_ids {
                inner_inner_output.insert((inner_neuron_id, inner_output_neuron_id));
            }
        }

        // --- End of hashset creation --- //

        let input_genes = vec![];
        let inner_genes = vec![];
        let output_genes = vec![];

        // from id, to id
        let duplicate_prevention_set: HashSet<(usize, usize)> = HashSet::new();

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

        // To prevent duplicate genes.
        // TODO This isn't the most efficient practice. If the number of
        // genes starts to approach the number of neurons we have, we're going to start having
        // a lot of collisions and this could take who knows long.
        while duplicate_prevention_set.len() < props.size {
            let gene = Gene::new_random(input_neuron_ids, inner_neuron_ids, output_neuron_ids);

            let is_unique = duplicate_prevention_set.insert((gene.from, gene.to));

            if !is_unique {
                continue;
            }

            // From here on out we have a unique gene
            match props.neural_net.neuron_type_map[&gene.from] {
                NeuronType::InputNeuron => input_genes.push(gene),
                NeuronType::InnerNeuron => inner_genes.push(gene),
                NeuronType::OutputNeuron => output_genes.push(gene),
            }
        }

        Self {
            input_genes,
            inner_genes,
            output_genes,
        }
    }
}
