#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR/..

bash tools/clean.sh && \
npm i && \
node tools/generate-env.js && \
cp -r src/favicon.ico dist/ && \
cp -r src/images dist/ && \
cp -r src/styles dist/ && \
cp -r node_modules/normalize.css/normalize.css dist/styles && \
cp src/index.html dist
