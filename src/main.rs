use std::{
    io,
    time::{Duration, Instant}
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend, Terminal,
};

use evolution::*;

// TODO
// * Start world with many waters,foods,hearts
// * Remove random movement
// * Removing mating
// * Replae top stats thing with actual world stats, like num lifeforms, iterations, tick count
// * Order the lifeform ids in the select screen
// * Keep track of id, not index within the list of ids, so it's not always jumping around
// * Get timestamps for events
// * Ok, we don't really need water or hearts, we really just need FOOD. A LF will slowly lose
// health until it eats, and then its health will come up by a certain amount. Health is lost
// LINEARLY. Also though it can still be affected by the danger. This will make it so much easier
// to get at the right balances. If a LF has surpassed a certain amount of fullness when it eats
// then it splits!
// * The danger should move around!
// * Make rel distance go fro -1 to 10 (more sensitivity)
// * Consider having a direction and being able to turn, but not just move left and right
// * Let food be a thing that, after it gets a certain age, itself splits into multiple of it. That
//   way it's like plants, getting energy from the ambient system.
// * Let lifeforms know when they're up against the edge (ie. distance to edge, either distance to
// every edge or better would be distance to edge _in front_ of the lf)

fn main() {
    // Size of the world
    let size = 50;

    let num_inner_neurons = 3;

    let nnh = NeuralNetHelper::new(num_inner_neurons);

    let world_props = WorldProps {
        size,
        neural_net_helper: &nnh,
        num_initial_lifeforms: 20,
        genome_size: 25,
        mutation_rate: 0.001,
        food_density: 30,
        water_density: 3,
        heals_density: 30,
        num_inner_neurons,
        minimum_number_lifeforms: 15,
        // TODO Add num dangers
    };

    let mut world = World::new(world_props);
    let mut iteration = 0;

    // loop {
    //     iteration += 1;
    //     world.step();
    //     println!("iteration, num lifeforms: {}, {}", iteration, world.lifeforms.len());
    // }
    // return;

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut last_tick = Instant::now();

    // When we pause we greatly increase the tick rate to keep the loop from
    // cooking the CPUs. This is where we store the value to go back to.
    // Note we mutate this to adjust tick rate.
    let mut saved_tick_rate = 500;

    // Will be adjusted within the loop as well
    let mut paused = false;

    // Which lifeform is currently selected within the UI
    let mut selected_lf_index: i64 = 0;

    // TODO So the UI is only capable of drawing like 100 frames per second, even if the program
    // can go much faster than that. So first step to speed up will be to extract the program out
    // into a different thread.

    let mut pause_info = 0;

    loop {
        let lf = world.lifeforms.iter().map(|i| i.1).nth(selected_lf_index as usize);

        terminal
            .draw(|f| ui(f, size, &world, iteration, lf, saved_tick_rate))
            .unwrap();

        let tick_rate = Duration::from_millis(saved_tick_rate);

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
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
                    KeyCode::Up => selected_lf_index = i64::max(0, selected_lf_index - 1),
                    KeyCode::Down => {
                        selected_lf_index =
                            i64::min(world.lifeforms.len() as i64 - 1, selected_lf_index + 1)
                    }
                    KeyCode::Left => saved_tick_rate = saved_tick_rate / 3,
                    KeyCode::Right => saved_tick_rate = (saved_tick_rate * 2) + 1,
                    _ => (),
                };
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.step();
            iteration += 1;
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

