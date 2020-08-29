#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR/..

./node_modules/.bin/elm-make src/elm/Main.elm --output=dist/js/elm.js --yes
