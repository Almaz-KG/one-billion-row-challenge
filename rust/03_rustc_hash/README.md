## Use FxHash crate instead of std::collections::HashMap

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       34.57 real        29.35 user         3.99 sys
       34.86 real        29.99 user         3.83 sys
       35.16 real        30.41 user         3.83 sys
       34.92 real        30.05 user         3.71 sys
       34.15 real        29.68 user         3.58 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     