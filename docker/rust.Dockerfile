FROM ignisda/developr-workspace

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . ${HOME}/.cargo/env ; cargo install cargo-watch
