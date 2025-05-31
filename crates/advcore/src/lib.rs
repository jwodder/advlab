mod interface;
pub use crate::interface::*;
use std::io::{self, ErrorKind};
use std::process::ExitCode;

pub trait GameBuilder: Sized {
    type Engine: GameEngine;

    fn start(self) -> Output<Self::Engine>;
}

pub trait GameEngine: Sized {
    fn handle_input(self, input: &str) -> Output<Self>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Output<G> {
    // `text` should not end with a newline.
    Continue { game: G, text: String },
    Goodbye { text: String },
}

impl<G> Output<G> {
    pub fn text(&self) -> &str {
        match self {
            Output::Continue { text, .. } => text,
            Output::Goodbye { text } => text,
        }
    }

    pub fn into_game(self) -> Option<G> {
        if let Output::Continue { game, .. } = self {
            Some(game)
        } else {
            None
        }
    }
}

pub fn run_game<IC: InterfaceContext, G: GameBuilder>(ifctx: IC, game: G) -> io::Result<()> {
    let mut r = game.start();
    ifctx.with_interface(move |iface| loop {
        iface.show_output(r.text())?;
        let Some(game) = r.into_game() else {
            return Ok(());
        };
        let input = iface.get_input()?;
        r = game.handle_input(&input);
    })
}

pub fn io_exit(r: io::Result<()>) -> ExitCode {
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) if e.kind() == ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tester<G> {
    game: Option<G>,
    last_output: Option<String>,
}

impl<G: GameEngine> Tester<G> {
    pub fn start<B: GameBuilder<Engine = G>>(builder: B) -> Self {
        builder.start().into()
    }

    pub fn input(&mut self, input: &str) {
        let Some(game) = self.game.take() else {
            panic!("Tester::input() called after game finished");
        };
        *self = game.handle_input(input).into();
    }

    pub fn assert_output<S: AsRef<str>>(&self, output: S) {
        if let Some(prev) = self.last_output.as_deref() {
            assert_eq!(prev, output.as_ref());
        } else {
            panic!("Tester::assert_output() called with no previous output");
        }
    }

    pub fn game(&self) -> &G {
        match self.game.as_ref() {
            Some(game) => game,
            None => panic!("Tester::game() called after game finished"),
        }
    }

    pub fn done(&self) -> bool {
        self.game.is_none()
    }
}

impl<G: GameEngine> From<G> for Tester<G> {
    fn from(game: G) -> Tester<G> {
        Tester {
            game: Some(game),
            last_output: None,
        }
    }
}

impl<G: GameEngine> From<Output<G>> for Tester<G> {
    fn from(output: Output<G>) -> Tester<G> {
        let (game, last_output) = match output {
            Output::Continue { game, text } => (Some(game), Some(text)),
            Output::Goodbye { text } => (None, Some(text)),
        };
        Tester { game, last_output }
    }
}
