use std::collections::HashMap;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, List, ListItem, Paragraph,
    },
    Frame,
};

use crate::*;

pub fn ui<B>(
    f: &mut Frame<B>,
    size: usize,
    world: &World,
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

    draw_main(f, size, selected_lf, tick_rate, world, chunks[0]);
    draw_controls(f, chunks[1]);
}

fn draw_main<B>(
    f: &mut Frame<B>,
    size: usize,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
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
    draw_right(f, selected_lf, tick_rate, world, chunks[1]);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let text = vec![Spans::from(
        "q = quit | p = pause | d = pause drawing | Up/Down = Select LifeForm | Left/Right = change tick rate",
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
            for food in &world.food {
                ctx.print(
                    food.0 as f64,
                    food.1 as f64,
                    Span::styled("F", Style::default().fg(Color::Green)),
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
                    2 => "➋",
                    3 => "➌",
                    4 => "➍",
                    5 => "➎",
                    6 => "➏",
                    7 => "➐",
                    8 => "➑",
                    9 => "➒",
                    _ => "#",
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

                let mut style = Style::default().fg(color);

                if let Some(selected_lf) = selected_lf {
                    if lf.id == selected_lf.id {
                        style = style.fg(Color::White);
                    }
                }

                ctx.print(
                    lf.location.0 as f64,
                    lf.location.1 as f64,
                    Span::styled(char, style),
                );

            }

            ctx.print(
                world.danger.0 as f64,
                world.danger.1 as f64,
                Span::styled(
                    "☢",
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        });

    f.render_widget(world_canvas, area);
}

fn draw_right<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
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

    draw_top_right(f, selected_lf, tick_rate, world, chunks[0]);
    draw_single_lf_information(f, selected_lf, chunks[1]);
}

fn draw_single_lf_information<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
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

    draw_lf_stats(f, selected_lf, chunks[0]);
    draw_lf_input_neuron_values(f, selected_lf, chunks[1]);
    draw_lf_output_neuron_values(f, selected_lf, chunks[2]);
    draw_lf_neural_net(f, selected_lf, chunks[3]);
}

fn draw_lf_stats<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
    B: Backend,
{

    if let None = selected_lf {
        return;
    }

    let lf = selected_lf.unwrap();

    let mut items: Vec<ListItem> = vec![];

    // for (neuron_type, neuron) in selected_lf.unwrap().neural_net.input_neurons.values() {
    //     items.push(ListItem::new(format!(
    //         "{:?}: {:?}",
    //         neuron_type, neuron.value
    //     )));
    // }

    items.push(ListItem::new("Health:"));
    items.push(ListItem::new(lf.health.to_string()));

    items.push(ListItem::new("Hunger:"));
    items.push(ListItem::new(lf.hunger.to_string()));

    let list = List::new(items).block(
        Block::default()
            .title(format!(
                "LF {} Stats",
                selected_lf.unwrap().id
            ))
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_selection<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, world: &World, area: Rect)
where
    B: Backend,
{
    let height = area.height as usize;
    let column_width = 5.0;

    // TODO There's a subtle bug here where certain ids aren't showing up, although they still get
    // selected over. I believe there's overlap for some reason. Some id is being painted over
    // another. I checked the columns/rows and they do seem to be integer values (although with a
    // .0) after them.
    let canv = Canvas::default()
        .block(
            Block::default()
                .title("Select Lifeform")
                .borders(Borders::ALL),
        )
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            for (idx, lf) in world.lifeforms.values().enumerate() {
                let column = (idx / height) as f64 * column_width;
                let row = (height - (idx % height)) as f64 - 1.0;

                let mut style = Style::default();

                let txt = lf.id.to_string();

                if let Some(selected_lf) = selected_lf {
                    if lf.id == selected_lf.id {
                        style = style.fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD);
                    }
                }

                let span = Span::from(Span::styled(txt, style));

                ctx.print(column, row, span);
            }
        });

    f.render_widget(canv, area);
}

fn draw_lf_input_neuron_values<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
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

