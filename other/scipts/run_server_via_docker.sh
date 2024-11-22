#!/bin/sh
export DATA_FOLDER=/data/data/
export GAMES_FOLDER=/data/games/
export CACHE_FOLDER=/data/cache/
export IP="0.0.0.0"
export MOUNT_FOLDER=.
docker run \
-e DATA_FOLDER="${DATA_FOLDER}" \
-e GAMES_FOLDER="${GAMES_FOLDER}" \
-e CACHE_FOLDER="${CACHE_FOLDER}" \
-e IP="${IP}" \
-v "${MOUNT_FOLDER}:/data" \
-p 8415:8415 \
accessory/minus_games_server