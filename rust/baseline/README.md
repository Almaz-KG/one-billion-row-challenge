## A rust baseline implementation

> seq 5 | xargs -Iz time -a -o time.log cargo run -r -- ../../data/measurements.txt

```time.log
      317.72 real       183.08 user        63.69 sys
      314.67 real       184.55 user        63.78 sys
      311.29 real       181.88 user        65.25 sys
      302.04 real       182.33 user        60.73 sys
      307.38 real       180.75 user        64.29 sys
```

### Details
Read the content of the file into a single string variable, split it up into lines by (\n) symbol, and iterate over the line adding each line content into hashmap 