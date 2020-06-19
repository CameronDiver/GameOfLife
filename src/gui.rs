use crate::world::World;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::*;

pub struct WindowState {
    window: sdl2::video::Window,
    events: sdl2::EventPump,
    window_width: usize,
    window_height: usize,
}

pub enum Task {
    Nothing,
    Pause,
}

pub fn setup_window() -> WindowState {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game of Life", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    WindowState {
        window,
        events: event_pump,
        window_height: 600,
        window_width: 800,
    }
}

pub fn check_events(state: &mut WindowState) -> (bool, Task) {
    let mut exit = false;
    let mut task = Task::Nothing;
    for event in state.events.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => exit = true,
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                task = Task::Pause;
            }
            _ => (),
        }
    }
    (exit, task)
}

pub fn update_view(state: WindowState, world: &World) -> WindowState {
    // let mut surface = state.window.surface(&state.events).unwrap();
    // let mut canvas = Canvas::from_surface(surface);
    let mut canvas = state.window.into_canvas().build().unwrap();
    // Clear the canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let cell_w = state.window_width as i32 / world.width as i32;
    let cell_h = state.window_height as i32 / world.height as i32;

    // Draw the grid
    canvas.set_draw_color(Color::RGB(40, 40, 40));
    let mut draw_y: i32 = 0;
    let mut draw_x: i32 = 0;

    while draw_x < state.window_width as i32 {
        canvas
            .draw_line(
                Point::new(draw_x, 0),
                Point::new(draw_x, state.window_height as i32),
            )
            .expect("line");
        draw_x += cell_w
    }

    while draw_y < state.window_height as i32 {
        canvas
            .draw_line(
                Point::new(0, draw_y),
                Point::new(state.window_width as i32, draw_y),
            )
            .expect("line");
        draw_y += cell_h
    }

    let mut world_x;
    let mut world_y = 0;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    while world_y < world.height {
        world_x = 0;
        while world_x < world.width {
            if world.get_cell_at(world_x, world_y) {
                let draw_x = cell_w * world_x as i32;
                let draw_y = cell_h * world_y as i32;

                canvas
                    .fill_rect(Rect::new(draw_x, draw_y, cell_w as u32, cell_h as u32))
                    .unwrap();
            }
            world_x += 1;
        }
        world_y += 1;
    }

    canvas.present();

    WindowState {
        window: canvas.into_window(),
        ..state
    }
}
