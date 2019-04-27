# FutureCommander


[![codecov](https://codecov.io/gh/narfai/futurecommander/branch/master/graph/badge.svg)](https://codecov.io/gh/narfai/futurecommander)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![buddy pipeline](https://app.buddy.works/narfai/futurecommander/pipelines/pipeline/185011/badge.svg?token=621d010d2c0d56fa721cefa04f9d765b57c055fbc413be83bfbe30368c18ebe4 "buddy pipeline")](https://app.buddy.works/narfai/futurecommander/pipelines/pipeline/185011)
[![buddy pipeline](https://app.buddy.works/narfai/futurecommander/pipelines/pipeline/184827/badge.svg?token=621d010d2c0d56fa721cefa04f9d765b57c055fbc413be83bfbe30368c18ebe4 "buddy pipeline")](https://app.buddy.works/narfai/futurecommander/pipelines/pipeline/184827)
![Docker Build Status](https://img.shields.io/docker/cloud/build/fcadeillan/futurecommander.svg)

## License

This is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License.

## Docker

Build static image ( with embed source ) and interact with your target binaries

```
docker build -t futurecommander .
docker run \
    --rm -v "$(pwd)/target-docker":/usr/src/futurecommander/target \
    futurecommander:latest \
    <command>
```

Or for live source editing

```
docker run --rm -t \
    -v "$(pwd)/src":/usr/src/futurecommander/src \
    -v "$(pwd)/target-docker":/usr/src/futurecommander/target \
    futurecommander test
```

Releases needs github token

```
docker run \
    -e "GITHUB_TOKEN=..." \
    --rm -t \
    futurecommander release
```

Coverage needs codecov token( and no seccomp for code coverage )

```
docker run \
    -e "CODECOV_TOKEN=..." \
    --security-opt seccomp=unconfined \
    --rm -t \
    futurecommander coverage
```

Command can be :

* run : run the shell

* test : check tests

* lint : check lint

* build_windows : build windows 64 bit binary

* build_linux : build linux 64 bit binary

* cargo : run any cargo command

* check : test & lint

* coverage ( needs `-e "CODECOV_TOKEN=..."` and `--security-opt seccomp=unconfined`) : send test coverage to dovecot

* release ( needs `-e "GITHUB_TOKEN=..."` ) : test, lint and generate a pre-release on github containing both binaries

## Trello

https://trello.com/b/A2BvQdR9/futurecommander

## Latest release

### Linux x86_64

https://github.com/narfai/futurecommander/releases/download/vlatest/futurecommander_linux64_latest

### Windows x86_64 ( mingw-w64 )

https://github.com/narfai/futurecommander/releases/download/vlatest/futurecommander_win64_latest.exe



