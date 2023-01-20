use crate::*;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct WorldProps<'a> {
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

    /// After how many frames does a new health powerup appear
    pub heals_density: usize,
    pub neural_net_helper: &'a NeuralNetHelper,
}

#[derive(Debug)]
pub struct World<'a> {
    props: WorldProps<'a>,
    pub lifeforms: HashMap<usize, LifeForm>,
    pub food: HashSet<(usize, usize)>,
    pub water: HashSet<(usize, usize)>,
    pub danger: HashSet<(usize, usize)>,
    pub heals: HashSet<(usize, usize)>,
    oscillator: f32,
    tics: usize,
    pub events: Vec<(EventType, String)>,
}

#[derive(Debug)]
pub enum EventType {
    Death,
    Creation,
    Mate,
    Attack,
    AsexuallyReproduce,
}

impl<'a> World<'a> {
    pub fn new(props: WorldProps<'a>) -> Self {
        let neural_net_helper = &props.neural_net_helper;

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
        let heals = HashSet::new();
        let danger = HashSet::from([(0, 0)]); // TODO make random, take variable amount
                                              // TODO Add a health booster

        Self {
            props,
            food,
            water,
            danger,
            heals,
            lifeforms,
            oscillator: 0.0,
            tics: 0,
            events: vec![],
        }
    }

    pub fn step(&mut self) {
        self.tics += 1;
        self.oscillator = (self.tics as f32 / 10.0).sin();

        // Update resources
        if self.tics % self.props.food_density == 0 {
            self.generate_food();
        }

        if self.tics % self.props.water_density == 0 {
            self.generate_water();
        }

        if self.tics % self.props.heals_density == 0 {
            self.generate_heals();
        }

        self.update_inputs();

        // To avoid interior mutability, this keeps track of which lifeforms
        // are marked as deceased and will be removed after the mutable loop.
        let mut has_died: Vec<usize> = vec![];
        let mut has_split: Vec<((usize, usize), Genome)> = vec![];

        // do effects of environment on lifeforms
        for mut lf in self.lifeforms.values_mut() {
            lf.hunger += 0.000001;
            lf.thirst += 0.00001;
            lf.lifespan += 1;

            // If the lifeform is on a resource, remove it
            if self.food.remove(&lf.location) {
                lf.hunger -= 0.5;
                if lf.hunger < 0.0 {
                    lf.hunger = 0.0;
                    has_split.push((lf.location.clone(), lf.genome.clone()));
                    self.events.push((
                        EventType::AsexuallyReproduce,
                        format!(
                            "=> Lifeform {} has reproduced asexually by eating enough food!",
                            lf.id
                        ),
                    ));
                }
            }

            if self.water.remove(&lf.location) {
                lf.thirst = 0.0;
            }

            if self.heals.remove(&lf.location) {
                lf.health = 1.0;
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
                has_died.push(lf.id);
            }
        }

        for lf_id in has_died {
            // TODO When a really healthy one dies, it'd be nice if it reproduced
            self.lifeforms.remove(&lf_id);
            self.events
                .push((EventType::Death, format!("=> Lifeform {} has died!", lf_id)));
        }

        for info in has_split {
            let id = self.available_lifeform_id();
            let mut genome = info.1;

            if Evolver::should_mutate(self.props.mutation_rate) {
                Evolver::mutate(&mut genome, self.props.neural_net_helper)
            }

            self.lifeforms.insert(
                id,
                LifeForm {
                    id,
                    genome,
                    health: 1.0,
                    hunger: 0.0,
                    thirst: 0.0,
                    location: info.0,
                    lifespan: 0,
                    neural_net: self.props.neural_net_helper.spawn(),
                    most_recent_output_neuron_values: None,
                    orientation: Direction::new(),
                },
            );
        }

        // Run the neural net calculations. Uses rayon's par_iter() to parallelise the calculations
        // across threads.
        let all_output_neuron_values: Vec<(usize, Vec<(OutputNeuronType, f32)>)> = self
            .lifeforms
            .par_iter()
            .map(|(lf_id, lf)| (*lf_id, lf.run_neural_net(&self.props.neural_net_helper)))
            .collect();

        for (lf_id, output_neuron_values) in all_output_neuron_values {
            self.process_output_neuron_values(&lf_id, &output_neuron_values);
            self.lifeforms.entry(lf_id).and_modify(|lf| lf.most_recent_output_neuron_values = Some(output_neuron_values));
        }

        self.ensure_lifeform_count();
    }

