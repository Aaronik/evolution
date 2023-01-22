use rand::{thread_rng, Rng};

use crate::*;

#[derive(Debug)]
pub struct Evolver {}

impl Evolver {
    pub fn mate(genome1: &Genome, genome2: &Genome, nnh: &NeuralNetHelper) -> Genome {
        let mut genes = vec![];

        // TODO parallelize
        for i in 0..genome1.genes.len() / 2 {
            genes.push(genome1.genes[i].clone());
        }

        for i in (genome2.genes.len() / 2)..(genome2.genes.len()) {
            genes.push(genome2.genes[i].clone());
        }

        let mut genome = Genome {
            genes,
            ordered_gene_indices: vec![], // will be computed after creation
        };

        genome.recompute_ordered_gene_indices(nnh);

        genome
    }

    pub fn fitness(lf: &LifeForm) -> usize {
        lf.lifespan
    }

    pub fn should_mutate(mutation_rate: f32) -> bool {
        thread_rng().gen_bool(mutation_rate as f64)
    }

    /// Takes a mut ref to a genome and makes a slight mutation on the genome
    pub fn mutate(genome: &mut Genome, nnh: &NeuralNetHelper) {
        // First we just get one gene at random from the bunch
        let idx = thread_rng().gen_range(0..genome.genes.len());

        // Which of the three fields are we going to modify?
        let from_to_weight = thread_rng().gen_range(0..3);

        if from_to_weight == 0 {
            // TODO Maybe we should nudge this weight rather than raplace it?
            genome.genes[idx].weight = Genome::random_weight();
        } else if from_to_weight == 1 {
            genome.genes[idx].from = nnh.random_from_neuron(Some(genome.genes[idx].from));
        } else {
            genome.genes[idx].to = nnh.random_to_neuron(Some(genome.genes[idx].to));
        }

        genome.recompute_ordered_gene_indices(nnh);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_mates_genomes() {
        let nnh = NeuralNetHelper::new(0);

        let g1 = Genome::new(GenomeProps {
            neural_net_helper: &nnh,
            size: 10,
        });

        let g2 = Genome::new(GenomeProps {
            neural_net_helper: &nnh,
            size: 10,
        });

        let g = Evolver::mate(&g1, &g2, &nnh);

        assert_eq!(g.genes.len(), g1.genes.len());
        assert!(g.ordered_gene_indices.len() > 0);

        let mut has_some_different = false;

        for (idx, a) in g.genes.iter().enumerate() {
            let b = &g1.genes[idx];
            if a.weight != b.weight || a.from != b.from || a.to != b.to {
                has_some_different = true;
            }
        }

        assert!(has_some_different);
    }

    #[test]
    fn it_mutates_a_genome() {
        let nnh = NeuralNetHelper::new(0);

        let mut genome = Genome::new(GenomeProps {
            neural_net_helper: &nnh,
            size: 5,
        });

        let before = genome.clone();

        Evolver::mutate(&mut genome, &nnh);

        assert_eq!(before.genes.len(), genome.genes.len());

        let mut has_diff_gene = false;

        // There should be some kind of difference in the genome
        for i in 0..genome.genes.len() {
            let b = &before.genes[i];
            let a = &genome.genes[i];
            if a.from != b.from || a.to != b.to || a.weight != b.weight {
                has_diff_gene = true;
                break;
            }
        }

        assert!(has_diff_gene);
    }
}
