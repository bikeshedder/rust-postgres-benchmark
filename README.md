# Benchmark of rust-postgres

This repository contains benchmarks for `postgres` and `tokio-postgres`

## Installation and usage

    $ createdb mydb
    $ psql mydb < database.sql
    $ cp .env.example .env # edit this file afterwards
    $ bash benchmark.sh

## Results

Running this benchmark on a DELL XPS 13 with i7-8550U CPU @ 1.80GHz, 16 GiB of RAM and a SSD using Fedora 29 with PostgreSQL 10.10 yields the following results:

### postgres-0.15.2
```
Elapsed time: 65057 ms
Performance: 15371 req/s
```

### postgres-0.16-rc.2
```
Elapsed time: 134766 ms
Performance: 7420 req/s
```

### tokio-postgres-0.3.0
```
Elapsed time: 141756 ms
Performance: 7054 req/s
```

### tokio-postgres-0.4.0-rc.3
```
Elapsed time: 91957 ms
Performance: 10874 req/s
```

### tokio-postgres-0.5.0-alpha.2
```
Elapsed time: 83041 ms
Performance: 12042 req/s
```

### tokio-postgres-0.5.0-09a63d6
```
Elapsed time: 86778 ms
Performance: 11523 req/s
```
