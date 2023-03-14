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
LABEL org.label-schema.vcs-url="https://github.com/ignisda/developr"

ARG UUID=1000
ARG GUID=$UUID
ENV USERNAME=archlinux

RUN pacman -Syyu --noconfirm --needed reflector base-devel git sudo

RUN groupadd -g ${GUID} ${USERNAME} ;\
    useradd -ms /bin/bash ${USERNAME} -u ${UUID} -g ${USERNAME} -g wheel ;\
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} ;\
    chmod 0440 /etc/sudoers.d/${USERNAME}

RUN for pkg in nushell rustup nodejs npm python3 fish curl wget python-pip; do pacman -S --noconfirm $pkg; done

RUN sed -e '/NoProgressBar/ s/^#*/#/' -i /etc/pacman.conf ;\
    sed -e '/Color/ s/^#//' -i /etc/pacman.conf

RUN curl https://zyedidia.github.io/eget.sh | sh ;\
    mv eget /usr/bin/eget

RUN set -eu ;\
    npm install -g yarn zx ;\
    wget "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" -O "cargo-binstall.tgz" ;\
    tar zxvf "cargo-binstall.tgz" ;\
    mv "cargo-binstall" "/usr/bin/cargo-binstall" ;\
    rm -rf "cargo-binstall.tgz"

ADD docker/init-ssh.sh /init-ssh.sh

RUN chmod +x /init-ssh.sh ;\
    /init-ssh.sh ;\
    rm -rf /init-ssh.sh

USER $USERNAME

RUN rustup default stable ;\
    rustup component add rust-analyzer ;\
    sudo ln -s $(rustup which rust-analyzer) "/usr/local/bin/rust-analyzer"

RUN cd /tmp ;\
    git clone "https://aur.archlinux.org/paru-bin.git" paru ;\
    cd paru ;\
    makepkg -si --noconfirm ;\
    cd .. ;\
    rm -rf paru

RUN curl -fsSL https://moonrepo.dev/install.sh | bash
RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash

RUN cargo binstall cargo-nextest cargo-watch --no-confirm

RUN paru -S --noconfirm neovim httpie tokei-bin helix gitui mprocs-bin

RUN npm config set store-dir "$HOME/.store"

RUN git config --global init.defaultBranch main

RUN fish -c "set -Ua fish_user_paths $HOME/.cargo/bin $HOME/.local/bin $HOME/.moon/tools/moon/latest $HOME/.proto/bin"

CMD ["fish"]
