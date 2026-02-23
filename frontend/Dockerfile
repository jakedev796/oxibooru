FROM rust:1-bookworm AS builder

# Install trunk and add WASM target
RUN cargo install trunk \
    && rustup target add wasm32-unknown-unknown

WORKDIR /opt/app

# Copy workspace-level files needed for the build
COPY Cargo.toml /opt/
COPY shared/ /opt/shared/

# Copy the frontend crate
COPY frontend/ /opt/app/

# Build the frontend (trunk build outputs to dist/)
RUN trunk build --release


FROM nginx:alpine AS release

RUN apk --no-cache add dumb-init

# Copy nginx config
COPY frontend/nginx.conf.docker /etc/nginx/nginx.conf

# Copy built static files
COPY --from=builder /opt/app/dist/ /var/www/

# Copy startup script
COPY frontend/docker-start.sh /docker-start.sh
RUN chmod +x /docker-start.sh

CMD ["/docker-start.sh"]
VOLUME ["/data"]
