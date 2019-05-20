FROM rust:latest

ENV CODECOV_TOKEN=""
ENV GITHUB_TOKEN=""
ENV GOPATH="/usr/local/go"

WORKDIR /usr/src/futurecommander

RUN mkdir -p $GOPATH \
    && apt-get update \
    && apt-get install -y \
        gcc-mingw-w64-x86-64 libssl-dev pkg-config cmake zlib1g-dev golang jq \
    && rustup update && rustup target add x86_64-pc-windows-gnu && rustup component add clippy \
    && go get github.com/itchio/gothub \
    && rm -rf /var/lib/apt/lists/*

RUN RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin

COPY . .

VOLUME "/usr/src/futurecommander/target"
ENTRYPOINT ["/usr/src/futurecommander/entrypoint.sh"]
