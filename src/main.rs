use rand::{thread_rng, Rng};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

pub struct World {
    pub size: usize,
    pub lifeforms: HashMap<usize, ((usize, usize), LifeForm)>,
    pub food: Vec<(usize, usize)>,
    pub food_density: usize, // After how many frames does a new food appear
    pub water: Vec<(usize, usize)>,
    pub water_density: usize, // After how many frames does a new water appear
    pub danger: Vec<(usize, usize)>,
    pub mutation_rate: f32,
    oscillator: f32,
}

pub struct WorldProps {
    size: usize,
    num_initial_lifeforms: usize,
    genome_size: usize,
    food_density: usize,
    water_density: usize,
    mutation_rate: f32,
    num_inner_neurons: usize,
}

impl World {
    pub fn new(props: WorldProps) -> Self {
        // just going to be used for its ids
        let dummy_neural_net = NeuralNet::new(props.num_inner_neurons);

        let input_neuron_ids: Vec<usize> =
            dummy_neural_net.input_neurons.keys().map(|k| *k).collect();
        let inner_neuron_ids: Vec<usize> =
            dummy_neural_net.inner_neurons.keys().map(|k| *k).collect();
        let output_neuron_ids: Vec<usize> =
            dummy_neural_net.output_neurons.keys().map(|k| *k).collect();

        // LifeForm generation
        let mut lifeforms = HashMap::new();

        for lifeform_id in 0..props.num_initial_lifeforms {
            let neural_net = NeuralNet::new(props.num_inner_neurons);

            let mut genome = vec![];
            for _ in 0..props.genome_size {
                genome.push(Gene::new_random(
                    &input_neuron_ids,
                    &inner_neuron_ids,
                    &output_neuron_ids,
                ));
            }

            lifeforms.insert(
                lifeform_id,
                (
                    (0, lifeform_id), // Each lifeform will start out next to each other for now
                    LifeForm {
                        id: lifeform_id,
                        health: 1.0,
                        genome,
                        neural_net,
                        hunger: 0.0,
                        thirst: 0.0,
                    },
                ),
            );
        }

        // Food generation
        let food = vec![(0, 0)];
        let water = vec![(props.size, props.size)];
        let danger = vec![(props.size, 0)];

        Self {
            size: props.size,
            food_density: props.food_density,
            water_density: props.water_density,
            mutation_rate: props.mutation_rate,
            food,
            water,
            danger,
            lifeforms,
            oscillator: 0.0,
        }
    }

