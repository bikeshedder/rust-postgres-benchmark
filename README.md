# Benchmark of rust-postgres

This repository contains benchmarks for `postgres` and `tokio-postgres`

## Installation and usage

    $ createdb mydb
    $ psql mydb < database.sql
    $ cp .env.example .env # edit this file afterwards
    $ bash benchmark.sh

## Results

Running this benchmark on a DELL XPS 13 with i7-8550U CPU @ 1.80GHz, 16 GiB of RAM and a SSD using Fedora 29 with PostgreSQL 10.8 yields the following results:

### postgres-0.15.2

```
Elapsed time: 58486 ms
Performance: 17098 req/s
```

### postgres-0.16-rc.2

```
Elapsed time: 119389 ms
Performance: 8375 req/s
```

### tokio-postgres-0.3.0

```
Elapsed time: 123803 ms
Performance: 8077 req/s
```

### tokio-postgres-0.4.0-rc.3

```
Elapsed time: 82047 ms
Performance: 12188 req/s
```
