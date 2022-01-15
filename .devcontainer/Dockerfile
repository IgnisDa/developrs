FROM ignisda/developr-workspace:rust
RUN sudo apt-get update \
    sudo apt-get install -y libpq-dev \
    cargo install diesel_cli --no-default-features --features postgres
