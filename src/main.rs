use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use clap::Parser;

use evolution::*;

// TODO
// * Get timestamps for events
// * Make rel distance go fro -1 to 1 (more sensitivity)
// * Let food be a thing that, after it gets a certain age, itself splits into multiple of it. That
//  way it's like plants, getting energy from the ambient system.
// * Let lifeforms know when they're up against the edge (ie. distance to edge, either distance to
// every edge or better would be distance to edge _in front_ of the lf)
// * Should lifeforms leave behind food when they die? This would kind of add a little pressure to
// atack each other


fn main() {
    let args = Args::parse();

    // Size of the world
    let size = args.size;

    let num_inner_neurons = args.num_inner_neurons;

    let nnh = NeuralNetHelper::new(num_inner_neurons);

    let world_props = WorldProps {
        size,
        neural_net_helper: &nnh,
        num_initial_lifeforms: args.num_initial_lifeforms,
        genome_size: args.genome_size,
        mutation_rate: args.mutation_rate,
        food_density: args.food_density,
        num_inner_neurons,
        minimum_number_lifeforms: args.minimum_number_lifeforms,
        danger_delay: args.danger_delay,
        danger_damage: args.danger_damage,
    };

    let world = World::new(world_props);

    run_app(size, world);
}

fn run_app(size: usize, mut world: World) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut last_tick = Instant::now();

    // When we pause we greatly increase the tick rate to keep the loop from
    // cooking the CPUs. This is where we store the value to go back to.
    // Note we mutate this to adjust tick rate.
    let mut saved_tick_rate = 0;

    // Will be adjusted within the loop as well
    let mut paused = false;

    // Which lifeform is currently selected within the UI
    let mut selected_lf_id: Option<usize> = None;

    let mut pause_info = 0;
    let mut should_draw = true;

    loop {
        let lf = selected_lf_id.and_then(|id| world.lifeforms.get(&id));

        if should_draw {
            terminal
                .draw(|f| ui(f, size, &world, lf, saved_tick_rate))
                .unwrap();
        }

        let tick_rate = Duration::from_millis(saved_tick_rate);

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('d') => should_draw = !should_draw,
                    KeyCode::Char('p') => {
                        if paused {
                            paused = false;
                            saved_tick_rate = pause_info;
                        } else {
                            paused = true;
                            pause_info = saved_tick_rate.clone();
                            saved_tick_rate = u64::MAX;
                        }
                    }
                    KeyCode::Up => {
                        if let None = selected_lf_id {
                            selected_lf_id = world.lifeforms.keys().map(|k| *k).last();
                        } else if let Some(id) = selected_lf_id {
                            let current_index = world.lifeforms.keys().position(|lid| lid == &id);

                            if let Some(current_index) = current_index {
                                if current_index == 0 {
                                    selected_lf_id = None;
                                } else {
                                    selected_lf_id =
                                        world.lifeforms.keys().map(|k| *k).nth(current_index - 1)
                                }
                            } else {
                                selected_lf_id = None;
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let None = selected_lf_id {
                            selected_lf_id = world.lifeforms.keys().map(|k| *k).nth(0);
                        } else if let Some(id) = selected_lf_id {
                            let current_index = world.lifeforms.keys().position(|lid| lid == &id);

                            if let Some(current_index) = current_index {
                                if current_index == world.lifeforms.len() {
                                    selected_lf_id = None;
                                } else {
                                    selected_lf_id =
                                        world.lifeforms.keys().map(|k| *k).nth(current_index + 1)
                                }
                            } else {
                                selected_lf_id = None;
                            }
                        }
                    }
                    KeyCode::Left => saved_tick_rate = saved_tick_rate / 3,
                    KeyCode::Right => saved_tick_rate = (saved_tick_rate * 2) + 1,
                    _ => (),
                };

                // These are handy for when the terminal is set to not draw.
                let lf = selected_lf_id.and_then(|id| world.lifeforms.get(&id));

                terminal
                    .draw(|f| ui(f, size, &world, lf, saved_tick_rate))
                    .unwrap();
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.step();
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
