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
    pub fn mutate(genome: &mut Genome, nnh: &NeuralNetHelper) {
        // First we just get one gene at random from the bunch
        let idx = thread_rng().gen_range(0..genome.genes.len());
        let mut gene = genome.genes.remove(idx);

        // Which of the three fields are we going to modify?
        let from_to_weight = thread_rng().gen_range(0..3);

        if from_to_weight == 0 {
            gene.weight = Genome::random_weight();
        } else if from_to_weight == 1 {
            gene.from = nnh.random_from_neuron();
        } else {
            gene.to = nnh.random_to_neuron();
        }

        genome.recompute_ordered_genes(nnh);
    }
}

#[test]
fn it_mutates_a_genome() {
    let nnh = NeuralNetHelper::new(0);

    let mut genome = Genome::new(GenomeProps {
        neural_net_helper: &nnh,
        size: 1,
    });

    let before = genome.clone();

    Evolver::mutate(&mut genome, &nnh);

    let mut has_diff_gene = false;

    // If they're not the same size, it was definitely updated.
    if before.ordered_genes.len() != genome.ordered_genes.len() {
        assert!(true);
        return;
    }

    // If it does happen to be the same size, there should be ones that are different.
    for i in 0..genome.ordered_genes.len() {
        let b = &before.ordered_genes[i];
        let a = &genome.ordered_genes[i];
        if a.from != b.from || a.to != b.to || a.weight != b.weight {
            has_diff_gene = true;
        }
    }

    assert!(has_diff_gene);
}
