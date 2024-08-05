## use of memchr crate with SIMD instructions

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
        6.20 real        60.64 user         3.18 sys
        5.73 real        60.40 user         3.12 sys
        5.82 real        60.59 user         3.21 sys
        5.73 real        60.59 user         3.12 sys
        5.73 real        60.64 user         3.14 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     