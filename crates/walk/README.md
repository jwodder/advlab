[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.82-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/advlab.svg)](https://opensource.org/licenses/MIT)

`walk` is a text adventure (using the term loosely) in which you walk around
some rooms and can look at stuff.  There is no goal or end state; you just
enter commands until you quit.

Commands
========

All vocabulary is case-insensitive.

- `N`, `NORTH`
- `E`, `EAST`
- `W`, `WEST`
- `S`, `SOUTH`
- `BACK`, `RETURN`, `RETREAT` — Return to the previous room
- `EXAMINE`, `LOOK`, `DESCRIBE` — Print the full description of the current
  room.  When used with an object, print a detailed description of that object.
    - "`ROOM`" may also be used as an object for this command.
    - Additional syntaxes: `LOOK AT ⟨object⟩`, `LOOK @ ⟨object⟩`
- `READ` — Read a readable object.  When not used with an object, there must be
  a readable object in the current room.
- `QUIT`, `EXIT` — Quit the game
