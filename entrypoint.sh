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
GOTHUB="/usr/local/go/bin/gothub"

function user_cargo {
    arg="$CARGO $1"
    su futurecommander -c "$arg"
}

function build_windows {
    user_cargo "build -v --release --target=x86_64-pc-windows-gnu"
    echo "$EXEC_DIR/target/x86_64-pc-windows-gnu/release/futurecommander.exe"
}

function build_linux {
    user_cargo "build -v --release"
    echo "$EXEC_DIR/target/release/futurecommander"
}

function test_with_coverage {

    bash <(curl -s https://codecov.io/bash)
}

function release {
    if [ -z "${GITHUB_TOKEN}" ]; then
        exit 1
    fi
    git remote add release https://narfai:${GITHUB_TOKEN}@github.com/narfai/futurecommander.git

    branch=$(git rev-parse --abbrev-ref HEAD | tr \/ . | tr \_ .)
    build=$(date "+%y%m%d%s")

    if [ ! -z "$($GOTHUB info -u narfai -r futurecommander | grep -i ${branch})" ]; then
        echo "Release already exists, delete it"

        $GOTHUB delete \
            --user narfai \
            --repo futurecommander \
            --tag "${branch}"
    fi

    user_cargo "tarpaulin --all --count --out Xml"
    rm -Rf "${EXEC_DIR}/target/*"

    linux_file=$(build_linux)
    echo "Build $linux_file OK"

    windows_file=$(build_windows)
    echo "Build $windows_file OK"

    user_cargo "clippy --all-targets --all-features -- -D warnings"

    bash <(curl -s https://codecov.io/bash)

    if [ -z "$(git tag |grep -i ${branch})" ]; then
        git tag "${branch}"
        git push --tags release
    fi

    $GOTHUB release \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "${branch}-$build" \
        --description "Auto-release ${branch} ( Build $build )" \
        --pre-release

    $GOTHUB upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_linux64_${branch}" \
        --file "$linux_file" \
        --replace

    $GOTHUB upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_win64_${branch}.exe" \
        --file "$windows_file" \
        --replace
}

useradd -u $(get_bare_uid /usr/src/futurecommander/target) -g staff -d /usr/src/futurecommander futurecommander
chown futurecommander Cargo.toml Cargo.lock
chmod -R g+w  /usr/local
chmod a+rw .
chmod -R a+rw samples

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
