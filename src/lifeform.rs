use crate::*;

pub struct LifeForm {
    pub id: usize,
    pub health: f32, // 0 - 1
    pub genome: Vec<Gene>,
    pub neural_net: NeuralNet,
    pub hunger: f32, // 0 - 1
    pub thirst: f32, // 0 - 1
    pub location: (usize, usize),
}

impl LifeForm {
    /// returns a list of probablities associated with output neuron types
    pub fn calculate_output_probabilities(&self) -> Vec<(OutputNeuronType, f32)> {
        // * let running_sums = HashMap<neuron_id, sum>
        // * HashMap<neuron_id, NeuronType> (calculated at lifeform birth)

        // TODO in Genome
        // First, we validate:
        // * If there are no genes from input neurons OR no genes to
        // output neurons, the lifeform is invalid.
        // * Remove all genes that lead to an inner neuron that has no output (no genes have a from
        // from that inner neuron)
        // * Remove all genes that come from an inner neuron that has no input.
        // * If there are no genes left, lifeform is invalid.
        //
        // Game plan:
        // * For each gene from an input neuron (no matter its destination is inner or output):
        //  * record up its sums in running_sums[to]
        // * For each gene from an inner neuron to an inner neuron:
        //  * shift the gene out of the vec
        //  * If gene's FROM has no value:
        //      * push it back onto the end of the vec
        //  * else
        //      * add its sum to running_sums[to]
        //  (do this until the vec is empty)
        // * For each remaining gene, genes from inner neurons to output neurons:
        //  * Do the sums

        // This means we'll need the genes to be sorted into trhee pools, those from
        // input neurons, those from inner neurons to inner neurons, and those from
        // inner neurons to output neurons.
    }
}
