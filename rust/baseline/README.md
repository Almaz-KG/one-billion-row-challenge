## A rust baseline implementation

> time cargo run -p baseline -r -- ../data/measurements.txt  

*188.38s* user 70.51s system 76% cpu 5:40.63 total

### Details
Read the content of the file into a single string variable, split it up into lines by (\n) symbol, and iterate over the line adding each line content into hashmap 