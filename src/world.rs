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

    /// After how many tics does the danger randomly move a space
    pub danger_delay: usize,

    /// After how many frames does a new food appear
    pub food_density: usize,

    pub neural_net_helper: &'a NeuralNetHelper,
}

#[derive(Debug)]
pub struct World<'a> {
    props: WorldProps<'a>,
    pub lifeforms: HashMap<usize, LifeForm>,
    pub food: HashSet<(usize, usize)>,
    pub danger: (usize, usize),
    oscillator: f32,
    pub tics: usize,
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
        let danger = (0, 0);

        Self {
            props,
            food,
            danger,
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

        self.update_inputs();

        // To avoid interior mutability, this keeps track of which lifeforms
        // are marked as deceased and will be removed after the mutable loop.
        let mut has_died: Vec<usize> = vec![];
        let mut has_split: Vec<((usize, usize), Genome)> = vec![];

        // do effects of environment on lifeforms
        for mut lf in self.lifeforms.values_mut() {
            lf.hunger += 0.0001;
            lf.lifespan += 1;

            // If the lifeform is on a resource, remove it
            if self.food.remove(&lf.location) {
                lf.hunger -= 0.3;
                lf.health += 0.1;
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

            let dist_to_danger = dist_abs(&lf.location, &self.danger);
            lf.health -= 0.1 / dist_to_danger.powi(2);

            // Let the danger move around slowly
            if self.tics % self.props.danger_delay == 0 {
                randomize(self.props.size, &mut self.danger);
            }

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
            self.lifeforms
                .entry(lf_id)
                .and_modify(|lf| lf.most_recent_output_neuron_values = Some(output_neuron_values));
        }

        self.ensure_lifeform_count();
    }

    fn generate_food(&mut self) {
        self.food.insert(self.random_loc());
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
            if genome.genes.len() == 0 {
                panic!("genome: {:?}", genome);
            }
            Evolver::mutate(&mut genome, &self.props.neural_net_helper);
            let location = most_fit_lf.location.clone();

            let lf = LifeForm {
                id: self.available_lifeform_id(),
                health: 1.0,
                location,
                genome,
                hunger: 0.0,
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
            let mut loc = &mut lf.location;
            let size = self.props.size;

            for (neuron_type, value) in values {
                // This reads as continue on with the probability of value so long as value is above 0.
                if *value <= 0.0 || !thread_rng().gen_bool(*value as f64) {
                    return;
                }

                match neuron_type {
                    OutputNeuronType::TurnLeft => lf.orientation.turn_left(),
                    OutputNeuronType::TurnRight => lf.orientation.turn_right(),
                    OutputNeuronType::MoveForward => {
                        update_location(size, &mut loc, &lf.orientation.get_forward_modifier())
                    }
                    OutputNeuronType::Attack => other_lf_ids_at_loc
                        .iter()
                        .for_each(|id| lfs_to_attack.push(*id)),
                }
            }
        }

        // I'm just not totally sure if I want to completely remove mating. I might want to bring
        // it back so I'm going to leave this here.
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
        //             lf.health += 0.5;
        //         });

        //         self.lifeforms.entry(other_id).and_modify(|lf| {
        //             lf.hunger += 0.5;
        //             lf.health += 0.5;
        //         });

        //         let new_lf = LifeForm {
        //             id: available_id,
        //             genome,
        //             health: 1.0,
        //             hunger: 0.0,
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
            let loc = &lifeform.location;
            let orm = &lifeform.orientation.get_forward_modifier();

            let (num_in_vicinity, closest_lf_health, closest_lf_loc, closest_lf_distance) =
                close_lifeform_info_from_info_vec(
                    self.props.size,
                    lifeform_id,
                    loc,
                    &lfs_id_loc_health,
                );

            for (_nid, (neuron_type, neuron)) in lifeform.neural_net.input_neurons.iter_mut() {
                neuron.value = match neuron_type {
                    InputNeuronType::Random => thread_rng().gen_range(0.0..=1.0),
                    InputNeuronType::Oscillator => self.oscillator,
                    InputNeuronType::Health => lifeform.health,
                    InputNeuronType::Hunger => lifeform.hunger,
                    InputNeuronType::PopulationDensity => num_lifeforms as f32 / size.pow(2) as f32,
                    InputNeuronType::NeighborhoodDensity => (num_in_vicinity / 8) as f32,
                    InputNeuronType::DirectionToFood => rel_dir(loc, orm, closest_food),
                    InputNeuronType::DistanceToFood => dist_rel(size, loc, closest_food),
                    InputNeuronType::DirectionToDanger => rel_dir(loc, orm, &self.danger),
                    InputNeuronType::DistanceToDanger => dist_rel(size, loc, &self.danger),
                    InputNeuronType::DirectionToHealthiestLF => rel_dir(loc, orm, &hlthst_lf_loc),
                    InputNeuronType::DistanceToHealthiestLF => dist_rel(size, loc, &hlthst_lf_loc),
                    InputNeuronType::HealthiestLFHealth => hlthst_lf_health,
                    InputNeuronType::DirectionToClosestLF => rel_dir(loc, orm, &closest_lf_loc),
                    InputNeuronType::DistanceToClosestLF => closest_lf_distance,
                    InputNeuronType::ClosestLFHealth => closest_lf_health,
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
