use rustyline::{DefaultEditor, error::ReadlineError};
use std::io::{self, BufRead, Write};

pub trait InterfaceProvider {
    type Interface: Interface;

    /// Run `func` with the associated `Interface`.
    fn with_interface<F>(self, func: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self::Interface) -> io::Result<()>;
}

pub trait Interface {
    /// Display the given text in the interface.
    fn show_output(&mut self, text: &str) -> io::Result<()>;

    /// Read a line of input from the interface.
    ///
    /// Returns `None` on end of input.
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadlineInterface;

impl InterfaceProvider for ReadlineInterface {
    type Interface = ReadlineInterfaceImpl;

    fn with_interface<F>(self, func: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self::Interface) -> io::Result<()>,
    {
        let mut iface = ReadlineInterfaceImpl::new()?;
        func(&mut iface)
    }
}

#[derive(Debug)]
pub struct ReadlineInterfaceImpl {
    rl: DefaultEditor,
    stdout: io::StdoutLock<'static>,
    wrote_prompt: bool,
    wrote_last_output: bool,
}

impl ReadlineInterfaceImpl {
    fn new() -> io::Result<Self> {
        let cfg = rustyline::config::Builder::new()
            .auto_add_history(true)
            .build();
        let rl = match DefaultEditor::with_config(cfg) {
            Ok(rl) => rl,
            Err(ReadlineError::Io(e)) => return Err(e),
            #[cfg(unix)]
            Err(ReadlineError::Errno(e)) => return Err(e.into()),
            Err(e) => return Err(io::Error::other(e)),
        };
        let stdout = io::stdout().lock();
        Ok(ReadlineInterfaceImpl {
            rl,
            stdout,
            wrote_prompt: false,
            wrote_last_output: true,
        })
    }
}

impl Interface for ReadlineInterfaceImpl {
    fn show_output(&mut self, text: &str) -> io::Result<()> {
        if self.wrote_prompt {
            writeln!(&mut self.stdout)?;
        }
        if !text.is_empty() {
            writeln!(&mut self.stdout, "{text}")?;
            self.wrote_last_output = true;
        } else {
            self.wrote_last_output = false;
        }
        Ok(())
    }

    fn get_input(&mut self) -> io::Result<Option<String>> {
        if self.wrote_last_output {
            writeln!(&mut self.stdout)?;
        }
        self.wrote_prompt = true;
        loop {
            match self.rl.readline("> ") {
                Ok(line) => return Ok(Some(line)),
                Err(ReadlineError::Io(e)) => return Err(e),
                Err(ReadlineError::Eof) => return Ok(None),
                Err(ReadlineError::Interrupted) => return Ok(None),
                #[cfg(unix)]
                Err(ReadlineError::Errno(e)) => return Err(e.into()),
                Err(ReadlineError::Signal(_)) => (), // TODO: Rethink
                Err(e) => return Err(io::Error::other(e)),
            }
        }
    }
}