    /// Go through each lifeform and update the inputs for their neural_nets
    pub fn update_inputs(&mut self) {

        // Update the oscillator. Easiest to just jump back from one to zero.
        // If it'd be better to have a smooth transition, could use a trig function
        // and a frame counter.
        if self.oscillator < 1.0 {
            self.oscillator += 0.1;
        } else {
            self.oscillator = 0.0;
        }

        let (healthiest_lifeform_health, healthiest_lifeform_location) =
            self.healthiest_lifeform_info();

        // TODO Since there's a method to consume this, this should probably be in its own method
        // to create it.
        let lfs_id_loc_health: Vec<(usize, (usize, usize), f32)> = self
            .lifeforms
            .values()
            .map(|(location, lifeform)| (lifeform.id, *location, lifeform.health))
            .collect();

        let num_lifeforms = self.lifeforms.len();

        for (lifeform_id, (location, lifeform)) in self.lifeforms.iter_mut() {
            // TODO Eventually we probably want to just go through all our stuff
            // ONE TIME and extract all the info, like closest food and water and danger,
            // closest lifeform, etc. Especially for the lifeforms, we don't want to iterate
            // over them many times.
            let closest_food = World::closest_to(&location, &self.food);
            let closest_water = World::closest_to(&location, &self.water);
            let closest_danger = World::closest_to(&location, &self.danger);

            let (num_in_vicinity, closest_lf_health, closest_lf_loc, closest_lf_distance) =
                World::close_lifeform_info_from_info_vec(lifeform_id, location, &lfs_id_loc_health);

            for (_nid, (neuron_type, neuron)) in lifeform.neural_net.input_neurons.iter_mut() {
                neuron.value = match neuron_type {
                    InputNeuronType::DirectionToFood => {
                        World::direction_to(location, &closest_food)
                    }
                    InputNeuronType::DistanceToFood => {
                        World::distance_to_relative(self.size, location, &closest_food)
                    }
                    InputNeuronType::DirectionToWater => {
                        World::direction_to(location, &closest_water)
                    }
                    InputNeuronType::DistanceToWater => {
                        World::distance_to_relative(self.size, location, &closest_water)
                    }
                    InputNeuronType::DirectionToDanger => {
                        World::direction_to(location, &closest_danger)
                    }
                    InputNeuronType::DistanceToDanger => {
                        World::distance_to_relative(self.size, location, &closest_danger)
                    }
                    InputNeuronType::DirectionToHealthiestLF => {
                        World::direction_to(location, &healthiest_lifeform_location)
                    }
                    InputNeuronType::DistanceToHealthiestLF => World::distance_to_relative(
                        self.size,
                        location,
                        &healthiest_lifeform_location,
                    ),
                    InputNeuronType::HealthiestLFHealth => healthiest_lifeform_health,
                    InputNeuronType::DirectionToClosestLF => {
                        World::direction_to(location, &closest_lf_loc)
                    }
                    InputNeuronType::DistanceToClosestLF => closest_lf_distance,
                    InputNeuronType::ClosestLFHealth => closest_lf_health,
                    InputNeuronType::Health => lifeform.health,
                    InputNeuronType::Hunger => lifeform.hunger,
                    InputNeuronType::Thirst => lifeform.thirst,
                    InputNeuronType::PopulationDensity => (num_lifeforms / self.size ^ 2) as f32,
                    InputNeuronType::NeighborhoodDensity => {
                        (num_in_vicinity / 8) as f32
                    }
                    InputNeuronType::Random => {
                        rand::thread_rng().gen_range(0.0..=1.0)
                    }
                    InputNeuronType::Oscillator => {
                        self.oscillator
                    }
                };
            }
        }
    }

    /// TODO explain
    /// Takes id and location of the thing you're trying to find the closest other thing to, very
    /// specificly constructed vector
    /// Returns (
    ///     num_in_vicinity, (number of lifeforms within the vicinity of the lifeform)
    ///     health,  (of closest lf)
    ///     loc, (of closest lf)
    ///     distance (of closest lf)
    /// )
    fn close_lifeform_info_from_info_vec(
        id: &usize,
        location: &(usize, usize),
        lfs_id_loc_health: &Vec<(usize, (usize, usize), f32)>,
    ) -> (usize, f32, (usize, usize), f32) {
        let mut number_in_vicinity: usize = 0;
        let mut closest_lf_health: f32 = 0.0;
        let mut closest_lf_distance = f32::INFINITY;
        let mut closest_lf_location: (usize, usize) = (0, 0);
        for (object_id, loc, health) in lfs_id_loc_health {
            if object_id == id {
                break;
            }

            let dist = World::distance_to_absolute(location, loc);

            if dist < closest_lf_distance {
                closest_lf_health = *health;
                closest_lf_distance = dist;
                closest_lf_location = *loc;
            }

            if dist < 2.0 {
                number_in_vicinity += 1;
            }
        }

        (
            number_in_vicinity,
            closest_lf_health,
            closest_lf_location,
            closest_lf_distance,
        )
    }

    /// Gives a tuple of the healthiest lifeform's health and location
    fn healthiest_lifeform_info(&self) -> (f32, (usize, usize)) {
        let mut healthiest_lifeform_health = 0.0;
        let mut healthiest_lifeform_location: (usize, usize) = (0, 0);
        for (location, lifeform) in self.lifeforms.values() {
            if lifeform.health > healthiest_lifeform_health {
                healthiest_lifeform_health = lifeform.health;
                healthiest_lifeform_location = *location;
            }
        }

        (healthiest_lifeform_health, healthiest_lifeform_location)
    }

