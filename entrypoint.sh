#!/usr/bin/env bash

set -e

EXEC_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

function usage {
    echo -e "Usage:
    fc  {[run]|release|test|shell|build_windows|build_linux|lint|check|coverage}"
    exit 1
}

function get_bare_uid {
    DETECTED_UID=($(ls -dn $1))
    echo ${DETECTED_UID[2]}
}

CARGO=$(which cargo)
GOTHUB="/usr/local/go/bin/gothub"
BARE_UID=$(get_bare_uid /usr/src/futurecommander/target)

function user_cargo {
    arg="$CARGO $1"
    if [[ ${BARE_UID} -ne 0 ]]; then
        su futurecommander -c "$arg"
    else
        $arg
    fi
}

function build_windows {
    user_cargo "build -v --release --target=x86_64-pc-windows-gnu"
    echo "$EXEC_DIR/target/x86_64-pc-windows-gnu/release/futurecommander.exe"
}

function build_linux {
    user_cargo "build -v --release"
    echo "$EXEC_DIR/target/release/futurecommander"
}

echo "BARE UID"
if [[ ${BARE_UID} -ne 0 ]]; then
    useradd -u "${BARE_UID}" -g staff -d /usr/src/futurecommander futurecommander 2> /dev/null
    chown futurecommander Cargo.toml Cargo.lock
    chmod -R g+w  /usr/local
    chmod a+rw .
fi
chmod -R a+rwx samples
rm -Rf "${EXEC_DIR}/target/*"

case "$1" in
    test)
        user_cargo "test --all -v"
        ;;
    build_linux)
        build_linux
        ;;
    build_windows)
        build_windows
        ;;
    lint)
        user_cargo "clippy --all-targets --all-features -- -D warnings"
        ;;
    cargo)
        user_cargo "$@"
        ;;
    check)
        user_cargo "clippy --all-targets --all-features -- -D warnings"
        user_cargo "test --all -v"
        ;;
    coverage)
        rm -Rf "${EXEC_DIR}/samples/dynamic/*"
        user_cargo "tarpaulin --all --count --out Xml -- --test-threads=1"
        bash <(curl -s https://codecov.io/bash)
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
