use evolution::*;

// TODO
// * Food/water auto generation
// * Enable physics
// * Based on input neurons and genes, calculate output neuron likelihoods
// * Make output neuron effects

fn main() {
    let world_props = WorldProps {
        size: 30,
        num_initial_lifeforms: 10,
        genome_size: 10,
        mutation_rate: 0.001,
        food_density: 30,
        water_density: 30,
        num_inner_neurons: 5,
    };

    let mut world = World::new(world_props);
    world.step();
}

