use std::io::{self, Write};

pub struct Printer<W: Write> {
    writer: W,
}

impl<W: Write> Printer<W> {
    pub fn new(writer: W) -> Printer<W> {
        Printer { writer }
    }

    pub fn print(&mut self, text: &str) -> io::Result<()> {
        self.writer.write_all(text.as_bytes())
    }

    pub fn println(&mut self, text: &str) -> io::Result<()> {
        writeln!(self.writer, "{text}")
    }
}
