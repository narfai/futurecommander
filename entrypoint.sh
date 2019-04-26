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

    bash <(curl -s https://codecov.io/bash)
}

function release {
    if [ -z "${GITHUB_TOKEN}" ]; then
        exit 1
    fi
    su futurecommander -c "$CARGO tarpaulin --all --count --out Xml"
    rm -Rf "${EXEC_DIR}/target/*"

    linux_file=build_linux
    windows_file=build_windows

    user_cargo clippy --all-targets --all-features -- -D warnings

    bash <(curl -s https://codecov.io/bash)

    branch=$(git branch | tr \/ _ )
    git remote add release https://narfai:${GITHUB_TOKEN}@github.com/narfai/futurecommander.git
    git tag "${branch}"
    git push --tags release
    gothub release \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "Build ok for ${branch}" \
        --description "Build ok for ${branch}" \
        --pre-release

    gothub upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_linux64_${branch}" \
        --file "$linux_file"
        --replace

    gothub upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_win64_${branch}.exe" \
        --file "$windows_file"
        --replace
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
