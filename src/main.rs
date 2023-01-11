use evolution::*;

// TODO
// * Food/water auto generation
// * If food/water will use a frame ticker, move oscillator to use that too
// * Enable physics
// * Output neurons to have effects

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

    World::new(world_props);
}

