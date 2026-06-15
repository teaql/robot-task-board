#!/bin/bash

CONTAINER_NAME="robot-kanban-pg"
PORT=5433

# 检查是否提供了 PG_PASSWORD
if [ -z "$PG_PASSWORD" ]; then
    echo "Error: PG_PASSWORD environment variable is not set."
    echo "Usage: PG_PASSWORD='your_password' ./start_pg.sh"
    exit 1
fi

echo "Starting PostgreSQL container ($CONTAINER_NAME)..."

docker stop $CONTAINER_NAME 2>/dev/null
docker rm $CONTAINER_NAME 2>/dev/null

docker run -d \
  --name $CONTAINER_NAME \
  --platform linux/arm64 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD="$PG_PASSWORD" \
  -e POSTGRES_DB=postgres \
  -p $PORT:5432 \
  postgres:latest

echo "Waiting for PostgreSQL to be ready..."

until docker exec $CONTAINER_NAME pg_isready -U postgres > /dev/null 2>&1; do
    echo "Waiting for database connection..."
    sleep 2
done

echo "✅ PostgreSQL is ready and running on port $PORT!"
