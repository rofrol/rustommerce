# Rustommerce

## Client

I've created this example to have easy reference code how to use elm-lang/navigation without using hashes.

This example is based on:

- general structure taken from https://github.com/elm-lang/navigation/tree/master/examples
- the routing part from https://github.com/ohanhi/elm-taco

## How to run

To set up on your own computer, you will need `git`, `elm-0.18`, `node.js`, `yarnpkg`.

Also web browser with support of [Object.assign](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Object/assign) for loading `env.js`. There is also [polyfill](https://github.com/sindresorhus/object-assign).

Simply clone the repository and:


```bash
$ cp .env.example .env
$ ./client/tools/build-dev.sh
```

Based on https://12factor.net/config

You will get `dist/js/env.js` which is loaded to elm through flags.

Start server.

Start db.

In another terminal run:

```bash
$ ./client/tools/browsersync.js
```

Then navigate your browser to [http://localhost:8000](http://localhost:8000).

## Linter elm make - don't compile twice

https://gist.github.com/rofrol/fd46e9570728193fddcc234094a0bd99#atom-editor-and-linter-elm-make---dont-compile-twice

## Migrating to newer actix-web

- https://github.com/actix/actix-web/blob/master/MIGRATION.md
