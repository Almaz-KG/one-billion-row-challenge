## As the keys of the hashmap almost always contains 8 bytes, is useful to store them as u64 instead of byte array [u8;8]

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      151.57 real        84.97 user        34.78 sys
      142.54 real        82.05 user        31.32 sys
      146.31 real        83.22 user        33.29 sys
      138.93 real        80.25 user        30.83 sys
      141.47 real        81.15 user        31.58 sys
```
