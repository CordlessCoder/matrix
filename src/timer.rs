use std::time::{Duration, Instant};

pub struct Timer {
    tick_count: u64,
    last_tick: Instant,
    pub interval: Duration,
}

impl Timer {
    /// Create a new timer, ticking every count of the provided interval
    pub fn new(interval: Duration) -> Self {
        Timer {
            last_tick: Instant::now(),
            interval,
            tick_count: 0,
        }
    }
    /// Create a new timer, computing the interval based on a framerate
    pub fn from_framerate(framerate: u64) -> Self {
        Timer::new(Duration::from_nanos(1_000_000_000 / framerate))
    }

    /// Number of .tick() calls since this Timer was created.
    pub fn ticks(&self) -> u64 {
        self.tick_count
    }
    pub fn tick(&mut self) {
        std::thread::sleep(self.left());
        self.skip();
    }
    pub fn skip(&mut self) {
        self.tick_count += 1;
        self.last_tick = Instant::now();
    }
    /// Return the amount of time until the next tick
    pub fn left(&self) -> Duration {
        let took = Instant::now().saturating_duration_since(self.last_tick);
        self.interval.saturating_sub(took)
    }
}
