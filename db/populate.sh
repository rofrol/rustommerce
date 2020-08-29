#!/bin/bash

DIR="$(dirname "$0")"
cd $DIR

export $(cat ${DIR}/../.env | xargs)

psql -U ${DBUSER} -d ${DBNAME} -f populate.sql
