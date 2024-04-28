## DuckDb implementation of 1brc

```sql
-- Set temp directory (to avoid OOME)
SET temp_directory="./tmp/";

-- Load the data
CREATE OR REPLACE TABLE measurements AS
        SELECT * FROM READ_CSV('../data/measurements_1.txt', header=false, columns= {'station_name':'VARCHAR','measurement':'double'}, delim=';');

-- Run calculations
WITH src AS (SELECT station_name,
                    MIN(measurement) AS min_measurement,
                    CAST(AVG(measurement) AS DECIMAL(8,1)) AS mean_measurement,
                    MAX(measurement) AS max_measurement
            FROM measurements
            GROUP BY station_name)
    SELECT '{' ||
            ARRAY_TO_STRING(LIST_SORT(LIST(station_name || '=' || CONCAT_WS('/',min_measurement, mean_measurement, max_measurement))),', ') ||
            '}' AS "1BRC"
    FROM src;
```

> seq 5 | xargs -Iz time -a -o time.log duckdb -no-stdin -init query.sql

```time.log
      121.08 real       437.22 user       164.09 sys
      124.47 real       455.77 user       169.72 sys
      128.48 real       431.61 user       167.21 sys
      127.34 real       442.12 user       164.01 sys
      125.26 real       436.97 user       162.22 sys
```

