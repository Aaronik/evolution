use evolution::*;

// TODO
// * Make output neuron effects
// * Add physics for when lifeforms get down to so many, they auto reproduce

fn main() {
    let world_props = WorldProps {
        size: 30,
        num_initial_lifeforms: 10,
        genome_size: 45,
        mutation_rate: 0.001,
        food_density: 30,
        water_density: 30,
        num_inner_neurons: 3,
    };

    let mut world = World::new(world_props);

    // println!("{:#?}", world.lifeforms.values().last().unwrap());
    loop {
        world.step();
        // TODO This panics sometimes because the lifeforms are friggin dying
        // and I'm not generating new ones!
        println!("{:#?}", world.lifeforms.values().last().unwrap());
    }
}

