## Staticly allocate file-data memory

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       31.34 real        24.95 user         4.68 sys
       29.54 real        24.90 user         3.50 sys
       29.34 real        24.96 user         3.43 sys
       29.86 real        25.33 user         3.63 sys
       30.16 real        25.68 user         3.44 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt