use console_engine::crossterm::event::MouseEventKind;
use console_engine::events::Event;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use evolution::*;

// TODO
// * Make output neuron effects
// * Add physics for when lifeforms get down to so many, they auto reproduce

fn main() {
    let size = 50;
    let frame_rate = 100;

    let world_props = WorldProps {
        size,
        num_initial_lifeforms: 20,
        genome_size: 20,
        mutation_rate: 0.001,
        food_density: 300,
        water_density: 30,
        num_inner_neurons: 3,
        minimum_number_lifeforms: 15,
        // TODO Add num dangers
    };

    let mut world = World::new(world_props);

    // println!("lifeforms: {:#?}", world.lifeforms.values().map(|lf| &lf.genome).collect::<Vec<&Genome>>());
    world.step();
}

//     let mut engine =
//         console_engine::ConsoleEngine::init((size * 3) as u32, (size + 2) as u32, frame_rate)
//             .unwrap();

//     let mut paused = false;

//     // TODO ATTOW there's an error in console_engine that disallows any value over 1000 for
//     // target_fps. If this becomes a real issue we can switch back to the normal way instead
//     // of engine.poll() way. However it'd also be really nice to add an escape hatch to run
//     // the evolution and not show anything on the screen.
//     loop {
//         // Poll next event
//         match engine.poll() {
//             // A frame has passed
//             Event::Frame => {
//                 if !paused {
//                     step(size, &mut engine, &mut world);
//                 }
//             }

//             // A Key has been pressed
//             Event::Key(keyevent) => {
//                 if keyevent.code == KeyCode::Char('q') {
//                     break;
//                 }

//                 if keyevent.code == KeyCode::Char('p') {
//                     paused = !paused;
//                 }

//                 if keyevent.code == KeyCode::Char('f') {
//                     todo!();
//                 }

//                 if keyevent.code == KeyCode::Char('e') {
//                     todo!();
//                     // TODO here could pause this loop, call a fn that has another
//                     // loop that just steps. In that fn though need to figure out
//                     // how to capture key events.
//                     // Alternatively, could have e mean do like 10,000 frames or something
//                     // without UI. So like a quick jump into the future.
//                 }
//             }

//             // Mouse has been moved or clicked
//             Event::Mouse(mouseevent) => {
//                 if let MouseEventKind::Down(_) = mouseevent.kind {
//                     paused = true;
//                     let loc = (mouseevent.column as usize, mouseevent.row as usize);
//                     let lf = world.lifeform_at_location(&loc);
//                     if let Some(lf) = lf {
//                         engine.print(
//                             (size + 2) as i32,
//                             ((size / 2) - 2) as i32,
//                             &format!("LifeForm: {}", lf.id),
//                         );
//                         // engine.print((size + 2) as i32, ((size / 2) - 1) as i32, &format!("Input genes:"));
//                         for (idx, (start_id, genes)) in lf.genome.seed.iter().enumerate() {
//                             engine.print(
//                                 (size + 2) as i32,
//                                 ((size / 2) + idx) as i32,
//                                 &format!("{} gene(s) for {}: {:?}", genes.len(), start_id, genes),
//                             );
//                         }
//                         // engine.print((size + 2) as i32, (size / 2) as i32, &format!("{:#?}", lf));
//                         engine.draw();
//                     }
//                 }
//             }

//             // Window has been resized
//             Event::Resize(_w, _h) => { /* ... */ }
//         }
//     }
// }

// fn step(size: usize, engine: &mut ConsoleEngine, world: &mut World) {
//     engine.clear_screen(); // reset the screen

//     world.step();

//     for lifeform in world.lifeforms.values() {
//         engine.set_pxl(
//             lifeform.location.0 as i32,
//             lifeform.location.1 as i32,
//             pixel::pxl_fg('O', Color::White),
//         );
//     }

//     for water in &world.water {
//         engine.set_pxl(
//             water.0 as i32,
//             water.1 as i32,
//             pixel::pxl_fg('O', Color::Blue),
//         );
//     }

//     for food in &world.food {
//         engine.set_pxl(
//             food.0 as i32,
//             food.1 as i32,
//             pixel::pxl_fg('O', Color::Green),
//         );
//     }

//     for danger in &world.danger {
//         engine.set_pxl(
//             danger.0 as i32,
//             danger.1 as i32,
//             pixel::pxl_fg('O', Color::Red),
//         );
//     }

//     // Controls
//     engine.print(
//         0,
//         (engine.get_height() - 1) as i32,
//         format!(
//             "controls: q = quit | p = pause | f = change frame rate | e = evolve without UI | frame {}",
//             engine.frame_count
//         )
//         .as_str(),
//     );

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

//     // let stats = format!("{:#?}", stats);

//     // Stats
//     engine.line(
//         (size + 1) as i32,
//         0,
//         (size + 1) as i32,
//         (engine.get_height() - 2) as i32,
//         pixel::pxl('|'),
//     );
//     engine.print(
//         (size + 2) as i32,
//         0,
//         "Stats: id, lifespan, health, hunger, thirst",
//     );
//     for (idx, stat) in stats.iter().enumerate() {
//         engine.print(
//             (size + 2) as i32,
//             (idx + 1) as i32,
//             &format!("{:.10?}", stat),
//         );
//     }
//     // engine.print((size + 2) as i32, 1, &stats);

//     engine.draw(); // draw the screen
// }
