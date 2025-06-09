[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![CI Status](https://github.com/jwodder/advlab/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/advlab/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/advlab/branch/main/graph/badge.svg)](https://codecov.io/gh/jwodder/advlab)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.79-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/advlab.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/advlab) | [Issues](https://github.com/jwodder/advlab/issues)

This is a [workspace][] for experimenting with developing [text adventure
games][txtadv] in [Rust][] and figuring out how best to code them.  While the
code may work, it is not yet entertaining.

The workspace contains the following packages, in order of creation:

- [`advcore`][] — Core library of common code

- [`walk`][] — A text adventure in which you walk around some rooms and look at
  stuff

[workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[txtadv]: https://en.wikipedia.org/wiki/Text_adventure
[Rust]: https://rust-lang.org
[`advcore`]: https://github.com/jwodder/advlab/tree/main/crates/advcore
[`walk`]: https://github.com/jwodder/advlab/tree/main/crates/walk
