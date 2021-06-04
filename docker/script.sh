#!/usr/bin/env bash

# Start the postgres server if not running
sudo su - postgres -c "pg_ctl start -D /var/lib/postgresql/data -l /var/lib/postgresql/log.log"

# The following environment variables are acted upon by this script:

# *`DEVELOPR_DATABASE_NAMES`*
#   A list of `|` delimited databases to create on startup. If empty, then do nothing.
#   If there is no `|`, then only a single database is created with the given string.
#   example: 'mainDB', 'database_1|database2'

# *`DEVELOPR_DATABASE_USERS`*
#   A list of `|` delimited users to create on startup. If empty, then do nothing.
#   If there is no `|`, then only create a single user which owns all $DEVELOPR_DATABASE_NAMES.
#   If this is a list, then its length MUST be equal to $DEVELOPR_DATABASE_NAMES,
#   and $DEVELOPR_DATABASE_NAMES be zipped with $DEVELOPR_DATABASE_USERS and each
#   database will be created with corresponding user.
#   example: 'main_user', 'user_1|user2'

#       ENSURE THAT
# there is only one database user
#           OR
# the number of users is equal to the number of databases
test_correct_variables() {
    if [ "$LEN_USERS" == 1 ] || [ "$LEN_DATABASES" == "$LEN_USERS" ]; then
        # there are no errors
        :
    else
        echo "Incorrect \$DEVELOPR_DATABASE_NAMES=$LEN_DATABASES and \$DEVELOPR_DATABASE_USERS=$LEN_USERS set"
        exit 1
    fi
}

set_vars() {
    IFS='|' read -ra DATABASES <<< "$DEVELOPR_DATABASE_NAMES"
    LEN_DATABASES="${#DATABASES[@]}"
    IFS='|' read -ra USERS <<< "$DEVELOPR_DATABASE_USERS"
    LEN_USERS="${#USERS[@]}"
}

docker_process_sql() {
    local query_runner=( psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --no-password )
    if [ -n "$POSTGRES_DB" ]; then
        query_runner+=( --dbname "$POSTGRES_DB" )
    fi
    PGHOST= PGHOSTADDR= "${query_runner[@]}" "$@"
}

# usage: docker_process_init_files [file [file [...]]]
#    ie: docker_process_init_files /always-initdb.d/*
# process initializer files, based on file extensions and permissions
docker_process_init_files() {
    # psql here for backwards compatibility "${psql[@]}"
    psql=( docker_process_sql )

    echo
    local f
    for f; do
        case "$f" in
            *.sh)
                # https://github.com/docker-library/postgres/issues/450#issuecomment-393167936
                # https://github.com/docker-library/postgres/pull/452
                if [ -x "$f" ]; then
                    echo "$0: running $f"
                    "$f"
                else
                    echo "$0: sourcing $f"
                    . "$f"
                fi
                ;;
            *.sql)    echo "$0: running $f"; docker_process_sql -f "$f"; echo ;;
            *.sql.gz) echo "$0: running $f"; gunzip -c "$f" | docker_process_sql; echo ;;
            *.sql.xz) echo "$0: running $f"; xzcat "$f" | docker_process_sql; echo ;;
            *)        echo "$0: ignoring $f" ;;
        esac
        echo
    done
}

read -r -d '' SQL_CREATE_USER << EOL
CREATE ROLE "{{USER}}" WITH LOGIN CREATEDB ENCRYPTED PASSWORD '$POSTGRES_PASSWORD';
EOL

read -r -d '' SQL_CREATE_DB << EOL
CREATE DATABASE "{{DB}}" WITH OWNER="{{USER}}"
                                LC_COLLATE="en_US.utf8"
                                LC_CTYPE="en_US.utf8"
                                ENCODING="UTF8"
                                TEMPLATE=template0;
GRANT ALL PRIVILEGES ON DATABASE "{{DB}}" TO "{{USER}}";
EOL

main() {
    set_vars
    test_correct_variables

    if [ "$LEN_USERS" == 0 ] || [ "$LEN_DATABASES" == 0 ]; then
        echo "Either \$DEVELOPR_DATABASE_NAMES or \$DEVELOPR_DATABASE_USERS empty. Exiting cleanly..."
        return 0
    fi

    if [[ "$LEN_USERS" == "1" ]]; then
        if psql -t -c '\du' -U postgres | cut -d \| -f 1 | grep -qw "${USERS}"; then
            echo "USER '${USERS}' already exists. Skipping..."
        else
            local user_sql="${SQL_CREATE_USER//"{{USER}}"/$USERS}"
            docker_process_sql <<< "$user_sql"
        fi
        for db in "${DATABASES[@]}"; do
            if psql -lqt | cut -d \| -f 1 | grep -qw "${db}"; then
                echo "Database '${DATABASES[$i]}' already exists. Skipping..."
            else
                echo "Creating database '${db}'..."
                local db_sql="${SQL_CREATE_DB//"{{DB}}"/$db}"
                local db_sql="${db_sql//"{{USER}}"/$USERS}"
                docker_process_sql <<< "$db_sql"
            fi
        done
    else
        SQL_TEMPLATE="$SQL_CREATE_USER;;$SQL_CREATE_DB"
        unset result
        for i in {0..$LEN_DATABASES}; do
            if psql -lqt | cut -d \| -f 1 | grep -qw "${DATABASES[$i]}"; then
                echo "Database '${DATABASES[$i]}' already exists. Skipping..."
            else
                echo "Creating database '${DATABASES[$i]}'..."
                local sql="${SQL_TEMPLATE//"{{DB}}"/${DATABASES[$i]}}"
                local sql="${sql//"{{USER}}"/${USERS[$i]}}"
                docker_process_sql <<< $sql
            fi
        done
    fi
    docker_process_init_files /docker-entrypoint-initdb.d/*
}

main
exec "$@"
