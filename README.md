# FutureCommander


[![codecov](https://codecov.io/gh/narfai/futurecommander/branch/master/graph/badge.svg)](https://codecov.io/gh/narfai/futurecommander)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## License

This is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License.

## Docker

```
docker build -t futurecommander .
docker run --security-opt seccomp=unconfined \
    --rm -v "$(pwd)/target-docker":/usr/src/futurecommander/target \
    futurecommander:latest \
    <command>
```

Command can be :

* run

* test

* lint

* build_windows

* build_linux

* release ( need `-e "CODECOV_TOKEN=..."` and `-e "GITHUB_TOKEN=..."` )

## Trello

https://trello.com/b/A2BvQdR9/futurecommander

## Latest release

### Linux x86_64

https://bitbucket.org/kathreon/futurecommander/downloads/futurecommander_linux64_release_latest

### Windows x86_64 ( mingw-w64 )

https://bitbucket.org/kathreon/futurecommander/downloads/futurecommander_win64_release_latest.exe



