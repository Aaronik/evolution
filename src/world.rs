use crate::*;
use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct WorldProps {
    pub size: usize,
    pub num_initial_lifeforms: usize,
    pub genome_size: usize,
    pub mutation_rate: f32,
    pub num_inner_neurons: usize,
    pub minimum_number_lifeforms: usize,

    /// After how many frames does a new food appear
    pub food_density: usize,

    /// After how many frames does a new water appear
    pub water_density: usize,
}

#[derive(Debug)]
pub struct World {
    props: WorldProps,
    pub lifeforms: HashMap<usize, LifeForm>,
    pub food: HashSet<(usize, usize)>,
    pub water: HashSet<(usize, usize)>,
    pub danger: HashSet<(usize, usize)>,
    oscillator: f32,
    tics: usize,
    neural_net_helper: NeuralNetHelper,
}

impl World {
    pub fn new(props: WorldProps) -> Self {
        let neural_net_helper = NeuralNetHelper::new(props.num_inner_neurons);

        // LifeForm generation
        let mut lifeforms = HashMap::new();

        for lifeform_id in 0..props.num_initial_lifeforms {
            lifeforms.insert(
                lifeform_id,
                LifeForm::new(lifeform_id, props.genome_size, &neural_net_helper),
            );
        }

        // Food generation
        let food = HashSet::new();
        let water = HashSet::new();
        let danger = HashSet::from([(props.size / 2, props.size)]); // TODO make random, take variable amount

        Self {
            props,
            food,
            water,
            danger,
            lifeforms,
            neural_net_helper,
            oscillator: 0.0,
            tics: 0,
        }
    }

    pub fn step(&mut self) {
        self.tics += 1;
        self.oscillator = (self.tics as f32 / 10.0).sin();

        // Update food and water
        if self.tics % self.props.food_density == 0 {
            self.food.insert((
                thread_rng().gen_range(0..self.props.size),
                thread_rng().gen_range(0..self.props.size),
            ));
        }

        if self.tics % self.props.water_density == 0 {
            self.water.insert((
                thread_rng().gen_range(0..self.props.size),
                thread_rng().gen_range(0..self.props.size),
            ));
        }

        self.update_inputs();

        // In order to prevent data races, we clone each lifeform and loop
        // over that set, which allows us to call other methods that reference our lifeforms
        // hashmap.
        let mut lfs: Vec<LifeForm> = vec![];
        for lf in self.lifeforms.values_mut() {
            lfs.push(lf.clone());
        }

        // do effects of environment on lifeforms
        for mut lf in lfs {
            lf.hunger += 0.00000001;
            lf.thirst += 0.0000001;
            lf.lifespan += 1;

            // If the lifeform is on a resource, remove it
            if self.food.remove(&lf.location) {
                lf.hunger = 0.0;
            }

            if self.water.remove(&lf.location) {
                lf.thirst = 0.0;
            }

            lf.health -= lf.hunger;

            // TODO Eventually want this to have a more cool effect, like inihibiting
            // the accuracy of input neurons. Not MVP though, but totally doable by having
            // a function that wraps input neuron assignment and kind of randomly jacks the
            // number proportional to the thirst level of the creature. Will be interesting to
            // see how that evolves relative to having it be the same as hunger.
            lf.health -= lf.thirst;

            // TODO make this closest_danger, this assumes there's at least one danger
            let dist_to_danger = dist_abs(&lf.location, &self.danger.iter().last().unwrap());
            lf.health -= 0.01 / dist_to_danger.powi(2);

            if lf.health <= 0.0 {
                self.lifeforms.remove(&lf.id);
                continue;
            }

            // Enact the effects of output neurons
            let output_neuron_probabilities = lf.calculate_output_probabilities();
            self.process_output_probabilities(&mut lf, output_neuron_probabilities);

            // Overwrite the lifeform in our hashmap
            self.lifeforms.insert(lf.id, lf);
        }

        self.ensure_lifeform_count();
    }

