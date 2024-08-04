## Use manual parsing of the temp value

Currently about 13% of processing time CPU makes bytes to float parsing. Because we know the format of the input float, we can process it in much faster way, so let's see how it impacts to the performace

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       31.17 real        24.67 user         4.57 sys
       29.53 real        24.87 user         3.54 sys
       29.57 real        24.83 user         3.56 sys
       30.48 real        24.71 user         3.61 sys
       29.40 real        24.96 user         3.53 sys

```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     