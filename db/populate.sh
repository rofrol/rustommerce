#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

export $(cat ${DIR}/../.env | xargs)

psql -U ${DBUSER} -d ${DBNAME} -f populate.sql
