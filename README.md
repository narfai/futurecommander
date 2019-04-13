# FutureCommander

[![codecov](https://codecov.io/bb/kathreon/futurecommander/branch/master/graph/badge.svg)](https://codecov.io/bb/kathreon/futurecommander)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## License

This is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License.

## Trello

https://trello.com/b/A2BvQdR9/futurecommander

## Latest release

### Linux x86_64

https://bitbucket.org/kathreon/futurecommander/downloads/futurecommander_linux64_release_latest

### Windows x86_64 ( mingw-w64 )

https://bitbucket.org/kathreon/futurecommander/downloads/futurecommander_win64_release_latest.exe


## Note : Generate dependency graph

Must have GraphViz installed ( which provide `dot` binary ).

```
cargo +nightly install --git https://github.com/kbknapp/cargo-graph --force
```


Or add this to Cargo.lock with proper version
```
[root]
name="futurecommander"
version="0.x.x"
```

Then

```
cargo install cargo-graph
```

Generate `.dot` then `.png` files

```
cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange > doc/cargo-count.dot
dot -Tpng > doc/dependency-graph.png doc/cargo-count.dot
```
