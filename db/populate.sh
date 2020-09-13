#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR

export $(cat ${DIR}/../.env | xargs)

PGPASSWORD=${DBPASS} psql -U ${DBUSER} -d ${DBNAME} -f populate.sql
