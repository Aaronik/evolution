use std::collections::HashMap;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crate::{World, EventType, LifeForm};


pub fn ui<B>(
    f: &mut Frame<B>,
    size: usize,
    world: &World,
    iteration: usize,
    selected_lf_index: i32,
    saved_tick_rate: u64,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(20)].as_ref())
        .split(f.size());

    draw_main(f, size, selected_lf_index, world, chunks[0]);
    draw_controls(f, chunks[1], iteration, saved_tick_rate);
}

fn draw_main<B>(f: &mut Frame<B>, size: usize, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(10)].as_ref())
        .split(area);

    draw_world(f, size, selected_lf_index, world, chunks[0]);
    draw_right(f, selected_lf_index, world, chunks[1]);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect, iteration: usize, saved_tick_rate: u64)
where
    B: Backend,
{
    let block = Block::default().title("Info").borders(Borders::ALL);
    let mut text = vec![Spans::from("Controls: q = quit | p = pause | Up/Down = Select LifeForm | Left/Right = change tick rate")];
    text.push(Spans::from(format!("Info: tick rate: {}ms | iteration: {}", saved_tick_rate, iteration)));
    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn draw_world<B>(f: &mut Frame<B>, size: usize, selected_lf_index: i32, world: &World, area: Rect)
where
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

            for (idx, lf) in world.lifeforms.values().enumerate() {
                *num_at_location.entry(lf.location).or_insert(0) += 1;
                let num = num_at_location[&lf.location];

                let char = match num {
                    1 if lf.health >= 0.5 => "☺ ",
                    1 if lf.health < 0.5 => "☹ ",
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

                if idx == selected_lf_index as usize {
                    ctx.print(
                        lf.location.0 as f64,
                        lf.location.1 as f64,
                        Span::styled(
                            char,
                            Style::default()
                                .fg(Color::White)
                        ),
                    );
                } else {
                    ctx.print(
                        lf.location.0 as f64,
                        lf.location.1 as f64,
                        Span::styled(
                            char,
                            Style::default().fg(color),
                        ),
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

fn draw_right<B>(f: &mut Frame<B>, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_stats(f, selected_lf_index, world, chunks[0]);
    draw_bottom_right(f, selected_lf_index, world, chunks[1]);
}

fn draw_stats<B>(f: &mut Frame<B>, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let header = Row::new(["id", "lifespan", "health", "hunger", "thirst", "location"])
        .height(1)
        .bottom_margin(1);

    let rows = world.lifeforms.values().enumerate().map(|(idx, lf)| {
        let cells = vec![
            lf.id.to_string(),
            lf.lifespan.to_string(),
            lf.health.to_string(),
            lf.hunger.to_string(),
            lf.thirst.to_string(),
            format!("({}, {})", lf.location.0, lf.location.1),
        ];

        if idx == (selected_lf_index as usize) {
            return Row::new(cells).style(Style::default().add_modifier(Modifier::BOLD));
        } else {
            return Row::new(cells).style(Style::default());
        }
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

fn draw_bottom_right<B>(f: &mut Frame<B>, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(65),
                Constraint::Min(17),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(area);

    draw_events(f, world, chunks[0]);
    draw_lf_selection(f, selected_lf_index, world, chunks[1]);
    draw_lf_info(f, selected_lf_index, world, chunks[2]);
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

fn draw_lf_selection<B>(f: &mut Frame<B>, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let items: Vec<ListItem> = world
        .lifeforms
        .values()
        .enumerate()
        .map(|(idx, lf)| {
            if idx == selected_lf_index as usize {
                ListItem::new(format!("=> {}", lf.id)).style(
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
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

fn draw_lf_info<B>(f: &mut Frame<B>, selected_lf_index: i32, world: &World, area: Rect)
where
    B: Backend,
{
    let mut items: Vec<ListItem> = vec![];

    let lf_opt = world.lifeforms.values().nth(selected_lf_index as usize);
    let lf: &LifeForm;

    if let None = lf_opt {
        return;
    } else {
        lf = lf_opt.unwrap();
    }

    for (neuron_type, neuron) in lf.neural_net.input_neurons.values() {
        items.push(ListItem::new(format!(
            "{:?}: {:?}",
            neuron_type, neuron.value
        )));
    }

    // let probabilities = lf.run_neural_net(&nnh);
    // for (idx, (neuron_type, prob)) in probabilities.iter().enumerate() {
    //     items.push(ListItem::new(format!("{:?}: {}", neuron_type, prob)));
    // }

    let list = List::new(items).block(
        Block::default()
            .title("Input Neuron Values")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}
