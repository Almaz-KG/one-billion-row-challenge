## Staticly allocate file-data memory

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      147.96 real        80.80 user        35.55 sys
      141.23 real        78.37 user        34.11 sys
      143.93 real        79.23 user        34.39 sys
      141.49 real        78.79 user        33.88 sys
      137.59 real        78.05 user        30.81 sys

```
