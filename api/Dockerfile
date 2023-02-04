FROM rust:1.67 as builder

RUN USER=root cargo new --bin soundiboard
WORKDIR ./soundiboard
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/soundiboard*

ADD ./src ./src
ADD ./migrations ./migrations
ADD ./sqlx-data.json ./

RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    ROCKET_PORT=8000 \
    ROCKET_ADDRESS=0.0.0.0

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /soundiboard/target/release/soundiboard ${APP}/soundiboard

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./soundiboard"]