    fn closest_to(subject: &(usize, usize), objects: &Vec<(usize, usize)>) -> (usize, usize) {
        let mut shortest_distance = f32::INFINITY;
        let mut closest_object = (0, 0);

        for object in objects {
            let distance = World::distance_to_absolute(subject, object);
            if distance < shortest_distance {
                shortest_distance = distance;
                closest_object = *object;
            }
        }

        closest_object
    }

    // TODO Test this beast too
    fn distance_to_absolute(from: &(usize, usize), to: &(usize, usize)) -> f32 {
        let (x1, y1) = from;
        let (x2, y2) = to;

        (((x2 - x1) ^ 2 + (y2 - y1) ^ 2) as f32).sqrt() as f32
    }

    // TODO Test this beast, document differences
    fn distance_to_relative(world_size: usize, from: &(usize, usize), to: &(usize, usize)) -> f32 {
        let (x1, y1) = from;
        let (x2, y2) = to;

        let farthest_possible = ((2 * (world_size ^ 2)) as f32).sqrt() as f32;

        // root((x2 - x1)^2 + (y2 - y1)^2)
        let total_distance = (((x2 - x1) ^ 2 + (y2 - y1) ^ 2) as f32).sqrt() as f32;

        total_distance / farthest_possible
    }

    // TODO Test the crap out of this
    /// Returns
    /// 0.25 for north
    /// 0.50 for east
    /// 0.75 for south
    /// 1.00 for west
    /// 0.00 for same point
    fn direction_to(from: &(usize, usize), to: &(usize, usize)) -> f32 {
        let (x1, y1) = from;
        let (x2, y2) = to;

        // Ok it's easy to find ourselves in the four quadrants by comparing the x's and y's.
        // To get into the octants, within each quadrant, we can test the differences b/t x's and
        // y's, whichever is bigger will point us to the octant.
        // TODO handle when they're right on the lines

        if x2 > x1 && y2 < y1 {
            // first quadrant
            if x2 - x1 < y2 - y1 {
                // more vertical
                return 0.25; // north
            } else {
                // more horizontal
                return 0.5; // east
            }
        } else if x2 > x1 && y2 > y1 {
            // second quadrant
            if x2 - x1 > y2 - y1 {
                // more horizontal
                return 0.5; // east
            } else {
                // more vertical
                return 0.75; // south
            }
        } else if x2 < x1 && y2 < y1 {
            // third quadrant
            if y2 - y1 > x2 - x1 {
                // more vertical
                return 0.75; // south
            } else {
                // more horizontal
                return 1.0; // west
            }
        } else if x2 < x1 && y2 > y1 {
            // fourth quadrant
            if x2 - x1 > y2 - y1 {
                // more horizontal
                return 1.0; // west
            } else {
                // more vertical
                return 0.25; // north
            }
        }

        if x2 == x1 {
            if y2 > y1 {
                // straight up
                return 0.25; // north
            } else {
                // straight down
                return 0.75; // south
            }
        }

        if y2 == y1 {
            if x2 > y1 {
                // straight right
                return 0.5; // east
            } else {
                return 1.0; // west
            }
        }

        // Otherwise it's the same point
        0.0
    }
}

pub struct Runner {
    pub world: World,
}

impl Runner {
    pub fn mate(_lf1: LifeForm, _lf2: LifeForm) -> LifeForm {
        todo!()
    }

    // with a chance of `rate`(Neuron), flip a single bit
    pub fn mutate(_lf: LifeForm, _rate: f32) {
        todo!()
    }

    // triggers the effect on the world
    pub fn trigger_effect() {
        todo!()
    }

    // does the neuron calculations and runs the output effects
    pub fn step_lifeform(_lf: LifeForm) {
        todo!()
    }
}

