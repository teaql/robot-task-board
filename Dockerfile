# Stage 1: Build frontend (uses native architecture for speed)
FROM --platform=linux/amd64 node:18-alpine AS frontend-build
WORKDIR /app/web
COPY web/package*.json ./
RUN npm install
# Install CA certificates here so we can copy them to the final stage
RUN apk add --no-cache ca-certificates
COPY web/ .
RUN npm run build

# Stage 2: Build backend (uses native architecture for speed)
FROM --platform=linux/amd64 messense/cargo-zigbuild:latest AS backend-build
WORKDIR /app/robot-task-board
COPY . .
# Add musl target for arm64 and build a static binary via zig
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo zigbuild --release --target aarch64-unknown-linux-musl

# Stage 3: Runtime (FROM scratch, absolutely zero OS)
FROM scratch
# Copy CA certificates from frontend stage
COPY --from=frontend-build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
COPY --from=backend-build /app/robot-task-board/target/aarch64-unknown-linux-musl/release/robot-task-board .
COPY --from=frontend-build /app/web/dist ./web/dist
EXPOSE 3000
CMD ["./robot-task-board"]
