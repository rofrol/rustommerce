#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

source ${DIR}/../.env

psql -U ${DBUSER} -d ${DBNAME} -f populate.sql
