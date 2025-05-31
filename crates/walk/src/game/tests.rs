use super::*;
use advcore::Tester;

#[test]
fn noback() {
    let mut t = Tester::start(Builder);
    assert_eq!(t.game().location, Room::Center);
    t.input("BACK");
    t.assert_output("You weren't anywhere else before here.");
    assert_eq!(t.game().location, Room::Center);
}

#[test]
fn back() {
    let mut t = Tester::start(Builder);
    t.input("NORTH");
    assert_eq!(t.game().location, Room::North);
    assert_eq!(t.game().prev_location, Some(Room::Center));
    t.input("BACK");
    assert_eq!(t.game().location, Room::Center);
    assert_eq!(t.game().prev_location, Some(Room::North));
    t.input("BACK");
    assert_eq!(t.game().location, Room::North);
    assert_eq!(t.game().prev_location, Some(Room::Center));
}

#[test]
fn room_descriptions() {
    let mut t = Tester::start(Builder);
    t.assert_output(format!(
        "{}\n\n{}",
        Room::Center.long_description(),
        Entity::TicTacToe.describe()
    ));
    t.input("NORTH");
    t.assert_output(format!(
        "{}\n\n{}",
        Room::North.long_description(),
        Entity::Globe.describe()
    ));
    t.input("SOUTH");
    t.assert_output(format!(
        "{}\n\n{}",
        Room::Center.short_description(),
        Entity::TicTacToe.describe()
    ));
    t.input("EXAMINE");
    t.assert_output(format!(
        "{}\n\n{}",
        Room::Center.long_description(),
        Entity::TicTacToe.describe()
    ));
}
