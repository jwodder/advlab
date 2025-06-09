use std::io::{self, BufRead, Write};

pub trait InterfaceProvider {
    type Interface: Interface;

    fn with_interface<F>(self, func: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self::Interface) -> io::Result<()>;
}

pub trait Interface {
    fn show_output(&mut self, text: &str) -> io::Result<()>;

    // Returns None on end of input
    fn get_input(&mut self) -> io::Result<Option<String>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicInterface<R, W> {
    reader: R,
    writer: W,
    wrote_prompt: bool,
    wrote_last_output: bool,
}

impl<R, W> BasicInterface<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        BasicInterface {
            reader,
            writer,
            wrote_prompt: false,
            wrote_last_output: false,
        }
    }
}

impl<R: BufRead, W: Write> InterfaceProvider for BasicInterface<R, W> {
    type Interface = Self;

    fn with_interface<F>(mut self, func: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self::Interface) -> io::Result<()>,
    {
        func(&mut self)
    }
}

impl<R: BufRead, W: Write> Interface for BasicInterface<R, W> {
    fn show_output(&mut self, text: &str) -> io::Result<()> {
        if self.wrote_prompt {
            writeln!(&mut self.writer)?;
        }
        if !text.is_empty() {
            writeln!(&mut self.writer, "{text}")?;
            self.wrote_last_output = true;
        } else {
            self.wrote_last_output = false;
        }
        Ok(())
    }

    fn get_input(&mut self) -> io::Result<Option<String>> {
        if self.wrote_last_output {
            writeln!(&mut self.writer)?;
        }
        write!(&mut self.writer, "> ")?;
        self.writer.flush()?;
        self.wrote_prompt = true;
        let mut input = String::new();
        if self.reader.read_line(&mut input)? != 0 {
            Ok(Some(input))
        } else {
            // Force the start of a new line:
            writeln!(&mut self.writer)?;
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StandardInterface;

impl InterfaceProvider for StandardInterface {
    type Interface = BasicInterface<io::StdinLock<'static>, io::StdoutLock<'static>>;

    fn with_interface<F>(self, func: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self::Interface) -> io::Result<()>,
    {
        let mut iface = BasicInterface::new(io::stdin().lock(), io::stdout().lock());
        func(&mut iface)
    }
}
