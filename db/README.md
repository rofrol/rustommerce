## Windows portable

Install from zip http://www.enterprisedb.com/products/pgbindownload.do

Link via https://www.postgresql.org/download/windows/

Unpack zip and extracted `/path/to/psql/bin` to `PATH` env.

## Init

```shell
./start.sh
./database_and_user.sh
./populate.sh
```

You can run `./status.sh` to see db status.

## http server in WSL2, PostgreSQL in Windows host

I could not connect to database.

## Ubuntu in WSL2

Haven't checked if it works.

`sudo apt install postgresql postgresql-contrib`

- https://help.ubuntu.com/community/PostgreSQL#Installation
- https://skmcloughlin.github.io/configuring_postgresql_for_wsl2
- https://www.reddit.com/r/bashonubuntuonwindows/comments/glds15/how_to_use_postgres_with_wsl2/
