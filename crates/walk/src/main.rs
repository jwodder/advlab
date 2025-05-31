mod game;

fn main() -> std::io::Result<()> {
    ifcore::run_game(ifcore::StandardInterface, game::Builder)
}
