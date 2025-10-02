#![allow(unused)]
use colorsys::{Hsl, Rgb};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
    SynchronizedUpdate,
};
use rand::{Rng, SeedableRng};
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

mod terminal;
mod timer;

fn main() -> io::Result<()> {
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

    'render: loop {
        matrix.draw(&mut term, width, height)?;

        while !timer.left().is_zero() {
            let event = terminal::try_read_event(timer.left())?;
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
        println!("{frames} frames in {took:?}. {fps}fps at {width}x{height}");
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Line {
    x: u16,
    y: f32,
    length: f32,
    speed: f32,
    seed: u64,
}

impl Line {
    pub fn step(&mut self) {
        self.y += self.speed;
    }
    pub fn in_bounds(&self, width: u16, height: u16) -> bool {
        self.x < width && self.y - self.length < height as f32
    }
    pub fn draw(&mut self, mut writer: impl Write, height: u16, color: Hsl) -> io::Result<()> {
        let make_color = |brightness: f32| {
            let mut new = color.clone();
            new.set_lightness(brightness as f64 * 100.0);
            let rgb: Rgb = new.into();
            let [r, g, b] = rgb.into();
            Color::Rgb { r, g, b }
        };
        let top = self.y as i16 - self.length as i16;
        let rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
        let mut chars = rng.sample_iter(rand::distr::Alphanumeric);
        let points = (top..(self.y as u16).min(height) as i16)
            .enumerate()
            .skip(-top.min(0) as usize)
            .map(|(point, y)| (y as u16, ((point + 1) as f32) / self.length));
        for (y, value) in points {
            queue!(
                writer,
                MoveTo(self.x, y),
                SetForegroundColor(make_color(0.1 + (value * 0.7 * (self.speed / 1.5 * 0.8)))),
            )?;
            _ = writer.write(&[chars.next().unwrap()])?;
        }

        Ok(())
    }
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
        self.add_line(Line {
            x: rng.random_range(0..width),
            y: 0.0,
            length: rng.random_range(3.0..=15.0),
            speed: rng.random_range(0.5..=1.5),
            seed: rng.random(),
        })
    }
    fn draw(&mut self, mut writer: impl Write, width: u16, height: u16) -> io::Result<()> {
        let color = Hsl::new(120.0, 100., 100., None);
        writer.sync_update(|mut writer| -> io::Result<()> {
            queue!(writer, Clear(ClearType::All))?;
            #[allow(clippy::needless_borrows_for_generic_args)]
            self.lines.retain_mut(|line| {
                let mut color = color.clone();
                _ = line.draw(&mut writer, height, color);
                line.step();
                line.in_bounds(width, height)
            });
            Ok(())
        })?
    }
}
