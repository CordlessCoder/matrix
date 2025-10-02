use crate::line::Line;
use colorsys::Hsl;
use crossterm::{
    queue,
    terminal::{Clear, ClearType},
    SynchronizedUpdate,
};
use rand::Rng;
use std::io::{self, Write};

#[derive(Debug, Clone, Default)]
/// All the state that persists between frames.
pub struct Matrix {
    lines: Vec<Line>,
}

impl Matrix {
    pub const fn new() -> Self {
        Matrix { lines: Vec::new() }
    }
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line)
    }
    pub fn add_random_line(&mut self, rng: &mut impl Rng, width: u16) {
        self.add_line(Line::new(rng, width))
    }
    /// A function that draws a frame, advances all lines and cleans up any lines that go out of
    /// bounds of the screen.
    pub fn update(&mut self, mut writer: impl Write, width: u16, height: u16) -> io::Result<()> {
        let color = Hsl::new(120.0, 100., 100., None);

        // Only update the screen once
        writer.sync_update(|mut writer| -> io::Result<()> {
            queue!(writer, Clear(ClearType::All))?;

            // Only keep the lines that remain in bounds at the end of this opearation
            let mut res = Ok(());
            self.lines.retain_mut(|line| {
                if let Err(err) = line.draw(&mut writer, height, color.clone()) {
                    res = Err(err)
                }
                line.advance();
                line.in_bounds(width, height)
            });
            res
        })?
    }
}
