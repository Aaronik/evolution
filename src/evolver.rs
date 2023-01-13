use rand::{thread_rng, Rng};

use crate::*;

pub struct Evolver {
    neural_net_helper: NeuralNetHelper,
}

impl Evolver {
    pub fn new(neural_net_helper: NeuralNetHelper) -> Self {
        Self { neural_net_helper }
    }

    pub fn mate(_lf1: LifeForm, _lf2: LifeForm) -> LifeForm {
        todo!()
    }

    // Takes a genome, makes a clone of it with a slight mutation, returns that
    pub fn mutate(&self, genome: &Genome) -> Genome {
        let mut genome = genome.clone();

        // First we just get one gene at random from the bunch
        let num_genes =
            genome.input_genes.len() + genome.inner_genes.len() + genome.output_genes.len();

        let idx = thread_rng().gen_range(0..num_genes);

        // Remove a gene from one of the pools here
        let mut gene = if idx < genome.input_genes.len() {
            genome.input_genes.remove(idx)
        } else if idx < genome.input_genes.len() + genome.inner_genes.len() {
            genome.inner_genes.remove(idx - genome.input_genes.len())
        } else {
            genome
                .output_genes
                .remove(idx - (genome.input_genes.len() + genome.inner_genes.len()))
        };

        let mut from_neuron_ids: Vec<usize> = vec![];
        from_neuron_ids.append(&mut self.neural_net_helper.input_neuron_ids.clone());
        from_neuron_ids.append(&mut self.neural_net_helper.inner_neuron_ids.clone());

        let mut to_neuron_ids: Vec<usize> = vec![];
        to_neuron_ids.append(&mut self.neural_net_helper.inner_neuron_ids.clone());
        to_neuron_ids.append(&mut self.neural_net_helper.output_neuron_ids.clone());

        // Which of the three fields are we going to modify?
        let from_to_weight = thread_rng().gen_range(0..3);
        if from_to_weight == 0 {
            // modify weight
            gene.weight = Genome::random_weight();
        } else if from_to_weight == 1 {
            // modify FROM gene. We'll just pick another at random. If it's a duplicate,
            // so be it, it'll just have the effect of modifying the weight and reducing
            // the complexity of the little guy's neural net.
            let idx = thread_rng().gen_range(0..from_neuron_ids.len());
            gene.from = from_neuron_ids[idx];
        } else {
            // modify TO gene.
            let idx = thread_rng().gen_range(0..to_neuron_ids.len());
            gene.to = to_neuron_ids[idx];
        }

        let type_of = |id: &usize| {
            self.neural_net_helper.neuron_type_map.get(id).unwrap()
        };

        // Insert modified gene into the correct bucket
        if let NeuronType::InputNeuron = type_of(&gene.from) {
            genome.input_genes.push(gene);
        } else if let NeuronType::InnerNeuron = type_of(&gene.from) {
            if let NeuronType::InnerNeuron = type_of(&gene.to) {
                genome.input_genes.push(gene);
            }
        } else {
            genome.output_genes.push(gene);
        }

        genome
    }

    // TODO Just operate on genomes ok?
    pub fn asexually_reproduce(&self, lf: &LifeForm, available_id: usize) -> LifeForm {
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
