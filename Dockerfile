FROM rust:1.85.0 AS build-stage

ARG APP_NAME=discord-ratelimit-reporter
ARG TARGET=x86_64-unknown-linux-gnu

ARG BUILDDIR=/app
WORKDIR ${BUILDDIR}

# Install cross-compilation tools if needed
RUN apt-get update && \
    if [ "$TARGET" = "aarch64-unknown-linux-gnu" ]; then \
        apt-get install -y --no-install-recommends \
        gcc-aarch64-linux-gnu libc6-dev-arm64-cross; \
    fi

# Add target
RUN rustup target add ${TARGET}

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${BUILDDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release --target ${TARGET}
mkdir -p /bin
cp ./target/${TARGET}/release/$APP_NAME /bin/server
EOF


FROM debian:bookworm-slim

RUN apt update
RUN apt install -y libssl-dev

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build-stage /bin/server /bin/

CMD ["/bin/server"]