    /// Keep a minimum number of lifeforms on the board. If there are none,
    /// create a batch of random ones. If there are still living ones on the board, take
    /// the ones who are the most fit and clone them.
    fn ensure_lifeform_count(&mut self) {
        if self.lifeforms.len() >= self.props.minimum_number_lifeforms {
            return;
        }
        // If there are none, we can't get some from the most fit, so we'll make
        // a whole batch of randoms.
        if self.lifeforms.len() == 0 {
            for _ in 0..self.props.minimum_number_lifeforms {
                let lf = LifeForm::new(
                    self.available_lifeform_id(),
                    self.props.genome_size,
                    &self.neural_net_helper,
                );
                self.lifeforms.insert(lf.id, lf);
            }

            return;
        }

        // Get the most fit individual for later cloning
        let most_fit_lf = self.most_fit_lifeform();
        let mut to_add: Vec<LifeForm> = vec![];

        // Make a few clones
        for offset in 0..3 {
            let genome: Genome;

            if Evolver::should_mutate(1.0) {
                genome = Evolver::mutate(&most_fit_lf.genome, &self.neural_net_helper);
            } else {
                genome = most_fit_lf.genome.clone();
            }

            let lf = LifeForm {
                id: self.available_lifeform_id(),
                health: 1.0,
                location: (most_fit_lf.location.0 + offset, most_fit_lf.location.1),
                genome,
                hunger: 0.0,
                thirst: 0.0,
                lifespan: 0,
                neural_net: self.neural_net_helper.spawn(),
            };

            to_add.push(lf);
        }

        for lf in to_add {
            self.lifeforms.insert(lf.id, lf);
        }
    }

    fn most_fit_lifeform(&self) -> &LifeForm {
        let mut most_fit_lf: Option<&LifeForm> = None;

        for lf in self.lifeforms.values() {
            if let Some(most_fit) = most_fit_lf {
                if Evolver::fitness(lf) > Evolver::fitness(&most_fit) {
                    most_fit_lf = Some(lf);
                }
            } else {
                most_fit_lf = Some(lf);
            }
        }

        most_fit_lf.unwrap()
    }

    fn process_output_probabilities(
        &self,
        lf: &mut LifeForm,
        probabilities: Vec<(OutputNeuronType, f32)>,
    ) {
        let mut loc = lf.location;
        let size = self.props.size;

        for (neuron_type, value) in probabilities {
            if value <= 0.0 {
                return;
            }

            // This reads as continue on with the probability of value so long as value is above 0.
            if !thread_rng().gen_bool(value as f64) {
                return;
            }

            match neuron_type {
                OutputNeuronType::MoveUp if loc.1 == 0 => (),
                OutputNeuronType::MoveUp => loc.1 -= 1,
                OutputNeuronType::MoveRight => loc.0 = usize::min(loc.0 + 1, size),
                OutputNeuronType::MoveDown => loc.1 = usize::min(loc.1 + 1, size),
                OutputNeuronType::MoveLeft if loc.0 == 0 => (),
                OutputNeuronType::MoveLeft => loc.0 -= 1,
                OutputNeuronType::Attack => (),
                OutputNeuronType::Mate => (),
            }
        }

        // We can only move there if it's unoccupied!
        // TODO This is not working, or somehow lfs are still all
        // on top of each other.
        if let None = self.lifeform_at_location(&loc) {
            lf.location = loc;
        }

    }

