# Stage 1: Build frontend
FROM node:18 AS frontend-build
WORKDIR /app/robot-task-board/web
COPY robot-task-board/web/package*.json ./
RUN npm install
COPY robot-task-board/web/ .
RUN npm run build

# Stage 2: Build backend
FROM rust:latest AS backend-build
WORKDIR /app
COPY teaql-rs/ teaql-rs/
COPY robot-task-board/ robot-task-board/
WORKDIR /app/robot-task-board
RUN cargo build --release

# Stage 3: Runtime
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-build /app/robot-task-board/target/release/robot-task-board .
COPY --from=frontend-build /app/robot-task-board/web/dist ./web/dist
EXPOSE 3000
CMD ["./robot-task-board"]
