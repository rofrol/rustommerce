#!/bin/bash

DIR="$(dirname "$0")"

# You may need to create locale
# $ localedef -v -c -i en_US -f UTF-8 en_US.UTF-8
# or just change command to use available locale
# https://stackoverflow.com/questions/13115692/encoding-utf8-does-not-match-locale-en-us-the-chosen-lc-ctype-setting-requires/23273873#23273873
[ ! -d $DIR/pgdata ] && initdb -D $DIR/pgdata -U postgres -A trust -E UTF8 --locale=en_US.utf8

# Whichever method you use to start it, that user (usually postgres) has
# to own the /var/run/postgresql path and have write permissions.
# https://postgrespro.com/list/thread-id/1561020
[[ "$OSTYPE" == "linux-gnu" && ! -d /var/run/postgresql/ ]] && sudo install -d -m 0700 -o `id -u -n` -g `id -g -n` /var/run/postgresql/

pg_ctl -D $DIR/pgdata -l $DIR/pg.log start
