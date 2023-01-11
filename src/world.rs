use rand::{thread_rng, Rng};
use std::collections::HashMap;
use crate::*;

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
    pub size: usize,
    pub num_initial_lifeforms: usize,
    pub genome_size: usize,
    pub food_density: usize,
    pub water_density: usize,
    pub mutation_rate: f32,
    pub num_inner_neurons: usize,
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
                    InputNeuronType::NeighborhoodDensity => (num_in_vicinity / 8) as f32,
                    InputNeuronType::Random => thread_rng().gen_range(0.0..=1.0),
                    InputNeuronType::Oscillator => self.oscillator,
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

