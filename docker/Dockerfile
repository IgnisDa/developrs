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

RUN for pkg in rustup nodejs npm python3 fish curl wget python-pip; do pacman -S --noconfirm $pkg; done


RUN printf '#!/bin/sh\nexport PATH=$PATH:$HOME/.cargo/bin\n' >> /etc/profile.d/paths.sh ;\
    printf '#!/bin/sh\nexport PATH=$PATH:$HOME/.local/bin\n' >> /etc/profile.d/paths.sh ;\
    printf '#!/bin/sh\nexport PATH=$PATH:$HOME/.moon/tools/moon/latest\n' >> /etc/profile.d/paths.sh ;\
    chmod +x /etc/profile.d/paths.sh

RUN set -eu ;\
    npm install -g yarn ;\
    wget "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" -O "cargo-binstall.tgz" ;\
    tar zxvf "cargo-binstall.tgz" ;\
    mv "cargo-binstall" "/usr/bin/cargo-binstall"

USER $USERNAME

RUN rustup default stable

RUN cd /tmp ;\
    git clone "https://aur.archlinux.org/paru-bin.git" paru ;\
    cd paru ;\
    makepkg -si --noconfirm

RUN curl -fsSL https://moonrepo.dev/install.sh | bash

RUN cargo binstall cargo-nextest cargo-watch --no-confirm

RUN paru -S httpie --noconfirm

RUN npm config set store-dir "$HOME/.store"
