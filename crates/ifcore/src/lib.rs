use std::io::{self, BufRead};

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

pub fn run_game<G: GameBuilder>(game: G) -> io::Result<()> {
    let mut first = true;
    let mut stdin = io::stdin().lock();
    let mut r = game.start();
    loop {
        if !std::mem::replace(&mut first, false) {
            println!();
        }
        println!("{}", r.text());
        let Some(game) = r.into_game() else {
            break;
        };
        println!();
        print!("> ");
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        r = game.handle_input(&input);
    }
    Ok(())
}
