## Use manual parsing of the temp value

Currently about 13% of processing time CPU makes bytes to float parsing. Because we know the format of the input float, we can process it in much faster way, so let's see how it impacts to the performace

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      152.90 real        82.83 user        37.71 sys
      152.27 real        86.88 user        36.13 sys
      149.75 real        86.37 user        33.57 sys
      149.19 real        87.12 user        33.81 sys
      145.95 real        85.16 user        32.50 sys
```
