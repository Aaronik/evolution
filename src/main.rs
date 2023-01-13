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
    let size = 55;

    let world_props = WorldProps {
        size,
        num_initial_lifeforms: 10,
        genome_size: 45,
        mutation_rate: 0.001,
        food_density: 30,
        water_density: 30,
        num_inner_neurons: 3,
    };

    let mut world = World::new(world_props);
    let mut engine = console_engine::ConsoleEngine::init(size as u32, (size + 1) as u32, 1000).unwrap();

    // TODO ATTOW there's an error in console_engine that disallows any value over 1000 for
    // target_fps. If this becomes a real issue we can switch back to the normal way instead
    // of engine.poll() way. However it'd also be really nice to add an escape hatch to run
    // the evolution and not show anything on the screen.
    loop {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                step(&mut engine, &mut world);
            }

            // A Key has been pressed
            Event::Key(keyevent) => {
                if keyevent.code == KeyCode::Char('q') {
                    break;
                }
            }

            // Mouse has been moved or clicked
            Event::Mouse(_mouseevent) => { /* ... */ }

            // Window has been resized
            Event::Resize(_w, _h) => { /* ... */ }
        }
    }
}

fn step(engine: &mut ConsoleEngine, world: &mut World) {
    engine.clear_screen(); // reset the screen

    for lifeform in world.lifeforms.values() {
        engine.set_pxl(
            lifeform.location.0 as i32,
            lifeform.location.1 as i32,
            pixel::pxl_fg('O', Color::White),
        );
    }

    for water in &world.water {
        engine.set_pxl(
            water.0 as i32,
            water.1 as i32,
            pixel::pxl_fg('O', Color::Blue),
        );
    }

    for food in &world.food {
        engine.set_pxl(
            food.0 as i32,
            food.1 as i32,
            pixel::pxl_fg('O', Color::Green),
        );
    }

    for danger in &world.danger {
        engine.set_pxl(
            danger.0 as i32,
            danger.1 as i32,
            pixel::pxl_fg('O', Color::Red),
        );
    }

    engine.print(
        0,
        (engine.get_width() - 1) as i32,
        format!("controls: q = quit | frame {}", engine.frame_count).as_str(),
    );

    engine.draw(); // draw the screen

    world.step();
}
