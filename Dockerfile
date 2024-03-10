FROM rust:1.76.0 as base

RUN apt-get update -yqq && apt-get install -yqq cmake g++

WORKDIR /app

FROM base as build

RUN mkdir src; touch src/main.rs

COPY Cargo.toml Cargo.lock ./

COPY src ./src/

ENV SERVER1_ADDR 1.1.1.1:443
ENV SERVER2_ADDR 1.0.1.0:443
ENV LISTEN_PORT 9999

RUN cargo build --release

FROM base

COPY --from=build /app /app

EXPOSE 9999

CMD ./target/release/pingora-lb