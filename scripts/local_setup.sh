#!/bin/bash
set -euo pipefail

# --- Environment Variables ---
# Define your PostgreSQL configuration here:
POSTGRES_CONTAINER_NAME="timescaledb"
POSTGRES_PORT="5432"
POSTGRES_USER="yourusername"
POSTGRES_PASSWORD="yourpassword"
POSTGRES_DB="yourdbname"
POSTGRES_IMAGE="timescale/timescaledb:latest-pg14"
POSTGRES_HOST="localhost"

# Build the DATABASE_URL (with SSL disabled for local development)
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}?sslmode=disable"
echo "Using DATABASE_URL: ${DATABASE_URL}"

# Function to start the PostgreSQL container
start_postgres() {
  if ! docker ps --filter "name=^/${POSTGRES_CONTAINER_NAME}$" --format '{{.Names}}' | grep -q "^${POSTGRES_CONTAINER_NAME}\$"; then
    echo "Starting PostgreSQL container..."
    docker run -d \
      --name "${POSTGRES_CONTAINER_NAME}" \
      -p ${POSTGRES_PORT}:5432 \
      -e POSTGRES_USER=${POSTGRES_USER} \
      -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
      -e POSTGRES_DB=${POSTGRES_DB} \
      ${POSTGRES_IMAGE}
  else
    echo "PostgreSQL container '${POSTGRES_CONTAINER_NAME}' is already running."
  fi
}

# Function to wait until PostgreSQL is ready
wait_for_postgres() {
  echo "Waiting for PostgreSQL to be ready..."
  until docker exec "${POSTGRES_CONTAINER_NAME}" pg_isready -U "${POSTGRES_USER}" -h "${POSTGRES_HOST}" >/dev/null 2>&1; do
    echo "PostgreSQL not ready yet. Sleeping..."
    sleep 1
  done
  echo "PostgreSQL is up and running!"
}

# Function to run SQLx database setup
setup_database() {
  echo "Setting up the database with SQLx..."
  sqlx db setup
  echo "Database setup complete."
}

# Function to stop and remove the container
stop_postgres() {
  if docker ps --filter "name=^/${POSTGRES_CONTAINER_NAME}$" --format '{{.Names}}' | grep -q "^${POSTGRES_CONTAINER_NAME}\$"; then
    echo "Stopping and removing PostgreSQL container..."
    docker stop "${POSTGRES_CONTAINER_NAME}"
    docker rm "${POSTGRES_CONTAINER_NAME}"
    echo "PostgreSQL container removed."
  else
    echo "PostgreSQL container '${POSTGRES_CONTAINER_NAME}' is not running."
  fi
}

# Check for command-line argument and execute appropriate functions
if [[ $# -eq 1 ]]; then
  case "$1" in
    start)
      start_postgres
      wait_for_postgres
      setup_database
      ;;
    stop)
      stop_postgres
      ;;
    restart)
      stop_postgres
      start_postgres
      wait_for_postgres
      setup_database
      ;;
    *)
      echo "Usage: $0 {start|stop|restart}"
      exit 1
      ;;
  esac
else
  echo "Usage: $0 {start|stop|restart}"
  exit 1
fi
