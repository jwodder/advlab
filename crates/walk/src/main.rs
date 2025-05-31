mod game;
use ifcore::{io_exit, run_game, StandardInterface};

fn main() -> std::process::ExitCode {
    io_exit(run_game(StandardInterface, game::Builder))
}
