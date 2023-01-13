use rand::{thread_rng, Rng};

use crate::*;

pub struct Evolver {
    pub world: World,
}

impl Evolver {
    pub fn mate(_lf1: LifeForm, _lf2: LifeForm) -> LifeForm {
        todo!()
    }

    // TODO I think this jank might want to be in Genome
    // Reassign a single FROM or TO or WEIGHT in one gene
    pub fn mutate(lf: &LifeForm) -> Genome {
        let mut genome = lf.genome.clone();

        // First we just get one gene at random from the bunch
        let num_genes =
            genome.input_genes.len() + genome.inner_genes.len() + genome.output_genes.len();

        let idx = thread_rng().gen_range(0..num_genes);

        let mut gene: Gene;

        // TODO we'll have to remember from which pool we took so we can put back...
        // Wait, we're going to reassign the pool anyways. So that means we can do
        // the chain (append) method and get the gene from there... THIS IS TOO COMPLICATED
        if idx < genome.input_genes.len() {
            gene = genome.input_genes.remove(idx);
        } else if idx < genome.input_genes.len() + genome.inner_genes.len() {
            gene = genome.inner_genes.remove(idx - genome.input_genes.len());
        } else {
            gene = genome.output_genes.remove(idx - (genome.input_genes.len() + genome.inner_genes.len()));
        }

        let from_to_weight = thread_rng().gen_range(0..3);
        if from_to_weight == 0 {
            gene.weight = Genome::random_weight();
        } else if from_to_weight == 1 {
            // TODO Ok, so I want to pick from the available pool of froms.
            // * If the current to is an output neuron, we can pick any input neuron or any
            // inner neuron as a from.
            // * If the current to is an inner neuron, we can pick any input neuron or any inner
            // neuron as a from, same as above.
            // gene.from =
        } else {
            // * Anywhere but
            // gene.to =
        }

        // TODO OBLITERATE THIS
        genome.clone()
    }

    pub fn asexually_reproduce(lf: &LifeForm, available_id: usize) -> LifeForm {
        // TODO mutate a little
        let genome = lf.genome.clone();
        let neural_net = lf.neural_net.clone();

        LifeForm {
            id: available_id,
            genome,
            neural_net,
            health: 1.0,
            hunger: 0.0,
            thirst: 0.0,
            location: (lf.location.0, lf.location.1 + 1),
        }
    }
}

// fn mutate(genes: Vec<Gene>, possible_neuron_ids: Vec<usize>) -> Vec<Gene> {
//     let idx = thread_rng().gen_range(0..genes.len());
//     let gene = genes.get(idx).unwrap();

// if thread_rng().gen_bool() {
//     // gene.
// }
// }
