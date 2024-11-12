FROM rust:latest as build
COPY ./ ./
RUN cargo build --package minus_games_server --release


FROM debian:stable-slim
COPY --from=build ./target/release/minus_games_server /bin/minus_games_server

CMD [ "./bin/minus_games_server" ]