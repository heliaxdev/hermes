# Used for running Hermes in Docker containers
#
# Usage: (from the root of the working copy)
#   $ docker build . -t informalsystems/hermes -f ci/release/hermes.Dockerfile

FROM rust:1-buster AS build-env

ARG TAG
ARG PROTOC_VERSION=28.1

WORKDIR /root

COPY . .

# Install protoc
RUN ARCH=$(uname -m) && OS=$(uname -s) && \
    if [ "$OS" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then \
        PROTOC_ZIP=protoc-$PROTOC_VERSION-linux-x86_64.zip; \
    elif [ "$OS" = "Linux" ] && [ "$ARCH" = "aarch64" ]; then \
        PROTOC_ZIP=protoc-$PROTOC_VERSION-linux-aarch_64.zip; \
    else \
        echo "Unsupported OS/architecture: $OS-$ARCH"; exit 1; \
    fi && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_ZIP -O /tmp/protoc.zip && \
    unzip /tmp/protoc.zip -d /usr/local && \
    rm -rf /tmp/protoc.zip
RUN cargo build --release

FROM ubuntu:latest
LABEL maintainer="hello@informal.systems"
ARG UID=2000
ARG GID=2000

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
RUN groupadd -g ${GID} hermes && useradd -l -m hermes -s /bin/bash -u ${UID} -g ${GID}

WORKDIR /home/hermes
USER hermes:hermes
ENTRYPOINT ["/usr/bin/hermes"]

COPY --chown=hermes:hermes --from=build-env /root/target/release/hermes /usr/bin/hermes
