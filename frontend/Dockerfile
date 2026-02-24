FROM rust:1-bookworm AS builder

# Install trunk and add WASM target
RUN cargo install trunk \
    && rustup target add wasm32-unknown-unknown

WORKDIR /opt/app

# Copy workspace-level files needed for the build
COPY Cargo.toml /opt/
COPY shared/ /opt/shared/

# Create stub Cargo.toml files for workspace members not needed in this build
# so that `cargo metadata` (invoked by trunk) can resolve the full workspace
RUN mkdir -p /opt/server/src /opt/server/macros/src \
    && printf '[package]\nname = "oxibooru_server"\nversion = "0.0.0"\nedition = "2024"\n' > /opt/server/Cargo.toml \
    && touch /opt/server/src/lib.rs \
    && printf '[package]\nname = "server-macros"\nversion = "0.0.0"\nedition = "2024"\n' > /opt/server/macros/Cargo.toml \
    && touch /opt/server/macros/src/lib.rs

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
