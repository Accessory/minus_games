#!/bin/sh
export GAMES_FOLDER="$PWD/target/games/"
export DATA_FOLDER="$PWD/target/data/"
cargo run --release --bin minus_games_finder