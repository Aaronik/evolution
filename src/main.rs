use std::{
    io,
    time::{Duration, Instant}, thread,
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

fn main() {
    // Size of the world
    let size = 50;

    let num_inner_neurons = 1;

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

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut iteration = 0;
    let mut last_tick = Instant::now();

    // When we pause we greatly increase the tick rate to keep the loop from
    // cooking the CPUs. This is where we store the value to go back to.
    // Note we mutate this to adjust tick rate.
    let mut saved_tick_rate = 1;

    // Will be adjusted within the loop as well
    let mut paused = false;

    // Which lifeform is currently selected within the UI
    let mut selected_lf_index: i32 = 0;

    // TODO So the UI is only capable of drawing like 100 frames per second, even if the program
    // can go much faster than that. So first step to speed up will be to extract the program out
    // into a different thread.

    loop {
        if paused {
            thread::sleep(Duration::from_millis(50));
        }

        terminal
            .draw(|f| ui(f, size, &world, iteration, selected_lf_index, saved_tick_rate))
            .unwrap();

        let mut tick_rate = Duration::from_millis(saved_tick_rate);

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
                            tick_rate = Duration::from_millis(saved_tick_rate);
                        } else {
                            paused = true;
                            tick_rate = Duration::from_secs(u64::MAX);
                        }
                    }
                    KeyCode::Up => selected_lf_index = i32::max(0, selected_lf_index - 1),
                    KeyCode::Down => {
                        selected_lf_index =
                            i32::min(world.lifeforms.len() as i32 - 1, selected_lf_index + 1)
                    }
                    KeyCode::Left => saved_tick_rate = (saved_tick_rate / 3) + 1,
                    KeyCode::Right => saved_tick_rate = saved_tick_rate * 2,
                    _ => (),
                };
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if paused {
                continue;
            }

            world.step();
            last_tick = Instant::now();
        }

        iteration += 1;
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

