#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR/..

bash tools/build-non-elm.sh && \
bash tools/elm-make-dev.sh
