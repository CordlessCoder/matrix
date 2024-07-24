use colorsys::{Hsl, Rgb};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
    SynchronizedUpdate,
};
use rand::Rng;
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
    let (mut width, mut height) = crossterm::terminal::size()?;

    let bench = false;
    let mut timer = if bench {
        timer::Timer::new(Duration::ZERO)
    } else {
        timer::Timer::from_framerate(30)
    };

    let mut matrix = Matrix::new();

    let mut rng = rand::thread_rng();
    let mut frames = 0;
    let start = Instant::now();

    loop {
        matrix.draw(&mut term, &mut rng, width, height)?;

        let event = terminal::try_read_event(timer.left())?;
        if let Some(event) = event {
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
                }) => break,
                _ => (),
            }
        }
        matrix.add_random_line(&mut rng, width);
        timer.skip();
        frames += 1;
    }
    std::mem::drop(term);
    let took = start.elapsed();
    let fps = frames as f64 / took.as_secs_f64();
    println!("{frames} frames in {took:?}. {fps}fps as {width}x{height}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x: u16,
    y: u16,
    length: u16,
    speed: u16,
    r: u8,
    g: u8,
    b: u8,
}

impl Line {
    pub fn step(&mut self) {
        self.y += self.speed;
    }
    pub fn in_bounds(&self, width: u16, height: u16) -> bool {
        self.x < width && self.y.saturating_sub(self.length) < height
    }
    pub fn draw(
        &mut self,
        mut writer: impl Write,
        mut rng: impl Rng,
        height: u16,
    ) -> io::Result<()> {
        let make_color = |brightness: f32| Color::Rgb {
            r: (self.r as f32 * brightness) as u8,
            g: (self.g as f32 * brightness) as u8,
            b: (self.b as f32 * brightness) as u8,
        };
        let top = self.y as i16 - self.length as i16;
        let mut chars = rng.sample_iter(rand::distributions::Alphanumeric);
        let points = (top..self.y.min(height) as i16)
            .enumerate()
            .skip(-top.min(0) as usize)
            .map(|(point, y)| (y as u16, ((point + 1) as f32) / self.length as f32));
        for (y, value) in points {
            queue!(
                writer,
                MoveTo(self.x, y),
                SetForegroundColor(make_color(value)),
            )?;
            _ = writer.write(&[chars.next().unwrap()])?;
        }

        Ok(())
    }
}
#[derive(Debug, Clone)]
struct Matrix {
    lines: Vec<Line>,
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
        let rgb: Rgb = Hsl::new(
            rng.gen_range(0.0..=360.),
            rng.gen_range(80.0..=100.),
            rng.gen_range(80.0..=100.),
            None,
        )
        .into();
        let [r, g, b]: [u8; 3] = rgb.into();
        self.add_line(Line {
            x: rng.gen_range(0..width),
            y: 0,
            length: rng.gen_range(1..=6),
            speed: rng.gen_range(1..=1),
            r,
            g,
            b,
        })
    }
    fn draw(
        &mut self,
        mut writer: impl Write,
        mut rng: impl Rng,
        width: u16,
        height: u16,
    ) -> io::Result<()> {
        writer.sync_update(|mut writer| -> io::Result<()> {
            queue!(writer, Clear(ClearType::All))?;
            #[allow(clippy::needless_borrows_for_generic_args)]
            self.lines.retain_mut(|line| {
                _ = line.draw(&mut writer, &mut rng, height);
                let keep = line.in_bounds(width, height);
                line.step();
                keep
            });
            Ok(())
        })?
    }
}
