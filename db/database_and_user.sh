#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

PGDATABASE="${1}"
PGUSER="${PGDATABASE}"

psql -U postgres -c "CREATE DATABASE ${PGDATABASE};"
psql -U postgres -c "CREATE ROLE ${PGUSER} LOGIN NOSUPERUSER INHERIT CREATEDB CREATEROLE;" ${PGDATABASE}
