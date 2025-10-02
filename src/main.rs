use colorsys::Hsl;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    terminal::{Clear, ClearType},
    SynchronizedUpdate,
};
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use crate::line::Line;

mod line;
mod terminal;
mod timer;

static STOP_REQUEST: AtomicBool = AtomicBool::new(false);

fn main() -> io::Result<()> {
    _ = ctrlc::set_handler(|| {
        STOP_REQUEST.store(true, Ordering::Release);
    });
    let stdout = stdout().lock();
    let mut term = terminal::Terminal::new(stdout);
    term.make_raw()?;
    term.hide_cursor()?;
    term.enter_alternate()?;
    term.disable_wrapping()?;
    let (mut width, mut height) = crossterm::terminal::size()?;

    let bench = std::env::args_os()
        .any(|arg| arg.as_encoded_bytes() == b"--bench" || arg.as_encoded_bytes() == b"-b");
    let mut timer = if bench {
        timer::Timer::new(Duration::ZERO)
    } else {
        timer::Timer::from_framerate(30)
    };

    let mut matrix = Matrix::new();

    let mut rng = rand::rng();
    let start = Instant::now();

    'render: while !STOP_REQUEST.load(Ordering::Acquire) {
        matrix.draw(&mut term, width, height)?;
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
        let lines = rng.random_range(1..=(width / 30).max(1));
        for _ in 0..lines {
            matrix.add_random_line(&mut rng, width);
        }
        timer.tick();
    }
    core::mem::drop(term);

    if bench {
        let frames = timer.ticks();
        let took = start.elapsed();
        let fps = frames as f64 / took.as_secs_f64();
        let cells = frames * width as u64 * height as u64;
        let cps = cells as f64 / took.as_secs_f64();
        eprintln!(
            "{frames} frames in {took:?}. {fps:.2}fps at {width}x{height}, {cps:.0} cells per second"
        );
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Matrix {
    lines: Vec<Line>,
    // ptr, len, cap
    // |
    // V
    // [Line, Line]
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            lines: Vec::with_capacity(256),
        }
    }
    fn add_line(&mut self, line: Line) {
        self.lines.push(line)
    }
    fn add_random_line(&mut self, mut rng: impl Rng, width: u16) {
        let length = rng.random_range(3..=15);
        self.add_line(Line {
            x: rng.random_range(0..width),
            y: -(length as f32),
            length,
            speed: rng.random_range(0.5..=1.5),
            seed: rng.random(),
        })
    }
    fn draw(&mut self, mut writer: impl Write, width: u16, height: u16) -> io::Result<()> {
        let color = Hsl::new(120.0, 100., 100., None);
        writer.sync_update(|mut writer| -> io::Result<()> {
            queue!(writer, Clear(ClearType::All))?;
            self.lines.retain_mut(|line| {
                _ = line.draw(&mut writer, height, color.clone());
                line.step();
                line.in_bounds(width, height)
            });
            Ok(())
        })?
    }
}
