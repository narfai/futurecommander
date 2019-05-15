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

function release {
    if [[ -z "${GITHUB_TOKEN}" ]]; then
        exit 1
    fi

    branch=$(git rev-parse --abbrev-ref HEAD | tr \/ . | tr \_ .)
    build=$(date "+%y%m%d%s")

    linux_file=$(build_linux)
    echo "Build $linux_file OK"

    windows_file=$(build_windows)
    echo "Build $windows_file OK"

    user_cargo "clippy --all-targets --all-features -- -D warnings"

    existing_remote=$(git remote -v | grep -i release | xargs)
    echo "EXISTING REMOTE : ${existing_remote}"
    if [[ ! -z "${existing_remote}" ]]; then
        git remote remove release 2> /dev/null
    fi

    git remote add release https://narfai:${GITHUB_TOKEN}@github.com/narfai/futurecommander.git 2> /dev/null

    existing_local_tags=$(git tag | grep -i ${branch} | xargs)
    echo "LOCAL TAGS : ${existing_local_tags}"
    if [[ -z  "${existing_local_tags}" ]]; then
        git tag "${branch}" 2> /dev/null
    fi

    existing_remote_tags=$(git ls-remote --tags release | grep -i ${branch} | xargs)
    echo "REMOTE TAGS : ${existing_local_tags}"
    if [[ -z  "${existing_remote_tags}" ]]; then
        git push --tags release 2> /dev/null
    fi


    if [[ -z "$(${GOTHUB} info -u narfai -r futurecommander | grep -i ${branch} | xargs)" ]]; then
        echo "Create new release"
        ${GOTHUB} release \
           --user narfai \
           --repo futurecommander \
           --tag "${branch}" \
           --name "${branch}" \
           --description "Auto-release ${branch} ( Build $build )" \
           --pre-release
    else
        echo "Update existing release"
        ${GOTHUB} edit \
            --user narfai \
            --repo futurecommander \
            --tag "${branch}" \
            --name "${branch}" \
            --description "Auto-release ${branch} ( Build $build )"
    fi

    echo "Upload $linux_file"
    ${GOTHUB} upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_linux64_${branch}" \
        --file "$linux_file" \
        --replace

    echo "Upload $windows_file"
    ${GOTHUB} upload \
        --user narfai \
        --repo futurecommander \
        --tag "${branch}" \
        --name "futurecommander_win64_${branch}.exe" \
        --file "$windows_file" \
        --replace
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
        user_cargo "tarpaulin --all --count --out Xml"
        bash <(curl -s https://codecov.io/bash)
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
