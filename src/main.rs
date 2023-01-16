use std::{collections::HashMap, io, thread, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};

use evolution::*;

// TODO
// * Make output neuron effects
// * Add physics for when lifeforms get down to so many, they auto reproduce
// * For future event handling: https://qiita.com/wangya_eecs/items/b9e1a501cb0c0ab0de1c

fn main() {
    let size = 40;
    let frame_rate = 1000;
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

    let width = (size * 3) as u16;
    let height = (size + 2) as u16;

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Ok, I want:
    // * a main screen (top left)
    // * controls footer
    // * live update stats (top right)
    // * event log (bottom right)
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(size as u16), Constraint::Min(20)].as_ref())
                .split(f.size());

            draw_main(f, size, chunks[0]);
            draw_controls(f, chunks[1]);
        })
        .unwrap();

    thread::sleep(Duration::from_millis(5000));

    // let mut st = String::new();
    // std::io::stdin().read_line(&mut st).expect("issue reading input");
    // println!("st: {}", st);

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

fn draw_main<B>(f: &mut Frame<B>, size: usize, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(10)].as_ref())
        .split(area);

    draw_world(f, chunks[0]);
    draw_right(f, chunks[1]);
}

fn draw_world<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("World").borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let paragraph = Paragraph::new(
        "controls: q = quit | p = pause | f = change frame rate | e = evolve without UI | frame {}",
    )
    .block(block);

    f.render_widget(paragraph, area);
}

fn draw_right<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_stats(f, chunks[0]);
    draw_events(f, chunks[1]);
}

fn draw_stats<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let stats_block = Block::default().title("Stats").borders(Borders::ALL);
    f.render_widget(stats_block, area);
}

fn draw_events<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Events").borders(Borders::ALL);
    f.render_widget(block, area);
}

// let mut engine = console_engine::ConsoleEngine::init(width, height, frame_rate).unwrap();

// // The screen where all the lifeforms appear
// let mut main_screen = Screen::new(size as u32, size as u32);
// let mut controls_screen = Screen::new((size * 3) as u32, 2);
// let mut stats_screen = Screen::new((size * 2) as u32, engine.get_height());
// let mut info_screen = Screen::new((size * 2) as u32, engine.get_height());

// // Controls
// controls_screen.print(
//     0,
//     0,
//     format!(
//         "controls: q = quit | p = pause | f = change frame rate | e = evolve without UI | frame {}",
//         engine.frame_count
//     )
//     .as_str(),
// );
// controls_screen.draw();

// let mut paused = false;

// // TODO ATTOW there's an error in console_engine that disallows any value over 1000 for
// // target_fps. If this becomes a real issue we can switch back to the normal way instead
// // of engine.poll() way. However it'd also be really nice to add an escape hatch to run
// // the evolution and not show anything on the screen.
// // This is documented here: https://github.com/VincentFoulon80/console_engine/issues/17
// loop {
//     // Poll next event
//     match engine.poll() {
//         // A frame has passed
//         Event::Frame => {
//             if !paused {
//                 engine.clear_screen();
//                 for screen in [
//                     &mut main_screen,
//                     &mut controls_screen,
//                     &mut stats_screen,
//                     &mut info_screen,
//                 ] {
//                     screen.clear();
//                 }

//                 world.step();

//                 // update_main_screen(&mut main_screen, &world);
//                 // update_stats_screen(&mut stats_screen, &world);

//                 // engine.print_screen(0, 0, &main_screen);
//                 // engine.print_screen(0, (engine.get_height() - 1) as i32, &controls_screen);
//                 // engine.print_screen((size + 2) as i32, 0, &stats_screen);
//             }
//         }

//         // A Key has been pressed
//         Event::Key(keyevent) => {
//             if keyevent.code == KeyCode::Char('q') {
//                 break;
//             }

//             if keyevent.code == KeyCode::Char('p') {
//                 paused = !paused;
//             }

//             if keyevent.code == KeyCode::Char('f') {
//                 todo!();
//             }

//             if keyevent.code == KeyCode::Char('e') {
//                 todo!();
//                 // TODO here could pause this loop, call a fn that has another
//                 // loop that just steps. In that fn though need to figure out
//                 // how to capture key events.
//                 // Alternatively, could have e mean do like 10,000 frames or something
//                 // without UI. So like a quick jump into the future.
//             }
//         }

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

//         // Window has been resized
//         Event::Resize(_w, _h) => { /* ... */ }
//     }
// }

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

// fn update_main_screen(screen: &mut Screen, world: &World) {
//     let mut num_at_location: HashMap<(usize, usize), usize> = HashMap::new();

//     for lf in world.lifeforms.values() {
//         *num_at_location.entry(lf.location).or_insert(0) += 1;
//         let num = num_at_location[&lf.location];

//         let char = match num {
//             1 if lf.health >= 0.5 => '☺',
//             1 if lf.health < 0.5 => '☹',
//             2 => '2',
//             3 => '3',
//             4 => '4',
//             5 => '5',
//             6 => '6',
//             7 => '7',
//             8 => '8',
//             9 => '9',
//             _ => '!',
//         };

//         let color = match lf.health {
//             _ if lf.health >= 0.9 => Color::Green,
//             _ if lf.health >= 0.8 => Color::DarkGreen,
//             _ if lf.health >= 0.7 => Color::Blue,
//             _ if lf.health >= 0.6 => Color::DarkBlue,
//             _ if lf.health >= 0.5 => Color::DarkMagenta,
//             _ if lf.health >= 0.4 => Color::Magenta,
//             _ if lf.health >= 0.3 => Color::DarkYellow,
//             _ if lf.health >= 0.2 => Color::Yellow,
//             _ if lf.health >= 0.1 => Color::DarkRed,
//             _ if lf.health < 0.1 => Color::Red,
//             _ => Color::White,
//         };

//         screen.set_pxl(
//             lf.location.0 as i32,
//             lf.location.1 as i32,
//             pixel::pxl_fg(char, color),
//         );
//     }

//     for water in &world.water {
//         screen.set_pxl(
//             water.0 as i32,
//             water.1 as i32,
//             pixel::pxl_fg('W', Color::Blue),
//         );
//     }

//     for food in &world.food {
//         screen.set_pxl(
//             food.0 as i32,
//             food.1 as i32,
//             pixel::pxl_fg('F', Color::Green),
//         );
//     }

//     for danger in &world.danger {
//         screen.set_pxl(
//             danger.0 as i32,
//             danger.1 as i32,
//             pixel::pxl_fg('☠', Color::Red),
//         );
//     }

//     screen.draw();
// }

// fn update_stats_screen(screen: &mut Screen, world: &World) {
//     let stats: Vec<(usize, usize, f32, f32, f32, (usize, usize))> = world
//         .lifeforms
//         .values()
//         .map(|lf| {
//             (
//                 lf.id,
//                 lf.lifespan,
//                 lf.health,
//                 lf.hunger,
//                 lf.thirst,
//                 lf.location,
//             )
//         })
//         .collect();

//     // Stats
//     screen.line(0, 0, 0, (screen.get_height() - 2) as i32, pixel::pxl('|'));
//     screen.print(1, 0, "Stats: id, lifespan, health, hunger, thirst");
//     for (idx, stat) in stats.iter().enumerate() {
//         screen.print(1, idx as i32, &format!("{:.10?}", stat));
//     }

//     screen.draw();
// }
