#!/bin/bash
set -e

set -a; source .env ; set +a
docker kill ps-db || true; docker rm ps-db || true
docker run --name ps-db -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5432:5432 -d postgres:15.2-alpine