FROM ignisda/developr-workspace:rust
RUN sudo apt-get update ;\
    sudo apt-get install -y libpq-dev libssl-dev pkg-config ;\
    cargo install diesel_cli --no-default-features --features postgres ;\
    sudo pip install httpie
