FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld libpq-dev libssl-dev pkg-config \
   && apt-get autoremove -y && apt-get clean -y

RUN cargo install sea-orm-cli ;\
    cargo install sqlx-cli --no-default-features --features "postgres,native-tls" ;

RUN cargo install cargo-edit ; \
    cargo install cargo-watch
