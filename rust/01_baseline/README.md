## A rust baseline implementation

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
       65.28 real        58.77 user         4.36 sys
       69.41 real        59.99 user         6.14 sys
       67.49 real        59.97 user         5.09 sys
       64.35 real        59.38 user         3.80 sys
       64.06 real        59.50 user         3.55 sys
```

### Flamegraph
> sudo cargo flamegraph -r -- ../../data/measurements.txt     

### Details
Read the content of the file into a single string variable, split it up into lines by (\n) symbol, and iterate over the line adding each line content into hashmap 