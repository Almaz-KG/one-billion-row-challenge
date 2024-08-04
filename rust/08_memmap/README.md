## use of memchr crate with SIMD instructions

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      22.88 real        20.67 user         2.20 sys
      22.28 real        20.34 user         1.73 sys
      22.65 real        20.64 user         1.83 sys
      22.51 real        20.52 user         1.80 sys
      23.10 real        21.13 user         1.78 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     