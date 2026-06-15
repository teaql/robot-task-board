#!/bin/bash
set -e

# Build the docker container
# By default, docker build builds for the host's architecture.
# If you are on an M1/M2/M3 Mac (arm64), it will automatically build an arm64 container.
# If you are on x86_64 and want to cross-compile for arm64, you can use buildx:
# docker buildx build --platform linux/arm64 -t robot-task-board:latest .

echo "Building Docker container for Robot Task Board..."
docker build -t robot-task-board:latest .

echo "Build complete!"
echo "To run the container, use:"
echo "docker run -p 3000:3000 -e PG_HOST=... -e PG_USER=... -e PG_PASSWORD=... -e PG_DBNAME=... robot-task-board:latest"