pub struct Gene {
    pub from: usize,
    pub to: usize,
    pub weight: f32, // (-4.0 - 4.0)
}

impl Gene {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        Self { from, to, weight }
    }

    pub fn new_random(
        input_ids: &Vec<usize>,
        inner_ids: &Vec<usize>,
        output_ids: &Vec<usize>,
    ) -> Self {
        // from: randomly from input_ids and inner_ids
        // to: randomly from inner_ids and output_ids
        // weight: randomly from -4 to 4

        let from_idx = thread_rng().gen_range(0..input_ids.len() + inner_ids.len());
        let to_idx = thread_rng().gen_range(0..inner_ids.len() + output_ids.len());

        let from: usize;
        if from_idx < input_ids.len() {
            from = input_ids.get(from_idx).unwrap().clone();
        } else {
            from = inner_ids.get(from_idx - input_ids.len()).unwrap().clone();
        }

        let to: usize;
        if to_idx < inner_ids.len() {
            to = inner_ids.get(to_idx).unwrap().clone();
        } else {
            to = output_ids.get(to_idx - inner_ids.len()).unwrap().clone();
        }

        let weight = thread_rng().gen_range(-4.0..4.0);

        Self { from, to, weight }
    }
}

#[derive(Debug)]
pub struct NeuralNet {
    pub input_neurons: HashMap<usize, (InputNeuronType, InputNeuron)>,
    pub output_neurons: HashMap<usize, (OutputNeuronType, OutputNeuron)>,
    pub inner_neurons: HashMap<usize, InnerNeuron>,
}

impl NeuralNet {
    pub fn new(num_inner_neurons: usize) -> Self {
        let mut input_neurons = HashMap::new();
        let mut output_neurons = HashMap::new();
        let mut inner_neurons = HashMap::new();

        for (idx, neuron_member) in InputNeuronType::iter().enumerate() {
            let neuron = InputNeuron {
                // Assuming there'll never be more than 100 input neuron types, we'll do this
                // to assure a different id from the output neurons
                id: idx + 100,

                // TODO should this be random, or..?
                value: 0.0,
            };
            input_neurons.insert(neuron.id, (neuron_member, neuron));
        }

        for (idx, neuron_member) in OutputNeuronType::iter().enumerate() {
            let neuron = OutputNeuron { id: idx + 200 };
            output_neurons.insert(neuron.id, (neuron_member, neuron));
        }

        for idx in 0..num_inner_neurons {
            let neuron = InnerNeuron { id: idx + 300 };
            inner_neurons.insert(neuron.id, neuron);
        }

        Self {
            input_neurons,
            output_neurons,
            inner_neurons,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum InputNeuronType {
    DirectionToFood,
    DistanceToFood,
    DirectionToWater,
    DistanceToWater,
    DirectionToDanger,
    DistanceToDanger,
    DirectionToHealthiestLF,
    DistanceToHealthiestLF,
    HealthiestLFHealth,
    DirectionToClosestLF,
    DistanceToClosestLF,
    ClosestLFHealth,
    Health,
    Hunger,
    Thirst,
    PopulationDensity,
    NeighborhoodDensity,
    Random,
    Oscillator,
}

#[derive(Debug, Default)]
pub struct InputNeuron {
    // TODO One struct per type?
    pub id: usize,
    pub value: f32, // 0.0 - 1.0
}

#[derive(Debug, EnumIter)]
pub enum OutputNeuronType {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Attack,
    Mate,
    Eat,
    Drink,
}

#[derive(Debug, Default)]
pub struct OutputNeuron {
    pub id: usize,
}

#[derive(Debug)]
pub struct InnerNeuron {
    pub id: usize,
}

pub struct LifeForm {
    pub id: usize,
    pub health: f32, // 0 - 1
    pub genome: Vec<Gene>,
    pub neural_net: NeuralNet,
    pub hunger: f32, // 0 - 1
    pub thirst: f32, // 0 - 1
}
