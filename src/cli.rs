use clap::Parser;

/// An evolutionary ecosystem with LifeForms, food, and danger. Customize some input variables, or
/// don't. Each run will yield different behaviors as the lifeforms evolve and adapt to their
/// environment.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// How big is the world? The world is square (note that terminals will show a much bigger
    /// height since each space is a character which is taller than it is wide).
    #[arg(long, default_value_t = 50)]
    pub size: usize,

    /// How many lifeforms should start on the board?
    #[arg(long, default_value_t = 20)]
    pub num_initial_lifeforms: usize,

    /// How many genes are there? Each gene represents one connection from neuron to neuron.
    #[arg(long, default_value_t = 25)]
    pub genome_size: usize,

    /// What are the chances of a mutation occuring when a lifeform splits when it eats enough
    /// food?
    #[arg(long, default_value_t = 0.1)]
    pub mutation_rate: f32,

    /// After how many frames does a new food appear?
    #[arg(long, default_value_t = 30)]
    pub food_density: usize,

    /// How many inner neurons should there be? These are neurons that don't have input values
    /// based on the world around them, and don't result in a physical action by the lifeform.
    #[arg(long, default_value_t = 5)]
    pub num_inner_neurons: usize,

    /// What is the minimum number of lifeforms on the board at any time before new ones are
    /// created? The new creation scheme looks at the most fit individual and makes some clones of
    /// that one, and also makes a random new lifeform.
    #[arg(long, default_value_t = 5)]
    pub minimum_number_lifeforms: usize,

    /// After how many frames is the danger allowed to move one space?
    #[arg(long, default_value_t = 10)]
    pub danger_delay: usize,

    /// The danger is regarded as radioactive -- this means that it does more damage to a lifeform
    /// the closer the lifeform is to it. The effect of the danger falls off as the square of the
    /// distance from it.
    #[arg(long, default_value_t = 0.5)]
    pub danger_damage: f32,

}

//         size,
//         num_initial_lifeforms: 20,
//         genome_size: 25,
//         mutation_rate: 0.1,
//         food_density: 30,
//         num_inner_neurons,
//         minimum_number_lifeforms: 15,
//         danger_delay: 10,
//         danger_damage: 0.5,
