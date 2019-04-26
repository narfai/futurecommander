#!/usr/bin/env bash

set -e

EXEC_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

function usage {
    echo -e "Usage:
    fc  {[run]|release|test|shell|build_windows|build_linux|lint}"
    exit 1
}

function get_bare_uid {
    DETECTED_UID=($(ls -dn $1))
    echo ${DETECTED_UID[2]}
}

CARGO=$(which cargo)

function user_cargo {
    shift
    su futurecommander -c "$CARGO $@"
}

function build_windows {
    user_cargo build -v --release
    echo "$EXEC_DIR/target/x86_64-pc-windows-gnu/release/futurecommander.exe"
}


function build_linux {
    user_cargo build -v --release
    echo "$EXEC_DIR/target/release/futurecommander"
}

function test_with_coverage {
    su futurecommander -c "$CARGO tarpaulin --all --count --out Xml"
    bash <(curl -s https://codecov.io/bash)
}

function release {
    rm -Rf "${EXEC_DIR}/target/*"
    linux_file=build_linux
    windows_file=build_windows
    user_cargo clippy --all-targets --all-features -- -D warnings
    test_with_coverage
    #TODO send artifacts to github
}

useradd -u $(get_bare_uid /usr/src/futurecommander/target) -g staff -d /usr/src/futurecommander futurecommander
chown futurecommander Cargo.toml Cargo.lock
chmod -R g+w  /usr/local
chmod a+rw .
chmod -R a+rw samples



case "$1" in
    test)
        user_cargo test --all -v
        ;;
    build_linux)
        build_linux
        ;;
    build_windows)
        build_windows
        ;;
    lint)
        user_cargo clippy --all-targets --all-features -- -D warnings
        ;;
    cargo)
        user_cargo "$@"
        ;;
    release)
        release
        ;;
    run|"")
        user_cargo run
        ;;
    shell|sh)
        shift
        echo "use \"su futurecommander\" to reach userspace"
        bash $@
        ;;
    *)
        usage
        ;;
esac
shift

exit 0
