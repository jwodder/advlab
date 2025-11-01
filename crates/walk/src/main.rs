mod game;
use advcore::{ReadlineInterface, io_exit, run_game};

fn main() -> std::process::ExitCode {
    io_exit(run_game(ReadlineInterface, game::Builder))
}
