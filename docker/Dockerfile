# syntax=docker/dockerfile:1.3-labs

# DEV: Just collapsing the `RUN` commands into one does not reduce the final image size.
FROM archlinux/archlinux:latest

LABEL maintainer="ignisda2001@gmail.com"

ARG BUILD_DATE

# Labels.
LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.build-date=$BUILD_DATE
LABEL org.label-schema.name="ignisda/archlinux"
LABEL org.label-schema.description="The containerized workspace for smooth code development"
LABEL org.label-schema.vcs-url="https://github.com/ignisda/developrs"

ARG UUID=1000
ARG GUID=$UUID
ENV USERNAME=archlinux
ENV TZ="Asia/Kolkata"

RUN sed -e '/NoProgressBar/ s/^#*/#/' -i /etc/pacman.conf ;\
    sed -e '/Color/ s/^#//' -i /etc/pacman.conf

RUN pacman -Syyu --noconfirm --needed reflector base-devel git sudo

RUN groupadd -g "${GUID}" "${USERNAME}" ;\
    useradd -ms /bin/bash "${USERNAME}" -u "${UUID}" -g "${USERNAME}" -g wheel ;\
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}" ;\
    chmod 0440 "/etc/sudoers.d/${USERNAME}"

RUN pacman -S --noconfirm rustup nodejs npm python3 fish curl wget python-pip openssl-1.1

RUN curl https://zyedidia.github.io/eget.sh | sh ;\
    mv eget /usr/bin/eget

RUN set -eu ;\
    wget "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" -O "cargo-binstall.tgz" ;\
    tar zxvf "cargo-binstall.tgz" ;\
    mv "cargo-binstall" "/usr/bin/cargo-binstall" ;\
    rm -rf "cargo-binstall.tgz"

ADD docker/init-ssh.sh /init-ssh.sh

RUN chmod +x /init-ssh.sh ;\
    /init-ssh.sh ;\
    rm -rf /init-ssh.sh

USER $USERNAME

RUN pushd /tmp ;\
    git clone "https://aur.archlinux.org/paru-bin.git" paru ;\
    pushd paru ;\
    makepkg -si --noconfirm ;\
    popd -2 ;\
    rm -rf paru

RUN rustup default stable

RUN cargo binstall cargo-nextest cargo-watch cargo-udeps --no-confirm

RUN paru -S --noconfirm httpie tokei helix helixbinhx gitui mprocs-bin lf marksman-bin rust-analyzer

RUN npm config set store-dir "$HOME/.store"

RUN git config --global init.defaultBranch main

SHELL ["/usr/bin/fish", "--command"]

RUN set -Ua fish_user_paths \
    $HOME/.cargo/bin \
    $HOME/.local/bin \
    $HOME/.moon/bin \
    $HOME/.proto/bin \
    $HOME/.proto/tools/node/globals/bin

RUN curl -fsSL https://moonrepo.dev/install/moon.sh | bash && moon --version
RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash && proto --version

RUN mkdir -p ~/.config/fish/completions
RUN moon completions --shell fish > ~/.config/fish/completions/moon.fish
RUN proto completions --shell fish > ~/.config/fish/completions/proto.fish

RUN proto install-global node zx typescript-language-server vscode-langservers-extracted
