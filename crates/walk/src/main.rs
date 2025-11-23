mod game;
use advcore::{ReadlineInterfaceBuilder, io_exit, run_game};

fn main() -> std::process::ExitCode {
    io_exit(run_game(ReadlineInterfaceBuilder, game::Builder))
}
