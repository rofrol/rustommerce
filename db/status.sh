#!/bin/bash

DIR="$(dirname "$0")"

pg_ctl -D $DIR/pgdata status
