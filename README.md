# FutureCommander

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

[![codecov](https://codecov.io/gh/narfai/futurecommander/branch/master/graph/badge.svg)](https://codecov.io/gh/narfai/futurecommander)

![Docker Build Status](https://img.shields.io/docker/cloud/build/fcadeillan/futurecommander.svg)

[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=narfai/futurecommander)](https://dependabot.com)

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
    -v "$(pwd)":/usr/src/futurecommander \
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

https://github.com/narfai/futurecommander/releases/download/release.v0.4.0/futurecommander_linux64_release.v0.4.0

### Windows x86_64 ( mingw-w64 )

https://github.com/narfai/futurecommander/releases/download/release.v0.4.0/futurecommander_win64_release.v0.4.0.exe
