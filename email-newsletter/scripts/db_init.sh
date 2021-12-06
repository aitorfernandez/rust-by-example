#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: `sqlx` is not installed."
  echo >&2 "Use:"
  echo >&2 "cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres" echo >&2 "to install it."
  exit 1
fi

DB_USER="${DB_USER:=postgres}"
DB_PASSWORD="${DB_PASSWORD:=password}"
DB_NAME="${DB_NAME:=email-newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

if [[ -z "${SKIP_DOCKER}" ]]
then
  POSTGRES_CONTAINER_ID=$(docker run \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_PORT="${DB_PORT}" \
    -p "${DB_PORT}":5432 \
    --name email-newsletter \
    -d postgres:14.1-alpine \
    postgres -N 1000)

  echo >&2 "started postgres container with id: ${POSTGRES_CONTAINER_ID}"

  while ! docker exec "${POSTGRES_CONTAINER_ID}" pg_isready -U "${DB_USER}"; do
    echo >&2 "postgres is still unavailable - sleeping"
    sleep 1
  done

  # https://crates.io/crates/sqlx-cli
  sqlx database create --database-url ${DB_URL}
fi

sqlx migrate run --database-url ${DB_URL}

echo >&2 "postgres and database are up and running on port ${DB_URL}"
