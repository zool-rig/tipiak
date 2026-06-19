FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage x86 natif, forcé explicitement peu importe la plateforme cible
FROM --platform=linux/amd64 rust:1 AS dx-builder
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force

FROM chef AS builder

ARG TARGET=armv7-unknown-linux-musleabihf

RUN apt-get update && apt-get install -y \
    gcc-arm-linux-gnueabihf \
    musl-tools \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add ${TARGET}

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc
ENV CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc

# Copie dx compilé en x86 natif
COPY --from=dx-builder /.cargo/bin/dx /.cargo/bin/dx
ENV PATH="/.cargo/bin:$PATH"

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target ${TARGET} --recipe-path recipe.json

COPY . .

RUN dx bundle --web --release --target ${TARGET} \
    --manifest-path tipiak/crates/tipiak-app/Cargo.toml

FROM debian:bookworm-slim AS runtime

COPY --from=builder /app/tipiak/crates/tipiak-app/target/dx/tipiak-app/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT ["/usr/local/app/server"]