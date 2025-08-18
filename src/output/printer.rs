//! Handles outputting data to a target that implements the [`Write`] trait.

use std::io::{self, Write};

pub struct Printer<W: Write> {
    writer: W,
}

impl<W: Write> Printer<W> {
    /// Creates a new [`printer`] that writes to the given target.
    pub fn new(writer: W) -> Printer<W> {
        Printer { writer }
    }

    /// Writes text to the writer.
    pub fn print(&mut self, text: &str) -> io::Result<()> {
        self.writer.write_all(text.as_bytes())
    }

    /// Writes text to the writer followed by a new line.
    pub fn println(&mut self, text: &str) -> io::Result<()> {
        writeln!(self.writer, "{text}")
    }
}
