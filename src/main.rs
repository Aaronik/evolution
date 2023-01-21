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

use evolution::*;

// TODO
// * foods
// * Order the lifeform ids in the select screen
// * Get timestamps for events
// * Ok, we don't really need water or hearts, we really just need FOOD. A LF will slowly lose
// health until it eats, and then its health will come up by a certain amount. Health is lost
// LINEARLY. Also though it can still be affected by the danger. This will make it so much easier
// to get at the right balances. If a LF has surpassed a certain amount of fullness when it eats
// then it splits!
// * Make rel distance go fro -1 to 1 (more sensitivity)
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
        num_inner_neurons,
        minimum_number_lifeforms: 15,
        danger_delay: 10,
    };

    let mut world = World::new(world_props);

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
    let mut selected_lf_index: i64 = 0;

    // TODO So the UI is only capable of drawing like 100 frames per second, even if the program
    // can go much faster than that. So first step to speed up will be to extract the program out
    // into a different thread.
    // A hack can be brought in by skipping draw frames. If you have a loop counter and only run
    // terminal.draw on certain iterations of that loop, it works pretty alright and the program
    // can run way faster. But really it should just be that the UI keeps up with the program as
    // best it can. If the program's running faster than the UI, then the ui skips frames.

    let mut pause_info = 0;

    let mut should_draw = true;

    loop {
        let lf = world
            .lifeforms
            .iter()
            .map(|i| i.1)
            .nth(selected_lf_index as usize);

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
                    KeyCode::Up => selected_lf_index = i64::max(0, selected_lf_index - 1),
                    KeyCode::Down => {
                        selected_lf_index =
                            i64::min(world.lifeforms.len() as i64 - 1, selected_lf_index + 1)
                    }
                    KeyCode::Left => saved_tick_rate = saved_tick_rate / 3,
                    KeyCode::Right => saved_tick_rate = (saved_tick_rate * 2) + 1,
                    _ => (),
                };

                // These are handy for when the terminal is set to not draw.
                // TODO These should be removed when the UI thread just tries to keep up with the main
                // thread.
                let lf = world
                    .lifeforms
                    .iter()
                    .map(|i| i.1)
                    .nth(selected_lf_index as usize);
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
