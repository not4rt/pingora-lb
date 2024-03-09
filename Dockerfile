FROM rust:1.76.0 as base

RUN apt-get update -yqq

WORKDIR /app

FROM base as build

RUN mkdir src; touch src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src ./src/

RUN cargo build --release

FROM base

COPY --from=build /app /app

EXPOSE 9999

CMD ./target/release/backend