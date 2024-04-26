## use of memchr crate with SIMD instructions

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      130.15 real        56.96 user        40.85 sys
      127.87 real        57.34 user        37.65 sys
      121.53 real        55.07 user        36.83 sys
      124.08 real        56.75 user        38.23 sys
      125.08 real        56.56 user        38.65 sys

```
