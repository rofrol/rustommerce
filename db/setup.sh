#!/bin/bash

# You may need to create locale
# $ localedef -v -c -i en_US -f UTF-8 en_US.UTF-8
# or just change command to use available locale
# https://stackoverflow.com/questions/13115692/encoding-utf8-does-not-match-locale-en-us-the-chosen-lc-ctype-setting-requires/23273873#23273873
initdb -D ~/pgdata -U postgres -A trust -E UTF8 --locale=en-US
pg_ctl -D ~/pgdata -l ~/pg.log start
# https://www.postgresql.org/docs/12/app-pg-ctl.html
# pg_ctl -D ~/pgdata status
