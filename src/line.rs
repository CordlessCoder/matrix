use colorsys::{Hsl, Rgb};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, SetForegroundColor},
};
use rand::{Rng, SeedableRng};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Line {
    pub x: u16,
    pub y: f32,
    pub length: u16,
    pub speed: f32,
    pub seed: u64,
}

impl Line {
    pub fn step(&mut self) {
        self.y += self.speed;
    }
    pub fn in_bounds(&self, width: u16, height: u16) -> bool {
        self.x < width && self.y < height as f32
    }
    pub fn draw(&mut self, mut writer: impl Write, height: u16, color: Hsl) -> io::Result<()> {
        let with_brightness = |brightness: f32| {
            let mut new = color.clone();
            new.set_lightness(brightness as f64 * 100.0);
            let rgb: Rgb = new.into();
            let [r, g, b] = rgb.into();
            Color::Rgb { r, g, b }
        };
        let rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
        let mut chars = rng.sample_iter(rand::distr::Alphanumeric);
        let points = (self.y as i32..)
            .take(self.length as usize)
            .take_while(|&y| y < height as i32)
            .enumerate()
            .map(|(point, y)| (y, ((point + 1) as f32) / self.length as f32));
        for (y, value) in points {
            let c = chars.next().unwrap();
            let Ok(y) = y.try_into() else { continue };
            queue!(
                writer,
                MoveTo(self.x, y),
                SetForegroundColor(with_brightness(
                    0.1 + (value * 0.7 * (self.speed / 1.5 * 0.8))
                )),
            )?;
            _ = writer.write(&[c])?;
        }

        Ok(())
    }
}
