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
    pub fn mutate(genome: &Genome, nnh: &NeuralNetHelper) -> Genome {
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

        // Which of the three fields are we going to modify?
        let from_to_weight = thread_rng().gen_range(0..3);

        if from_to_weight == 0 {
            gene.weight = Genome::random_weight();
        } else if from_to_weight == 1 {
            gene.from = nnh.random_from_neuron();
        } else {
            gene.to = nnh.random_to_neuron();
        }

        match Genome::classify_gene(nnh, &gene) {
            GeneType::InputGene => genome.input_genes.push(gene),
            GeneType::InnerGene => genome.inner_genes.push(gene),
            GeneType::OutputGene => genome.output_genes.push(gene),
        }

        genome
    }
}
