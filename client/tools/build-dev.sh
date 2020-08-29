#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR/..

bash tools/build-non-elm.sh && \
bash tools/elm-make-dev.sh
