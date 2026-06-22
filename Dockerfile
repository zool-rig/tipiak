FROM rust:1.96 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=linux/amd64 rust:1.96 AS dx-builder
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli@0.7.3 --root /.cargo -y --force

FROM --platform=linux/amd64 rust:1.96 AS builder

ARG TARGET=armv7-unknown-linux-gnueabihf

RUN cargo install cargo-chef

RUN apt-get update && apt-get install -y \
    gcc-arm-linux-gnueabihf \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add ${TARGET}

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
ENV CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc

COPY --from=dx-builder /.cargo/bin/dx /.cargo/bin/dx
ENV PATH="/.cargo/bin:$PATH"

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target ${TARGET} --recipe-path recipe.json

COPY . .

# dx bundle gère le frontend WASM et les assets sans --target
RUN dx bundle --web --release --package tipiak-app

# Cross-compile le binaire serveur pour ARM séparément
RUN cargo build --release --target ${TARGET} \
    --package tipiak-app \
    --features server

# Remplace le binaire serveur x86 par le binaire ARM
RUN cp target/${TARGET}/release/tipiak-app \
       target/dx/tipiak-app/release/web/server

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/tipiak-app/release/web/ /usr/local/app

ENV PORT=8090
ENV IP=0.0.0.0
EXPOSE 8090

WORKDIR /usr/local/app
ENTRYPOINT ["/usr/local/app/server"]