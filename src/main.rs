mod gui;
mod world;

use std::{thread, time};

fn main() {
    let mut v: world::World = world::World::new(60, 50);
    v.set_cell_at(10, 10, true);
    v.set_cell_at(11, 10, true);
    v.set_cell_at(12, 10, true);
    v.set_cell_at(12, 9, true);
    v.set_cell_at(11, 8, true);

    v.set_cell_at(41, 21, true);
    v.set_cell_at(42, 21, true);
    v.set_cell_at(42, 22, true);
    v.set_cell_at(41, 22, true);

    let mut state = gui::setup_window();
    let mut paused = true;
    loop {
        let (exit, task) = gui::check_events(&mut state);
        if exit {
            break;
        }
        match task {
            gui::Task::Pause => {
                paused = !paused;
            }
            _ => {}
        }
        if paused {
            state = gui::update_view(state, &v);
            v.tick();
        }
        thread::sleep(time::Duration::from_millis(50))
    }
}
