FROM ignisda/developr-workspace

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="${HOME}/.cargo/bin:$PATH"

RUN cargo install cargo-watch
