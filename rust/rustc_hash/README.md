## Use FxHash crate instead of std::collections::HashMap

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      197.30 real       126.66 user        36.09 sys
      184.62 real       121.83 user        33.12 sys
      190.38 real       124.92 user        34.45 sys
      175.40 real       115.40 user        32.12 sys
      191.10 real       124.15 user        34.32 sys
```
