#!/bin/bash

echo "Starting Robot Kanban Server (Multi-Tenant)..."

if [ -z "$PG_PASSWORD" ]; then
    echo "Error: PG_PASSWORD environment variable is not set."
    echo "Usage: PG_PASSWORD='your_password' ./start_app.sh"
    exit 1
fi

export PG_HOST="127.0.0.1"
export PG_PORT="5433"
export PG_USER="postgres"
export PG_DBNAME="robot_task_board"
export RUST_LOG="info"

echo "Connecting to PostgreSQL at $PG_HOST:$PG_PORT / $PG_DBNAME"

cargo run --release
