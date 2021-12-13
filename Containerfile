FROM ubuntu:latest as build

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Moscow
RUN apt-get update && apt-get install -y cargo pkg-config libssl-dev

WORKDIR /build/

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --target-dir /target --release --target x86_64-unknown-linux-gnu

FROM ubuntu:latest

# TODO: still doesn't work
RUN apt-get update && apt-get install libssl1.1

COPY --from=build /target/x86_64-unknown-linux-gnu/release/slasyz_ru /usr/local/bin/

CMD ["/usr/local/bin/slasyz_ru"]
