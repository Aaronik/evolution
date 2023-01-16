use std::{
    collections::HashMap,
    io, thread,
    time::{Duration, Instant}, cell::Cell,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph, Widget, Table, Row},
    Frame, Terminal,
};

use evolution::*;

fn main() {
    let size = 50;
    let tick_rate = Duration::from_millis(10);

    let num_inner_neurons = 3;

    let nnh = NeuralNetHelper::new(num_inner_neurons);

    let world_props = WorldProps {
        size,
        neural_net_helper: &nnh,
        num_initial_lifeforms: 20,
        genome_size: 25,
        mutation_rate: 0.001,
        food_density: 300,
        water_density: 30,
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

    let mut paused = false;

    loop {
        terminal.draw(|f| ui(f, size, &world, iteration)).unwrap();

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') => paused = !paused,

                    // TODO here could pause this loop, call a fn that has another
                    // loop that just steps. In that fn though need to figure out
                    // how to capture key events.
                    // Alternatively, could have e mean do like 10,000 frames or something
                    // without UI. So like a quick jump into the future.
                    KeyCode::Char('e') => todo!(),
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

fn ui<B>(f: &mut Frame<B>, size: usize, world: &World, iteration: usize)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(20)].as_ref())
        .split(f.size());

    draw_main(f, size, world, chunks[0]);
    draw_controls(f, chunks[1], iteration);
}

fn draw_main<B>(f: &mut Frame<B>, size: usize, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(10)].as_ref())
        .split(area);

    draw_world(f, size, world, chunks[0]);
    draw_right(f, world, chunks[1]);
}

fn draw_world<B>(f: &mut Frame<B>, size: usize, world: &World, area: Rect)
where
    B: Backend,
{
    let world_canvas = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .x_bounds([0.0, size as f64])
        .y_bounds([0.0, size as f64])
        .paint(|ctx| {
            let mut num_at_location: HashMap<(usize, usize), usize> = HashMap::new();

            for lf in world.lifeforms.values() {
                *num_at_location.entry(lf.location).or_insert(0) += 1;
                let num = num_at_location[&lf.location];

                let char = match num {
                    1 if lf.health >= 0.5 => "☺",
                    1 if lf.health < 0.5 => "☹",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    9 => "9",
                    _ => "!",
                };

                let color = match lf.health {
                    _ if lf.health >= 0.9 => Color::LightGreen,
                    _ if lf.health >= 0.8 => Color::Green,
                    _ if lf.health >= 0.7 => Color::LightBlue,
                    _ if lf.health >= 0.6 => Color::Blue,
                    _ if lf.health >= 0.5 => Color::Magenta,
                    _ if lf.health >= 0.4 => Color::LightMagenta,
                    _ if lf.health >= 0.3 => Color::Yellow,
                    _ if lf.health >= 0.2 => Color::LightYellow,
                    _ if lf.health >= 0.1 => Color::LightRed,
                    _ if lf.health < 0.1 => Color::Red,
                    _ => Color::White,
                };

                // TODO add_modifier Modifier::BOLD
                ctx.print(
                    lf.location.0 as f64,
                    lf.location.1 as f64,
                    Span::styled(char, Style::default().fg(color)),
                );
            }

            for water in &world.water {
                ctx.print(
                    water.0 as f64,
                    water.1 as f64,
                    Span::styled("W", Style::default().fg(Color::Blue)),
                );
            }

            for food in &world.food {
                ctx.print(
                    food.0 as f64,
                    food.1 as f64,
                    Span::styled("F", Style::default().fg(Color::Green)),
                );
            }

            for danger in &world.danger {
                ctx.print(
                    danger.0 as f64,
                    danger.1 as f64,
                    Span::styled("☠", Style::default().fg(Color::Red)),
                );
            }
        });

    f.render_widget(world_canvas, area);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect, iteration: usize)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let text = format!( "controls: q = quit | p = pause | f = change frame rate | e = evolve without UI | iteration {}", iteration);
    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn draw_right<B>(f: &mut Frame<B>, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_stats(f, world, chunks[0]);
    draw_events(f, chunks[1]);
}

fn draw_stats<B>(f: &mut Frame<B>, world: &World, area: Rect)
where
    B: Backend,
{
    let header = Row::new(["id", "lifespan", "health", "hunger", "thirst", "location"])
        .height(1)
        .bottom_margin(1);

    let rows = world.lifeforms.values().map(|lf| {
        let cells = vec![
            lf.id.to_string(),
            lf.lifespan.to_string(),
            lf.health.to_string(),
            lf.hunger.to_string(),
            lf.thirst.to_string(),
            format!("({}, {})", lf.location.0, lf.location.1),
        ];
        Row::new(cells)
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .widths(&[
            Constraint::Length(3),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]);

    f.render_widget(table, area);
}

fn draw_events<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Events").borders(Borders::ALL);
    f.render_widget(block, area);

    // let mut lis: Vec<ListItem> = vec![ListItem::new(Span::from(
    //     "Stats: id, lifespan, health, hunger, thirst, location",
    // ))];

    // for lf in world.lifeforms.values() {
    //     let stat = (
    //         lf.id,
    //         lf.lifespan,
    //         lf.health,
    //         lf.hunger,
    //         lf.thirst,
    //         lf.location,
    //     );
    //     lis.push(ListItem::new(Span::from(format!("{:.10?}", stat))));
    // }

    // let list = List::new(lis).block(Block::default().title("Stats").borders(Borders::ALL));

}

//         // Mouse has been moved or clicked
//         Event::Mouse(mouseevent) => {
//             if let MouseEventKind::Down(_) = mouseevent.kind {
//                 paused = true;
//                 let loc = (mouseevent.column as usize, mouseevent.row as usize);
//                 let lf = world.lifeform_at_location(&loc);
//                 if let Some(lf) = lf {
//                     update_info_screen(lf, &nnh, &mut info_screen);
//                     engine.print_screen((size + 2) as i32, 0, &info_screen);
//                 }
//             }
//         }

// fn update_info_screen(lf: &LifeForm, nnh: &NeuralNetHelper, screen: &mut Screen) {
//     // Clear the screen part that we're using
//     screen.clear();

//     let x = 0 as i32;
//     let y = 0 as i32;
//     screen.print(x, y, &format!("LifeForm {} at {:?}", lf.id, lf.location));
//     let y = y + 1;
//     screen.print(x, y, "------- INPUTS --------");
//     let y = y + 1;
//     for (idx, (neuron_type, neuron)) in lf.neural_net.input_neurons.values().enumerate() {
//         screen.print(
//             x,
//             y + idx as i32,
//             &format!("{:?}: {:?}", neuron_type, neuron.value),
//         );
//     }
//     let y = y + lf.neural_net.input_neurons.len() as i32;
//     screen.print(x, y, "------- OUTPUTS -------");
//     let y = y + 1;
//     let probabilities = lf.run_neural_net(&nnh);
//     for (idx, (neuron_type, prob)) in probabilities.iter().enumerate() {
//         screen.print(x, y + idx as i32, &format!("{:?}: {}", neuron_type, prob));
//     }
//     let y = y + probabilities.len() as i32;
//     screen.print(x, y, "-------");
//     screen.draw();
// }

// fn update_stats_screen(screen: &mut Screen, world: &World) {

//     screen.draw();
// }
