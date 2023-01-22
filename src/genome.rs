use std::collections::HashMap;

use crate::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Gene {
    pub id: usize,
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

pub enum GeneType {
    InputGene,
    InnerGene,
    OutputGene,
}

#[derive(Debug, Clone)]
pub struct GenomeProps<'a> {
    pub size: usize,
    pub neural_net_helper: &'a NeuralNetHelper,
}

// Hashmap of neuron id to vec of neuron ids
type Seed = HashMap<usize, Vec<usize>>;

#[derive(Debug, Clone)]
pub struct Genome {
    /// An unordered unique list of genes representing one of each gene the genome has. Use this
    /// list to modify the genes in a genome, then call genome.recompute_ordered_genes().
    pub genes: Vec<Gene>,

    /// An ordered list of ids for genes in the self.genes vec constructed to enable fast and easy
    /// neural net calculations. Overwritten by calling genome.recompute_ordered_genes, which needs
    /// to be done after every time self.genes is modified.
    /// Note that I tried to make this a list of references to the genes in the self.genes vec, but
    /// it turns out rust doesn't do so called "self referential structs" natively, and there are
    /// all kinds of language hacks and cumbersome external crates to make that possible. I figure
    /// best to only use those if absolutely necessary, which, in this case, it's not.
    pub ordered_gene_indices: Vec<usize>,
}

impl Genome {
    pub fn new(props: GenomeProps) -> Self {
        let mut genes: Vec<Gene> = vec![];

        for id in 0..props.size {
            genes.push(Gene {
                id,
                from: props.neural_net_helper.random_from_neuron(None),
                to: props.neural_net_helper.random_to_neuron(None),
                weight: Genome::random_weight(),
            });
        }

        let ordered_gene_indices = compute_ordered_gene_indices(&genes, props.neural_net_helper);

        Self { genes, ordered_gene_indices }
    }

    /// After the genome.genes vector has been messed with, call this to rebuild the data
    /// structures necessary to efficiently calculate this genome's neural net probabilities.
    pub fn recompute_ordered_gene_indices(&mut self, nnh: &NeuralNetHelper) {
        self.ordered_gene_indices = compute_ordered_gene_indices(&self.genes, nnh);
    }

    /// An InputGene is one that comes from an input. These are where you start when you do a
    /// recursive calculation to find out the final output neuron probabilities. An InnerGene is
    /// one that goes from an inner neuron to another inner neuron. An OutputGene is from an
    /// InnerNeuron to an OutputNeuron.
    pub fn classify_gene(nnh: &NeuralNetHelper, gene: &Gene) -> GeneType {
        if let NeuronType::InputNeuron = nnh.neuron_type(&gene.from) {
            return GeneType::InputGene;
        } else if let NeuronType::InnerNeuron = nnh.neuron_type(&gene.from) {
            if let NeuronType::InnerNeuron = nnh.neuron_type(&gene.to) {
                return GeneType::InnerGene;
            }
        }

        return GeneType::OutputGene;
    }

    pub fn random_weight() -> f32 {
        thread_rng().gen_range(-4.0..=4.0)
    }

}

/// Takes a vector of unique genes, returns a vector of indices of those genes in the correct order
/// to walk them to do the neural net calculation. Going from one end of the returned vector to
/// the other adding the sums of the neurons for the genes already seen and taking the tanh of
/// them, etc, will be like starting at the input neurons and following their connections, then
/// following the connection of the next neuron, etc, recursively, for a specified maximum
/// number sf times per gene, in case there is a loop. This is the best way I could think of to
/// approximate biological neural nets.
fn compute_ordered_gene_indices(genes: &Vec<Gene>, nnh: &NeuralNetHelper) -> Vec<usize> {
    // TODO I don't think this is necessary any more, we can make this an arbitrary number.
    // No matter what, all the genes that can be followed will get followed at least once.
    let max_gene_follows = nnh.inner_neurons.len() + 2;

    let mut seed: Seed = HashMap::new();
    let mut inputs: Vec<usize> = vec![];
    for (idx, gene) in genes.iter().enumerate() {
        if let GeneType::InputGene = Genome::classify_gene(nnh, &gene) {
            inputs.push(gene.from);
        }

        seed.entry(gene.from).or_insert(vec![]).push(idx);
    }

    // gene id, number of times seen
    let mut follow_count: HashMap<usize, usize> = HashMap::new();

    let mut ordered_gene_indices: Vec<usize> = vec![];

    // * Append all genes from each input_id to ordered_genes, incrementing follow_count
    // for each TO.
    for input_id in inputs {
        for gene_idx in &seed[&input_id] {
            ordered_gene_indices.push(*gene_idx);
        }
    }

    // * Then go through each member of ordered_genes, append all genes for each FROM
    // to ordered genes, again incrementing follow_count for each.
    // * If at any time follow_count is above the limit, don't add that gene again.
    // * Do this until the loop iterated the same number of times as the length of
    // ordered_genes, noting that that length will be growing as the loop proceeds.
    let mut index = 0;
    while index < ordered_gene_indices.len() {
        // let gene = &ordered_gene_indices[index];
        let gene = &genes[ordered_gene_indices[index]];
        // only if it hasn't exceeded its follow_count
        if let Some(count) = follow_count.get_mut(&gene.id) {
            if *count < max_gene_follows {
                for gene_idx in &seed[&gene.from] {
                    ordered_gene_indices.push(*gene_idx);
                }

                *count += 1;
            }
        } else {
            follow_count.insert(gene.id, 1);
            for gene_idx in &seed[&gene.from] {
                ordered_gene_indices.push(*gene_idx);
            }
        }

        index += 1;
    }

    ordered_gene_indices
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn makes_stable_ordered_genes() {
        let nnh = NeuralNetHelper::new(0);

        let g1 = Genome::new(GenomeProps {
            neural_net_helper: &nnh,
            size: 10,
        });

        let mut g2 = g1.clone();
        g2.recompute_ordered_gene_indices(&nnh);

        assert_eq!(g1.ordered_gene_indices, g2.ordered_gene_indices);
    }
}
