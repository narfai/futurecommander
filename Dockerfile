FROM rust:latest

ENV CODECOV_TOKEN=""
ENV GITHUB_TOKEN=""

WORKDIR /usr/src/futurecommander

RUN apt-get update \
    && apt-get install -y \
        gcc-mingw-w64-x86-64 libssl-dev pkg-config cmake zlib1g-dev golang \
    && rustup update && rustup target add x86_64-pc-windows-gnu && rustup component add clippy \
    && go get github.com/itchio/gothub

RUN RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin

COPY . .

VOLUME "/usr/src/futurecommander/target"
ENTRYPOINT ["/usr/src/futurecommander/entrypoint.sh"]
