FROM archlinux

ENV UUID=1000
ENV GUID=${UUID}
ENV USERNAME=code

RUN pacman -Syyu --noconfirm reflector
RUN reflector --latest 20 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
RUN pacman -S --noconfirm --needed base-devel git sudo

RUN groupadd -g ${GUID} ${USERNAME}
RUN useradd -ms /bin/bash ${USERNAME} -u ${UUID} -g ${USERNAME} -g wheel
RUN echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME}
RUN chmod 0440 /etc/sudoers.d/${USERNAME}

RUN for pkg in rustup nodejs npm python3 python-pip fish curl wget; do pacman -S --noconfirm $pkg; done
RUN npm install --global pnpm yarn

RUN wget "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" -O "/tmp/cargo-binstall.tgz"
RUN tar zxvf "/tmp/cargo-binstall.tgz"
RUN mv cargo-binstall "/usr/bin/"

USER $USERNAME

RUN rustup default stable
RUN cargo install sqlx-cli --no-default-features --features "postgres,native-tls"
RUN cargo binstall sea-orm-cli cargo-nextest cargo-watch --no-confirm
RUN pip install httpie