    fn generate_food(&mut self) {
        self.food.insert(self.random_loc());
    }

    fn generate_water(&mut self) {
        self.water.insert(self.random_loc());
    }

    fn generate_heals(&mut self) {
        self.heals.insert(self.random_loc());
    }

    fn random_loc(&self) -> (usize, usize) {
        (
            thread_rng().gen_range(0..self.props.size),
            thread_rng().gen_range(0..self.props.size),
        )
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
                    &self.props.neural_net_helper,
                );
                self.events.push((
                    EventType::Creation,
                    format!(
                        "=> New lifeform {} has been created with a random genome due to insufficient population",
                        &lf.id
                    ),
                ));
                self.lifeforms.insert(lf.id, lf);
            }

            return;
        }

        // Make a few clones
        for _ in 0..3 {
            let most_fit_lf = self.most_fit_lifeform();
            let mut genome = most_fit_lf.genome.clone();
            let location = most_fit_lf.location.clone();
            Evolver::mutate(&mut genome, &self.props.neural_net_helper);

            let lf = LifeForm {
                id: self.available_lifeform_id(),
                health: 1.0,
                location,
                genome,
                hunger: 0.0,
                thirst: 0.0,
                lifespan: 0,
                neural_net: self.props.neural_net_helper.spawn(),
                most_recent_output_neuron_values: None,
                orientation: Direction::new(),
            };

            self.events.push((
                EventType::Creation,
                format!(
                    "=> New lifeform {} has been created based on lifeform {} due to insufficient population",
                    &lf.id, most_fit_lf.id
                ),
            ));
            self.lifeforms.insert(lf.id, lf);
        }

        let lf = LifeForm::new(
            self.available_lifeform_id(),
            self.props.genome_size,
            &self.props.neural_net_helper,
        );
        self.events.push((
            EventType::Creation,
            format!(
                "=> New lifeform {} has been created with a random genome due to insufficient population",
                &lf.id
            ),
        ));
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

    fn process_output_neuron_values(
        &mut self,
        lf_id: &usize,
        values: &Vec<(OutputNeuronType, f32)>,
    ) {
        let other_lf_ids_at_loc =
            self.other_lf_ids_at_location(*lf_id, &self.lifeforms[lf_id].location);

        // let mut lfs_to_mate_with: Vec<usize> = vec![];
        let mut lfs_to_attack: Vec<usize> = vec![];

        {
            let lf = self.lifeforms.get_mut(lf_id).unwrap();
            let mut loc = &mut lf.location; // TODO
            let size = self.props.size;

            for (neuron_type, value) in values {
                // This reads as continue on with the probability of value so long as value is above 0.
                if *value <= 0.0 || !thread_rng().gen_bool(*value as f64) {
                    return;
                }

                // TODO I've gotta find a better way to update the location at MoveForward.
                // Maybe it's a util method, maybe lf.location becomes a struct
                match neuron_type {
                    OutputNeuronType::TurnLeft => lf.orientation.turn_left(),
                    OutputNeuronType::TurnRight => lf.orientation.turn_right(),
                    OutputNeuronType::MoveForward => update_location(size, &mut loc, &lf.orientation.get_forward_modifier()),
                    OutputNeuronType::Attack => other_lf_ids_at_loc
                        .iter()
                        .for_each(|id| lfs_to_attack.push(*id)),
                }
            }
        }

        // for other_id in lfs_to_mate_with {
        //     for _ in 0..2 {
        //         let available_id = self.available_lifeform_id();
        //         let location = self.lifeforms[lf_id].location;
        //         let g1 = &self.lifeforms[lf_id].genome;
        //         let g2 = &self.lifeforms[&other_id].genome;
        //         let mut genome = Evolver::mate(&g1, &g2, &self.props.neural_net_helper);
        //         if Evolver::should_mutate(self.props.mutation_rate) {
        //             Evolver::mutate(&mut genome, &self.props.neural_net_helper);
        //         }

        //         self.lifeforms.entry(*lf_id).and_modify(|lf| {
        //             lf.hunger += 0.5;
        //             lf.thirst += 0.5;
        //             lf.health += 0.5;
        //         });

        //         self.lifeforms.entry(other_id).and_modify(|lf| {
        //             lf.hunger += 0.5;
        //             lf.thirst += 0.5;
        //             lf.health += 0.5;
        //         });

        //         let new_lf = LifeForm {
        //             id: available_id,
        //             genome,
        //             health: 1.0,
        //             hunger: 0.0,
        //             thirst: 0.0,
        //             lifespan: 0,
        //             location,
        //             neural_net: self.props.neural_net_helper.spawn(),
        //         };

        //         self.events.push((
        //             EventType::Mate,
        //             String::from(format!(
        //                 "=> New lifeform {} was birthed from {lf_id} and {other_id}",
        //                 &new_lf.id
        //             )),
        //         ));
        //         self.lifeforms.insert(new_lf.id, new_lf);
        //     }
        // }

        for other_id in lfs_to_attack {
            self.lifeforms.entry(*lf_id).and_modify(|lf| {
                lf.hunger += 0.3;
                lf.thirst += 0.5;
                lf.health = lf.health / 2.0;
            });

            self.lifeforms.entry(other_id).and_modify(|lf| {
                lf.health = lf.health / 2.0;
            });

            self.events.push((
                EventType::Attack,
                String::from(format!("=> {lf_id} just attacked {other_id}!!")),
            ));
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
            let closest_heal = &closest_to(
                &lifeform.location,
                &self.heals.iter().map(|loc| *loc).collect(),
            );
            let closest_dang = &closest_to(
                &lifeform.location,
                &self.danger.iter().map(|loc| *loc).collect(),
            );
            let loc = &lifeform.location;

            let (num_in_vicinity, closest_lf_health, closest_lf_loc, closest_lf_distance) =
                close_lifeform_info_from_info_vec(
                    self.props.size,
                    lifeform_id,
                    loc,
                    &lfs_id_loc_health,
                );

            for (_nid, (neuron_type, neuron)) in lifeform.neural_net.input_neurons.iter_mut() {
                neuron.value = match neuron_type {
                    InputNeuronType::DirectionToFood => direc(loc, closest_food),
                    InputNeuronType::DistanceToFood => dist_rel(size, loc, closest_food),
                    InputNeuronType::DirectionToWater => direc(loc, closest_wat),
                    InputNeuronType::DistanceToWater => dist_rel(size, loc, closest_wat),
                    InputNeuronType::DirectionToHeal => direc(loc, closest_heal),
                    InputNeuronType::DistanceToHeal => dist_rel(size, loc, closest_heal),
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
                    InputNeuronType::PopulationDensity => num_lifeforms as f32 / size.pow(2) as f32,
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

    pub fn other_lf_ids_at_location(&self, id: usize, location: &(usize, usize)) -> Vec<usize> {
        let mut lf_ids = vec![];

        // TODO This could be sped up by keeping a hashmap of refs to the lifeforms keyed on their
        // locations
        for lf in self.lifeforms.values() {
            if &lf.location == location && lf.id != id {
                lf_ids.push(lf.id);
            }
        }

        lf_ids
    }
}
