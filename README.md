# one-billion-row-challenge

Initial source: [1brc by gunnarmorling](https://github.com/gunnarmorling/1brc)

## The challenge

Initially, `The One Billion Row Challenge (1BRC)` was devoted to fun exploration of how fast Java programs can handle 1 billion rows.
And the challenge goes far beyond the java-only state, with the implementations in a variety of languages and frameworks. 
[Here](https://github.com/gunnarmorling/1brc/discussions/categories/show-and-tell) is the list of all implementations

## The Task

> The task is to write a Java program which reads the file, calculates the `min`, `mean`, and `max` temperature value per weather station, and emits the results on stdout like this (i.e. sorted alphabetically by station name, and the result values per station in the format `<min>/<mean>/<max>`, rounded to one fractional digit)

Input lines example 
```csv
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0
```

Output line:
```
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, ...}
```

## How to test your implementation

1. Generate test data (1 billion rows)
> $> python3 create_measurements.py 1_000_000_000

It will take about ~2 minutes to complete and produce a ~15Gb size file in data/ folder (of course depending on your machine)

2. Implement your code in any prefered language/framework

3. Measure the performance of your implementation
> time $your_programm_command data/measurements.txt
