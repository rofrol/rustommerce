#!/bin/bash

# You may need to create locale
# $ localedef -v -c -i en_US -f UTF-8 en_US.UTF-8
# or just change command to use available locale
# https://stackoverflow.com/questions/13115692/encoding-utf8-does-not-match-locale-en-us-the-chosen-lc-ctype-setting-requires/23273873#23273873
[ ! -d ~/pgdata ] && initdb -D ~/pgdata -U postgres -A trust -E UTF8 --locale=en_US.utf8
pg_ctl -D ~/pgdata -l ~/pg.log start
