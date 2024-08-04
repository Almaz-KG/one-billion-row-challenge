## Use bytes array instead of Strings...

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       44.48 real        37.52 user         5.14 sys
       42.96 real        37.91 user         4.19 sys
       43.72 real        38.72 user         3.86 sys
       44.17 real        38.80 user         4.24 sys
       43.44 real        38.93 user         3.71 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     