FROM debian:trixie-20250811-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    curl \
    git \
    libssl-dev \
    liblua5.4-dev \
    lua5.4 \
    vim \
    powercap-utils \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m dev

RUN mkdir /root/bin
WORKDIR /root/bin
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
ENV PATH="/root/bin/:/root/.local/bin/:${PATH}"

# https://www.rust-lang.org/tools/install
ENV CARGO_HOME="/root/bin/.cargo" PATH="/root/bin/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ARG SCRIPTS_VER=unknown

RUN cat /etc/os-release