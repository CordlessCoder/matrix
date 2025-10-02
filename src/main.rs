use crate::matrix::Matrix;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
mod line;
mod matrix;
mod terminal;
mod timer;

static STOP_REQUEST: AtomicBool = AtomicBool::new(false);

fn main() -> io::Result<()> {
    _ = ctrlc::set_handler(|| {
        STOP_REQUEST.store(true, Ordering::Release);
    });
    // Initialize the terminal
    let stdout = stdout().lock();
    let mut term = terminal::Terminal::new(stdout);
    term.make_raw()?;
    term.hide_cursor()?;
    term.enter_alternate()?;
    term.disable_wrapping()?;
    let (mut width, mut height) = crossterm::terminal::size()?;

    // Do not wait between frames if -b or --bench was provided as a command line argument
    let bench = std::env::args_os()
        .any(|arg| arg.as_encoded_bytes() == b"--bench" || arg.as_encoded_bytes() == b"-b");

    // Set up the frame timer
    let mut timer = if bench {
        timer::Timer::new(Duration::ZERO)
    } else {
        timer::Timer::from_framerate(30)
    };

    let mut matrix = Matrix::new();

    let mut rng = rand::rng();
    let start = Instant::now();

    'render: while !STOP_REQUEST.load(Ordering::Acquire) {
        matrix.update(&mut term, width, height)?;
        term.flush()?;

        let mut first_wait = true;
        while first_wait || !timer.left().is_zero() {
            first_wait = false;
            let event = terminal::try_read_event(timer.left().max(Duration::from_nanos(1)))?;
            let Some(event) = event else { break };
            match event {
                Event::Resize(nw, nh) => {
                    width = nw;
                    height = nh
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Esc | KeyCode::Char('q'),
                    ..
                }) => break 'render,
                _ => (),
            }
        }
        let lines_to_add = rng.random_range(1..=(width / 30).max(1));
        for _ in 0..lines_to_add {
            matrix.add_random_line(&mut rng, width);
        }
        timer.tick();
    }
    // Restore terminal
    term.reset()?;

    if bench {
        let took = start.elapsed();
        let frames = timer.ticks();
        let fps = frames as f64 / took.as_secs_f64();
        let cells = frames * width as u64 * height as u64;
        let cps = cells as f64 / took.as_secs_f64();
        eprintln!(
            "{frames} frames in {took:?}. {fps:.2}fps at {width}x{height}, {cps:.0} cells per second"
        );
    }

    Ok(())
}
