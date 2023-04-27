FROM rust:latest as builder
LABEL stage=builder
WORKDIR /usr/src/rbx-gp-watcher
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml


RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rbx-gp-watcher/target \
    cargo install --path .


from debian:buster-slim
ENV PATH="/home/pog/.cargo/bin:${PATH}"
COPY --from=builder /usr/local/cargo/bin/rbx-gp-watcher /usr/local/bin/rbx-gp-watcher
RUN apt update
RUN apt install openssl -y

CMD ["rbx-gp-watcher"]