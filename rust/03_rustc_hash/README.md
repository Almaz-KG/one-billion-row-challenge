## Use FxHash crate instead of std::collections::HashMap

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      185.01 real       108.19 user        41.36 sys
      171.88 real       105.97 user        35.48 sys
      172.25 real       105.84 user        37.10 sys
      172.83 real       105.29 user        35.16 sys
      177.27 real       108.27 user        37.49 sys
```
