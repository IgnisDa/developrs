FROM debian:bookworm

LABEL maintainer="ignisda2001@gmail.com"

ARG BUILD_DATE
ARG VCS_REF
ARG BUILD_VERSION

# Labels.
LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.build-date=$BUILD_DATE
LABEL org.label-schema.name="IgnisDa/developr-workspace"
LABEL org.label-schema.description="The containerized workspace for smooth code development"
LABEL org.label-schema.vcs-url="https://github.com/IgnisDa/developr"
LABEL org.label-schema.vcs-ref=$VCS_REF
LABEL org.label-schema.version=$BUILD_VERSION

ENV USERNAME=developr
ARG UUID=1000
ARG GUID=$UUID
ENV HOME=/home/${USERNAME}

RUN set -eux ; \
    apt-get update && \
    apt-get install -y postgresql-13 postgresql-contrib-13 sudo fish git gcc \
    neovim bash openssl exa bat wget python3 python3-pip nodejs yarn npm python3-dev curl ; \
    groupadd -g $GUID $USERNAME ; \
    useradd -ms /bin/bash ${USERNAME} -u $UUID -g $USERNAME -g sudo ; \
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} && chmod 0440 /etc/sudoers.d/${USERNAME} ; \
    pip3 install cookiecutter poetry ; \
    curl https://starship.rs/install.sh -sSf | sh -s -- --force ; \
    curl https://raw.githubusercontent.com/IgnisDa/printr/main/get-printr.sh | sh -s -- --yes ; \
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/bin

USER ${USERNAME}

# Setup PostgreSQL server for user gitpod
ENV PATH="$PATH:/usr/lib/postgresql/13/bin"
ENV PGDATA="${HOME}/.pgsql/data"
ENV DATABASE_URL="postgresql://${USERNAME}@localhost"
ENV PGHOSTADDR="127.0.0.1"
ENV PGDATABASE="postgres"
ENV PATH="$PATH:${HOME}/.pg_ctl/bin"

RUN set -eux ; \
    mkdir -p ~/.pg_ctl/bin ~/.pg_ctl/sockets \
    && printf '#!/bin/bash\n[ ! -d $PGDATA ] && mkdir -p $PGDATA && initdb -D $PGDATA\npg_ctl -D $PGDATA -l ~/.pg_ctl/log -o "-k ~/.pg_ctl/sockets" start\n' > ~/.pg_ctl/bin/pg_start \
    && printf '#!/bin/bash\npg_ctl -D $PGDATA -l ~/.pg_ctl/log -o "-k ~/.pg_ctl/sockets" stop\n' > ~/.pg_ctl/bin/pg_stop \
    && chmod +x ~/.pg_ctl/bin/* ; \
    git config --global pull.rebase true ; \
    sudo npm install -g pnpm ; \
    curl -sSL https://raw.githubusercontent.com/IgnisDa/Gitzer/main/get-gitzer.py | python3 ; \
    # This is a bit of a hack. At the moment we have no means of starting background
    # tasks from a Dockerfile. This workaround checks, on each bashrc eval, if the
    # PostgreSQL server is running, and if not starts it.
    printf "\n# Auto-start PostgreSQL server.\n[[ \$(pg_ctl status | grep PID) ]] || pg_start > /dev/null\n" | sudo tee -a /etc/bash.bashrc ; \
    # Install a snippet manager
    cd /tmp && wget https://github.com/knqyf263/pet/releases/download/v0.3.0/pet_0.3.0_linux_amd64.deb \
    && sudo dpkg -i pet_0.3.0_linux_amd64.deb
