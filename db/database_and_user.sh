#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR

export $(cat ${DIR}/../.env | xargs)

psql -U postgres -c "CREATE DATABASE ${DBNAME};" && \
psql -U postgres -c "CREATE ROLE ${DBUSER} LOGIN ENCRYPTED PASSWORD '$DBPASS' NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE;" ${DBNAME}
