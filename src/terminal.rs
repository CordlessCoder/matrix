use crossterm::{terminal as term, QueueableCommand};
use std::{
    io::{self, Write},
    ops::{Deref, DerefMut},
    time::Duration,
};

use crossterm::event::{self, Event};

pub fn try_read_event(timeout: Duration) -> io::Result<Option<Event>> {
    if event::poll(timeout)? {
        return event::read().map(Some);
    }
    Ok(None)
}

pub struct Terminal<W: Write> {
    writer: W,
    raw: bool,
    alternate: bool,
    cursor_hidden: bool,
    wrapping: bool,
}

impl<W: Write> Drop for Terminal<W> {
    fn drop(&mut self) {
        _ = self.enable_wrapping();
        _ = self.show_cursor();
        _ = self.leave_alternate();
        _ = self.make_cooked();
    }
}

impl<W: Write> DerefMut for Terminal<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

impl<W: Write> Deref for Terminal<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl<W: Write> Write for Terminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }
}

impl<W: Write> Terminal<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            raw: false,
            alternate: false,
            cursor_hidden: false,
            wrapping: true,
        }
    }
    pub fn enter_alternate(&mut self) -> io::Result<()> {
        if self.alternate {
            return Ok(());
        }
        self.writer.queue(term::EnterAlternateScreen)?;
        self.alternate = true;
        Ok(())
    }
    pub fn leave_alternate(&mut self) -> io::Result<()> {
        if !self.alternate {
            return Ok(());
        }
        self.writer.queue(term::LeaveAlternateScreen)?;
        self.alternate = false;
        Ok(())
    }
    pub fn make_raw_nontty(&mut self) -> io::Result<()> {
        if self.raw {
            return Ok(());
        }
        term::enable_raw_mode()?;
        self.raw = true;
        Ok(())
    }
    pub fn make_cooked(&mut self) -> io::Result<()> {
        if !self.raw {
            return Ok(());
        }
        term::disable_raw_mode()?;
        self.raw = false;
        Ok(())
    }
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        if self.cursor_hidden {
            return Ok(());
        }
        self.writer.queue(crossterm::cursor::Hide)?;
        self.cursor_hidden = true;
        Ok(())
    }
    pub fn show_cursor(&mut self) -> io::Result<()> {
        if !self.cursor_hidden {
            return Ok(());
        }
        self.writer.queue(crossterm::cursor::Show)?;
        self.cursor_hidden = false;
        Ok(())
    }
    pub fn disable_wrapping(&mut self) -> io::Result<()> {
        if !self.wrapping {
            return Ok(());
        }
        self.writer.queue(term::DisableLineWrap)?;
        self.wrapping = false;
        Ok(())
    }
    pub fn enable_wrapping(&mut self) -> io::Result<()> {
        if self.wrapping {
            return Ok(());
        }
        self.writer.queue(term::EnableLineWrap)?;
        self.wrapping = true;
        Ok(())
    }
}
impl<W: Write + crossterm::tty::IsTty> Terminal<W> {
    pub fn make_raw(&mut self) -> io::Result<()> {
        let tty = self.writer.is_tty();
        if tty {
            term::enable_raw_mode()?;
        };
        self.raw = true;
        Ok(())
    }
}
