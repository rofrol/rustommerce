#!/bin/bash

initdb -D ~/pgdata -U postgres -A trust -E UTF8 --locale=en-US
pg_ctl -D ~/pgdata -l ~/pg.log start
