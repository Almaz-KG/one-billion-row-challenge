## use of memchr crate with SIMD instructions

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       24.57 real        19.54 user         3.98 sys
       24.57 real        19.81 user         3.75 sys
       23.95 real        19.89 user         3.17 sys
       24.99 real        20.35 user         3.81 sys
       25.03 real        20.33 user         3.63 sys

```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt