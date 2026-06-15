#!/bin/bash
set -e

# This Dockerfile uses zigbuild to build an optimized, statically-linked arm64 binary.
# It then packages it into a tiny Alpine Linux image.

echo "Building a tiny Alpine-based Docker container for Robot Task Board (arm64)..."

# If you are already on an arm64 machine (e.g. Apple M1/M2), just run:
docker build -t robot-task-board:latest .

# If you are on an x86_64 machine and want to cross-build for arm64, 
# you should uncomment the line below to explicitly set the platform for the final image:
# docker buildx build --platform linux/arm64 -t robot-task-board:latest .

echo "Build complete! The resulting image should be very small (under 50MB)."
echo "To run the container, use:"
echo "docker compose up -d"
