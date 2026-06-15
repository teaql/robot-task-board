# Stage 1: Build frontend
FROM node:18 AS frontend-build
WORKDIR /app/web
COPY web/package*.json ./
RUN npm install
COPY web/ .
RUN npm run build

# Stage 2: Build backend
FROM rust:latest AS backend-build
WORKDIR /app/robot-task-board
COPY . .
RUN cargo build --release

# Stage 3: Runtime
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-build /app/robot-task-board/target/release/robot-task-board .
COPY --from=frontend-build /app/web/dist ./web/dist
EXPOSE 3000
CMD ["./robot-task-board"]
