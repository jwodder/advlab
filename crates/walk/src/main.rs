mod game;
use advcore::{io_exit, run_game, ReadlineInterface};

fn main() -> std::process::ExitCode {
    io_exit(run_game(ReadlineInterface, game::Builder))
}
