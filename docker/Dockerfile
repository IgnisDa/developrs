FROM alpine:3.13

ENV USERNAME=developr
ARG UUID=1000
ARG GUID=$UUID

RUN apk update && \
    apk add postgresql=13.3-r0 sudo fish git neovim bash \
    curl gcc libressl-dev musl-dev libffi-dev python3 nodejs yarn npm

RUN addgroup -S sudo; \
    addgroup -g $GUID $USERNAME \
    && adduser -u $UUID -G $USERNAME -G sudo -s /bin/bash -D $USERNAME --disabled-password; \
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/$USERNAME && chmod 0440 /etc/sudoers.d/$USERNAME

RUN set -eux; \
    ln -sf python3 /usr/bin/python && python3 -m ensurepip && pip3 install --no-cache --upgrade pip setuptools; \
    mkdir -p /var/lib/postgresql; \
    chown -R postgres:postgres /var/lib/postgresql; \
    su - postgres -c "initdb /var/lib/postgresql/data"; \
    echo "host all  all    0.0.0.0/0  md5" >> /var/lib/postgresql/data/pg_hba.conf; \
    echo "listen_addresses='*'" >> /var/lib/postgresql/data/postgresql.conf; \
    sed -i '66d' /var/lib/postgresql/data/postgresql.conf; \
    echo "unix_socket_directories = '/tmp'" >> /var/lib/postgresql/data/postgresql.conf; \
    su - postgres -c "pg_ctl start -D /var/lib/postgresql/data -l /var/lib/postgresql/log.log"; \
    su - postgres -c "createuser ${USERNAME}"

ENV PGDATA=/var/lib/postgresql/data
ENV POSTGRES_PASSWORD=admin-password
ENV POSTGRES_USER=postgres

RUN set -eux; \
    pip3 install cookiecutter; \
    apk add poetry --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/ ; \
    curl https://starship.rs/install.sh -sSf | sh -s -- --force ; \
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/bin ;

USER ${USERNAME}

RUN set -eux; \
    curl https://raw.githubusercontent.com/IgnisDa/printr/main/get-printr.sh | sh -s -- --yes ; \
    git config --global pull.rebase true ; \
    curl -sSL https://raw.githubusercontent.com/IgnisDa/Gitzer/main/get-gitzer.py | python3 ;

COPY script.sh /usr/local/bin/

ENTRYPOINT [ "script.sh" ]

CMD [ "bash" ]