fn draw_lf_output_neuron_values<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
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

fn draw_lf_neural_net<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let lf = selected_lf.unwrap();

    // TODO For self referencing arrow, could use ↺. Can change font size?

    // TODO Get this outside of here and reassign to it instead of recreating a new one each time
    let neuron_locs = generate_neuron_hashmap(&lf.neural_net, &area);

    // Then for each genome, draw a line from each gene.from to gene.to
    // If it's a self reference... need a loop arrow, or just 3/4 or 4/5 of a circle
    let neural_net_canvas = Canvas::default()
        .block(Block::default().title("Neural Net").borders(Borders::ALL))
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            for gene in &lf.genome.genes {
                let (name, from) = &neuron_locs[&gene.from];
                let to = neuron_locs[&gene.to].1;
                ctx.draw(&Line {
                    x1: from.0 + (name.len() / 2) as f64,
                    y1: from.1,
                    x2: to.0,
                    y2: to.1,
                    color: Color::Rgb(((gene.weight + 4.0) * 31.0) as u8, ((gene.weight + 4.0) * 31.0) as u8, ((gene.weight + 4.0) * 31.0) as u8),
                });
                ctx.layer();
            }

            for (name, loc) in neuron_locs.values() {
                ctx.print(loc.0, loc.1, Span::from(Span::styled(String::from(name), Style::default().fg(Color::White))));
            }
        });

    f.render_widget(neural_net_canvas, area);
}

fn draw_top_right<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    draw_world_and_selection(f, selected_lf, tick_rate, world, chunks[0]);
    draw_events(f, world, chunks[1]);
}

fn draw_world_and_selection<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
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

    draw_world_information(f, tick_rate, world, chunks[0]);
    draw_lf_selection(f, selected_lf, world, chunks[1]);
}

fn draw_world_information<B>(f: &mut Frame<B>, tick_rate: u64, world: &World, area: Rect)
where
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
            "Info: iteration: {} | tick rate: {}ms",
            world.tics, tick_rate
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

/// Construct a hashmap of neuron_id => neuron location, used for drawing the neural net
fn generate_neuron_hashmap(
    neural_net: &NeuralNet,
    area: &Rect,
) -> HashMap<usize, (String, (f64, f64))> {
    let max_names_per_line = 3 as u16;

    // TODO Having a static max per line is alright, but it'd be nicer if it measured the length of
    // the neuron names, choosing dynamically.

    let input_neuron_spacing = area.width as f64 / (max_names_per_line) as f64;
    let inner_neuron_spacing = area.width as f64 / (neural_net.inner_neurons.len() + 1) as f64;
    let output_neuron_spacing = area.width as f64 / (neural_net.output_neurons.len() + 1) as f64;

    let output_neuron_row = 1;
    let inner_neuron_row = (area.height / 2) - 2;
    let input_neuron_row = area.height - 1;

    let mut neuron_location_map = HashMap::new();

    for (idx, (neuron_type, neuron)) in neural_net.input_neurons.values().enumerate() {
        let row = input_neuron_row - ((idx as u16 / max_names_per_line) * 2);

        neuron_location_map.insert(
            neuron.id,
            (
                format!("{}", neuron_type),
                (
                    ((idx as u16 + 1) % max_names_per_line) as f64 * input_neuron_spacing,
                    row as f64,
                ),
            ),
        );
    }

    for (idx, neuron) in neural_net.inner_neurons.values().enumerate() {
        neuron_location_map.insert(
            neuron.id,
            (
                String::from("⬤"),
                (
                    (idx + 1) as f64 * inner_neuron_spacing,
                    inner_neuron_row as f64,
                ),
            ),
        );
    }

    for (idx, (neuron_type, neuron)) in neural_net.output_neurons.values().enumerate() {
        neuron_location_map.insert(
            neuron.id,
            (
                format!("{}", neuron_type),
                (
                    (idx + 1) as f64 * output_neuron_spacing,
                    output_neuron_row as f64,
                ),
            ),
        );
    }

    neuron_location_map
}
