use rand::{thread_rng, Rng};

use crate::*;

#[derive(Debug)]
pub struct Evolver {
}

impl Evolver {
    pub fn mate(_lf1: LifeForm, _lf2: LifeForm) -> LifeForm {
        todo!()
    }

    pub fn fitness(lf: &LifeForm) -> usize {
        lf.lifespan
    }

    pub fn should_mutate(mutation_rate: f32) -> bool {
        thread_rng().gen_bool(mutation_rate as f64)
    }

    /// Takes a genome, makes a clone of it with a slight mutation, returns that
    pub fn mutate(genome: &Genome, neural_net_helper: &NeuralNetHelper) -> Genome {
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
        from_neuron_ids.append(&mut neural_net_helper.input_neuron_ids.clone());
        from_neuron_ids.append(&mut neural_net_helper.inner_neuron_ids.clone());

        let mut to_neuron_ids: Vec<usize> = vec![];
        to_neuron_ids.append(&mut neural_net_helper.inner_neuron_ids.clone());
        to_neuron_ids.append(&mut neural_net_helper.output_neuron_ids.clone());

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
            neural_net_helper.neuron_type_map.get(id).unwrap()
        };

        // TODO for example, evolver here knows about this ordering below.
        // This is something that genome knows about, not evolver.

        // Insert modified gene into the correct bucket
        if let NeuronType::InputNeuron = type_of(&gene.from) {
            genome.input_genes.push(gene);
        } else if let NeuronType::InnerNeuron = type_of(&gene.from) {
            if let NeuronType::InnerNeuron = type_of(&gene.to) {
                genome.inner_genes.push(gene);
            }
        } else {
            genome.output_genes.push(gene);
        }

        genome
    }
}