    /// Go through each lifeform and update the inputs for their neural_nets
    fn update_inputs(&mut self) {
        let (hlthst_lf_health, hlthst_lf_loc) = self.healthiest_lifeform_info();
        let lfs_id_loc_health = generate_lifeform_info_vec(&self.lifeforms);
        let num_lifeforms = self.lifeforms.len();
        let size = self.props.size;

        for (lifeform_id, lifeform) in self.lifeforms.iter_mut() {
            let closest_food = &closest_to(
                &lifeform.location,
                &self.food.iter().map(|loc| *loc).collect(),
            );
            let closest_wat = &closest_to(
                &lifeform.location,
                &self.water.iter().map(|loc| *loc).collect(),
            );
            let closest_dang = &closest_to(
                &lifeform.location,
                &self.danger.iter().map(|loc| *loc).collect(),
            );
            let loc = &lifeform.location;

            let (num_in_vicinity, closest_lf_health, closest_lf_loc, closest_lf_distance) =
                close_lifeform_info_from_info_vec(lifeform_id, loc, &lfs_id_loc_health);

            for (_nid, (neuron_type, neuron)) in lifeform.neural_net.input_neurons.iter_mut() {
                neuron.value = match neuron_type {
                    InputNeuronType::DirectionToFood => direc(loc, closest_food),
                    InputNeuronType::DistanceToFood => dist_rel(size, loc, closest_food),
                    InputNeuronType::DirectionToWater => direc(loc, closest_wat),
                    InputNeuronType::DistanceToWater => dist_rel(size, loc, closest_wat),
                    InputNeuronType::DirectionToDanger => direc(loc, closest_dang),
                    InputNeuronType::DistanceToDanger => dist_rel(size, loc, closest_dang),
                    InputNeuronType::DirectionToHealthiestLF => direc(loc, &hlthst_lf_loc),
                    InputNeuronType::DistanceToHealthiestLF => dist_rel(size, loc, &hlthst_lf_loc),
                    InputNeuronType::HealthiestLFHealth => hlthst_lf_health,
                    InputNeuronType::DirectionToClosestLF => direc(loc, &closest_lf_loc),
                    InputNeuronType::DistanceToClosestLF => closest_lf_distance,
                    InputNeuronType::ClosestLFHealth => closest_lf_health,
                    InputNeuronType::Health => lifeform.health,
                    InputNeuronType::Hunger => lifeform.hunger,
                    InputNeuronType::Thirst => lifeform.thirst,
                    InputNeuronType::PopulationDensity => (num_lifeforms / size ^ 2) as f32,
                    InputNeuronType::NeighborhoodDensity => (num_in_vicinity / 8) as f32,
                    InputNeuronType::Random => thread_rng().gen_range(0.0..=1.0),
                    InputNeuronType::Oscillator => self.oscillator,
                };
            }
        }
    }

    /// Gives a tuple of the healthiest lifeform's health and location
    fn healthiest_lifeform_info(&self) -> (f32, (usize, usize)) {
        let mut healthiest_lifeform_health = 0.0;
        let mut healthiest_lifeform_location: (usize, usize) = (0, 0);
        for lifeform in self.lifeforms.values() {
            if lifeform.health > healthiest_lifeform_health {
                healthiest_lifeform_health = lifeform.health;
                healthiest_lifeform_location = lifeform.location;
            }
        }

        (healthiest_lifeform_health, healthiest_lifeform_location)
    }

    fn available_lifeform_id(&self) -> usize {
        let mut extent_ids: HashSet<usize> = HashSet::new();
        for lf in self.lifeforms.values() {
            extent_ids.insert(lf.id);
        }

        let mut id: usize = 0;

        for potential_id in 0..=self.lifeforms.len() {
            if !extent_ids.contains(&potential_id) {
                id = potential_id;
                break;
            }
        }

        id
    }

    pub fn lifeform_at_location(&self, location: &(usize, usize)) -> Option<&LifeForm> {
        for lf in self.lifeforms.values() {
            if &lf.location == location {
                return Some(lf);
            }
        }

        None
    }

}

/// Generates a vec that has a very specific set of information relative to a lifeform, to be used
/// later on with the close_lifeform_info_from_info_vec function
fn generate_lifeform_info_vec(
    lifeforms: &HashMap<usize, LifeForm>,
) -> Vec<(usize, (usize, usize), f32)> {
    lifeforms
        .values()
        .map(|lifeform| (lifeform.id, lifeform.location, lifeform.health))
        .collect()
}

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

        let dist = dist_abs(location, loc);

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

fn closest_to(subject: &(usize, usize), objects: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut shortest_distance = f32::INFINITY;
    let mut closest_object = (0, 0);

    for object in objects {
        let distance = dist_abs(subject, object);
        if distance < shortest_distance {
            shortest_distance = distance;
            closest_object = *object;
        }
    }

    closest_object
}

// TODO Test this beast too
fn dist_abs(from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.0 as f32;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// TODO Test this beast, document differences
fn dist_rel(world_size: usize, from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.0 as f32;

    let farthest_possible = ((2 * (world_size ^ 2)) as f32).sqrt();

    // root((x2 - x1)^2 + (y2 - y1)^2)
    let total_distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    total_distance / farthest_possible
}

// TODO Test the crap out of this
/// Returns
/// 0.25 for north
/// 0.50 for east
/// 0.75 for south
/// 1.00 for west
/// 0.00 for same point
fn direc(from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.0 as f32;

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
