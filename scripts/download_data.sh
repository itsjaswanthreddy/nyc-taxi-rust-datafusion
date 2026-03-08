#!/bin/bash

mkdir -p data

for m in $(seq -w 1 12); do
    url="https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-$m.parquet"
    out="data/yellow_tripdata_2024-$m.parquet"

    if [ ! -f $out ]; then
        echo "Downloading $out ..."
        curl -L -o $out $url
    else
        echo "$out already exists, skipping."
    fi
done
