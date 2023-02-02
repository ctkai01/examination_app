
#!/usr/bin/env bash

# turn on debug mode
set -x


if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v diesel)" ]; then
    echo >&2 "Error: diesel is not installed."
    echo >&2 "Use:"
    echo >&2 "  cargo install diesel_cli --no-default-features --features postgres  "
    echo >&2 "to install it"
    exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}

# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name has been set, otherwise default to 'examnination''
DB_NAME="${POSTGRES_DB:=examnination}"

# Check if a custom port has been set, otherwise default to '5432''
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
# if [[ -z "${SKIP_DOCKER}" ]]; then
    docker run \
        --name postgres-init \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres
# fi

# Keep pinging Postgres until it's ready to accept commands


until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q';do
    >&2 echo "Progress is still unavaiable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

echo DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME} > .env
diesel setup

>&2 echo "Postgres has been migrated, ready to go!"
