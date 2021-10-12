FROM ubuntu:latest as build

RUN apt-get update && apt-get install -y cargo

WORKDIR /build/

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM ubuntu:latest

COPY --from=build /build/target/x86_64-unknown-linux-gnu/release/slasyz_ru /usr/local/bin/

CMD ["/usr/local/bin/slasyz_ru"]
