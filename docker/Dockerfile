FROM debian:bookworm

LABEL maintainer="ignisda2001@gmail.com"

ARG BUILD_DATE
ARG BUILD_VERSION

# Labels.
LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.build-date=$BUILD_DATE
LABEL org.label-schema.name="IgnisDa/developr-workspace"
LABEL org.label-schema.description="The containerized workspace for smooth code development"
LABEL org.label-schema.vcs-url="https://github.com/IgnisDa/developr"
LABEL org.label-schema.version=$BUILD_VERSION

ARG UUID=1000
ARG GUID=$UUID
ARG POSTGRES_VERSION=13
ENV USERNAME=developr
ENV HOME=/home/${USERNAME}
ENV WORKSPACE="/workspace"
ENV PATH="$PATH:/usr/lib/postgresql/${POSTGRES_VERSION}/bin"
ENV PSQL_BASE_PATH="${WORKSPACE}/.postgresql"
ENV PGDATA="${PSQL_BASE_PATH}/pgsql/data"

RUN set -eux ; \
    apt-get update -y && apt-get install -y curl wget gnupg2 lsb-release ; \
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - ;\
    sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ `lsb_release -cs`-pgdg main" >> /etc/apt/sources.list.d/pgdg.list' ;\
    apt-get update -y ;\
    curl -fsSL https://deb.nodesource.com/setup_16.x | bash - ; \
    apt-get install -y postgresql-${POSTGRES_VERSION} postgresql-contrib-${POSTGRES_VERSION} sudo fish git \
    neovim bash exa bat python3 python3-pip nodejs python3-dev procps ; \
    groupadd -g ${GUID} ${USERNAME} ; \
    useradd -ms /bin/bash ${USERNAME} -u ${UUID} -g ${USERNAME} -g sudo ; \
    npm install --global yarn pnpm ; \
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} && chmod 0440 /etc/sudoers.d/${USERNAME} ; \
    pip3 install cookiecutter poetry ; \
    # Install just command runner
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/bin ;\
    # Setup user workspace
    mkdir -p ${WORKSPACE} && chown -R ${USERNAME}:${USERNAME} ${WORKSPACE} ; \
    # Setup locales
    echo en_US.UTF-8 UTF-8 > /etc/locale.gen ; \
    locale-gen en_US.UTF-8 ; \
    # Install printr
    curl https://raw.githubusercontent.com/IgnisDa/printr/main/get-printr.sh | sh -s -- --yes ; \
    # Install tokei to count code count
    wget --quiet -O tokei.tar.gz https://github.com/XAMPPRocky/tokei/releases/latest/download/tokei-x86_64-unknown-linux-gnu.tar.gz ;\
    tar xf tokei.tar.gz -C /usr/local/bin ;\
    tokei --version


USER ${USERNAME}

RUN git config --global pull.rebase true ; \
    curl -sSL https://raw.githubusercontent.com/IgnisDa/Gitzer/main/get-gitzer.py | python3 ; \
    npm config set store-dir ~/.pnpm-store

# Setup PostgreSQL server for user, taken from
# https://github.com/gitpod-io/workspace-images/blob/master/postgres/Dockerfile
RUN set -eux ; \
    mkdir -p ${PSQL_BASE_PATH}/.pg_ctl/bin ${PSQL_BASE_PATH}/.pg_ctl/sockets \
    && printf '#!/bin/bash\n[ ! -d $PGDATA ] && mkdir -p $PGDATA && initdb -D $PGDATA\npg_ctl -D $PGDATA -l ${PSQL_BASE_PATH}/.pg_ctl/log -o "-k ${PSQL_BASE_PATH}/.pg_ctl/sockets" start\n' > ${PSQL_BASE_PATH}/.pg_ctl/bin/pg_start \
    && printf '#!/bin/bash\npg_ctl -D $PGDATA -l ${PSQL_BASE_PATH}/.pg_ctl/log -o "-k ${PSQL_BASE_PATH}/.pg_ctl/sockets" stop\n' > ${PSQL_BASE_PATH}/.pg_ctl/bin/pg_stop \
    && chmod +x ${PSQL_BASE_PATH}/.pg_ctl/bin/* ; \
    printf "sudo chown -R \$USERNAME:\$USERNAME \"\$PGDATA\"\n" | sudo tee -a /etc/bash.bashrc ; \
    printf "if [ -z \"\$(ls -A \"\$PGDATA\")\" ];\n     then initdb;\nfi\n" | sudo tee -a /etc/bash.bashrc ; \
    printf "[[ \$(pg_ctl status | grep PID) ]] || pg_start > /dev/null\n" | sudo tee -a /etc/bash.bashrc ;

ENV PGHOSTADDR="127.0.0.1"
ENV PATH="$PATH:${PSQL_BASE_PATH}/.pg_ctl/bin"
