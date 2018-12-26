#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

export $(cat ${DIR}/../.env | xargs)

psql -U postgres -c "CREATE DATABASE ${DBNAME};"
psql -U postgres -c "CREATE ROLE ${DBUSER} LOGIN ENCRYPTED PASSWORD '$DBPASS' NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE;" ${DBNAME}
