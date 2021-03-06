## Ubuntu

`sudo apt install postgresql -y`

Note: No need to `apt install postgresql-contrib` or `postgresql-client`. No need to run `pg_ctlcluster 12 main start`.

Change authentication to md5 for postgres and all users as descibed here https://ubuntu.com/server/docs/databases-postgresql.

```shell
PGPASSWORD='postgres_password' ./database_and_user.sh
./populate.sh
```

You can check status with `sudo systemctl status postgresql.service`

- https://help.ubuntu.com/community/PostgreSQL#Installation
- https://skmcloughlin.github.io/configuring_postgresql_for_wsl2
- https://www.reddit.com/r/bashonubuntuonwindows/comments/glds15/how_to_use_postgres_with_wsl2/

## Ubuntu WSL2

`sudo apt install postgresql -y`

```shell
./start.sh
./database_and_user.sh
./populate.sh
```

You can run `./status.sh` to see db status.

http and PostgreSQL servers have to be run in WSL2.

## Windows portable

Install from zip http://www.enterprisedb.com/products/pgbindownload.do

Link via https://www.postgresql.org/download/windows/

Unpack zip and extracted `/path/to/psql/bin` to `PATH` env.
