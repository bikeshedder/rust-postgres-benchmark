# Benchmark of rust-postgres

This repository contains benchmarks for `postgres` and `tokio-postgres`

## Installation and usage

    $ createdb mydb
    $ psql mydb < database.sql
    $ cp .env.example .env # edit this file afterwards
    $ bash benchmark.sh

## Results

Running this benchmark on a i9-9900K CPU, 128 GiB of RAM and a SSD using Ubuntu 22.04 with PostgreSQL 15.1 yields the following results:

### postgres-0.16-rc.2
```
Running 1000000 queries...
Elapsed time: 97232 ms
Performance: 10284 req/s
```

### postgres-0.17.5
```
Running 1000000 queries...
Elapsed time: 58377 ms
Performance: 17130 req/s
```

### postgres-0.19.4
```
Running 1000000 queries...
Elapsed time: 59870 ms
Performance: 16702 req/s
```

### tokio-postgres-0.5.0-alpha.2 (tokio 0.2)
```
Running 1000000 queries...
Elapsed time: 60448 ms
Performance: 16543 req/s
```

### tokio-postgres-0.5.5 (tokio 0.2)
```
Running 1000000 queries...
Elapsed time: 59747 ms
Performance: 16737 req/s
```

### tokio-postgres-0.7.7
```
Running 1000000 queries...
Elapsed time: 73286 ms
Performance: 13645 req/s
```

### tokio-postgres-0.7.7\_current\_thread
```
Running 1000000 queries...
Elapsed time: 59614 ms
Performance: 16774 req/s
```
