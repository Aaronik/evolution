use std::collections::HashMap;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::*;

pub fn ui<B>(
    f: &mut Frame<B>,
    size: usize,
    world: &World,
    iteration: usize,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(20)].as_ref())
        .split(f.size());

    draw_main(f, size, selected_lf, tick_rate, iteration, world, chunks[0]);
    draw_controls(f, chunks[1]);
}

fn draw_main<B>(
    f: &mut Frame<B>,
    size: usize,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
    iteration: usize,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(10)].as_ref())
        .split(area);

    draw_world(f, size, selected_lf, world, chunks[0]);
    draw_right(f, selected_lf, tick_rate, iteration, world, chunks[1]);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let text = vec![Spans::from(
        "q = quit | p = pause | Up/Down = Select LifeForm | Left/Right = change tick rate",
    )];
    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn draw_world<B>(
    f: &mut Frame<B>,
    size: usize,
    selected_lf: Option<&LifeForm>,
    world: &World,
    area: Rect,
) where
    B: Backend,
{

    let world_canvas = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .x_bounds([0.0, size as f64])
        .y_bounds([0.0, size as f64])
        .paint(|ctx| {
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

            for heal in &world.heals {
                ctx.print(
                    heal.0 as f64,
                    heal.1 as f64,
                    Span::styled("♥", Style::default().fg(Color::Red)),
                );
            }

            let mut num_at_location: HashMap<(usize, usize), usize> = HashMap::new();

            for lf in world.lifeforms.values() {
                *num_at_location.entry(lf.location).or_insert(0) += 1;
                let num = num_at_location[&lf.location];

                // Ideations on how to print the lifeforms once they have a direction
                // Ô (circumplex), O̺ (combined inverted bridge below), Ό (with tonos), O҉ (cryllic millions sign), O҈ (cryllic hundred thousands sign)
                // Oՙ (armenian half ring), O֑ (hebre etnahta), O֒ ,O֓ , O֔ , O֕ , ֕O, O֟, O֚   , O֛   O֣
                // ↘҉  , ↗, ↙, ↖,
                // Use arrows with the "combining cryllic millions sign (U+0489)", found here: https://www.fileformat.info/info/charset/UTF-8/list.htm?start=1024
                // TRIANGLES: ▲, ◥, ▶, ◢, ▼, ◣, ◀, ◤,
                //
                // TRIANGLES: ▲҉, ◥҉, ▶҉, ◢҉, ▼҉, ◣҉, ◀҉, ◤҉,

                let single_lf_char = match lf.orientation.name() {
                    DirectionName::North => "▲",
                    DirectionName::NorthEast => "◥",
                    DirectionName::East => "▶",
                    DirectionName::SouthEast => "◢",
                    DirectionName::South => "▼",
                    DirectionName::SouthWest => "◣",
                    DirectionName::West => "◀",
                    DirectionName::NorthWest => "◤",
                };

                let char = match num {
                    1 => single_lf_char,
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

                if let Some(selected_lf) = selected_lf {
                    if lf.id == selected_lf.id {
                        ctx.print(
                            lf.location.0 as f64,
                            lf.location.1 as f64,
                            Span::styled(char, Style::default().fg(Color::White)),
                        );
                    } else {
                        ctx.print(
                            lf.location.0 as f64,
                            lf.location.1 as f64,
                            Span::styled(char, Style::default().fg(color)),
                        );
                    }
                } else {
                    ctx.print(
                        lf.location.0 as f64,
                        lf.location.1 as f64,
                        Span::styled(char, Style::default().fg(color)),
                    );
                }
            }

            for danger in &world.danger {
                ctx.print(
                    danger.0 as f64,
                    danger.1 as f64,
                    Span::styled(
                        "☠ ",
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
            }
        });

    f.render_widget(world_canvas, area);
}

fn draw_right<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
    iteration: usize,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_top_right(f, tick_rate, iteration, world, chunks[0]);
    draw_single_lf_information(f, selected_lf, world, chunks[1]);
}

// TODO UI update, top right panel
// Select
// Graphs of health, thirst, hunger
// Input neuron values
// Output neuron values
// Reach: I'd LOVE to see the neural net somehow

fn draw_single_lf_information<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Min(17),
                Constraint::Min(35),
                Constraint::Min(35),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(area);

    draw_lf_selection(f, selected_lf, world, chunks[0]);
    draw_lf_input_neuron_values(f, selected_lf, chunks[1]);
    draw_lf_output_neuron_values(f, selected_lf, chunks[2]);
    draw_lf_neural_net(f, selected_lf, chunks[3]);
}

fn draw_lf_selection<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, world: &World, area: Rect)
where
    B: Backend,
{
    let items: Vec<ListItem> = world
        .lifeforms
        .values()
        .map(|lf| {
            if let Some(selected_lf) = selected_lf {
                if lf.id == selected_lf.id {
                    ListItem::new(format!("=> {}", lf.id)).style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(format!("{}", lf.id))
                }
            } else {
                ListItem::new(format!("{}", lf.id))
            }
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Select LifeForm")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_input_neuron_values<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    area: Rect,
) where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let mut items: Vec<ListItem> = vec![];

    for (neuron_type, neuron) in selected_lf.unwrap().neural_net.input_neurons.values() {
        items.push(ListItem::new(format!(
            "{:?}: {:?}",
            neuron_type, neuron.value
        )));
    }

    let list = List::new(items).block(
        Block::default()
            .title(format!(
                "Input Neuron Values for {}",
                selected_lf.unwrap().id
            ))
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_output_neuron_values<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    area: Rect,
) where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let values: &Vec<(OutputNeuronType, f32)>;

    if let None = selected_lf.unwrap().most_recent_output_neuron_values {
        return;
    } else {
        values = selected_lf
            .unwrap()
            .most_recent_output_neuron_values
            .as_ref()
            .unwrap();
    }

    let mut items: Vec<ListItem> = vec![];

    for (neuron_type, value) in values.iter() {
        items.push(ListItem::new(format!("{:?}: {}", neuron_type, value)));
    }

    let list = List::new(items).block(
        Block::default()
            .title("Output Neuron Values")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_neural_net<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    area: Rect,
) where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    // TODO This is gonna be friggin awesome
    let neural_net_canvas = Canvas::default()
        .block(Block::default().title("Neural Net").borders(Borders::ALL))
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            ctx.print(
                0.0 as f64,
                selected_lf.unwrap().lifespan as f64 / 10.0,
                Span::styled(
                    format!("TODO LF {} Neural Net", selected_lf.unwrap().id),
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        });

    f.render_widget(neural_net_canvas, area);
}

fn draw_top_right<B>(f: &mut Frame<B>, tick_rate: u64, iteration: usize, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    draw_world_information(f, tick_rate, iteration, world, chunks[0]);
    draw_events(f, world, chunks[1]);
}

fn draw_world_information<B>(
    f: &mut Frame<B>,
    tick_rate: u64,
    iteration: usize,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    // TODO
    // * Dude the chart could be an amazing visualization for this, allowing us to see things
    // like average age over time
    // * Get oldest LF

    let block = Block::default()
        .borders(Borders::ALL)
        .title("World Information");

    let mut items: Vec<ListItem> = vec![];

    items.push(
        ListItem::new(format!(
            "Info: tick rate: {}ms | iteration: {}",
            tick_rate, iteration
        ))
        .style(Style::default().fg(Color::Cyan)),
    );

    items.push(
        ListItem::new(format!("LifeForms: {}", world.lifeforms.len()))
            .style(Style::default().fg(Color::Green)),
    );

    let average_age: f32 = world
        .lifeforms
        .values()
        .map(|lf| lf.lifespan as f32)
        .sum::<f32>()
        / world.lifeforms.len() as f32;

    items.push(
        ListItem::new(format!("Avergae Age: {}", average_age))
            .style(Style::default().fg(Color::Green)),
    );

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn draw_events<B>(f: &mut Frame<B>, world: &World, area: Rect)
where
    B: Backend,
{
    let mut items: Vec<ListItem> = vec![];

    for (event_type, description) in &world.events {
        let color = match event_type {
            EventType::Death => Color::Blue,
            EventType::Creation => Color::Cyan,
            EventType::Mate => Color::Magenta,
            EventType::Attack => Color::Red,
            EventType::AsexuallyReproduce => Color::LightGreen,
        };

        items.insert(
            0,
            ListItem::new(Span::from(Span::styled(
                description,
                Style::default().fg(color),
            ))),
        );
    }

    let list = List::new(items).block(Block::default().title("Events").borders(Borders::ALL));

    f.render_widget(list, area);
}
