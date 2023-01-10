#!/usr/bin/env bash
set -x
set -eo pipefail

#Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
#Check if a customer password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#Check if a custom database name gas been set, otherwise default to 'newsletters'
DB_NAME="${POSTGRES_DB:=newsletters}"
#Check if a custom port gas been set,, otherwise default to '5432'
DB_PORT="${POSTGRESS_PORT:=5432}"

#Lounch postgres using Docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_PASSWORD} \
  -e "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
  # ^ Increased maximum number of connections for testing purposes