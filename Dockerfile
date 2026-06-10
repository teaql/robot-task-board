FROM ubuntu:22.04

# Install necessary libraries if any, or just setup the env
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the TTYD binary and the Rust TUI release binary
COPY ttyd /app/ttyd
COPY robot-task-board-release /app/robot-task-board-release

RUN chmod +x /app/ttyd /app/robot-task-board-release

# Expose the ttyd default port or custom port
EXPOSE 80

# Run ttyd, exposing the rust TUI
CMD ["/app/ttyd", "-p", "80", "/app/robot-task-board-release"]
