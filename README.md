# Advent of Code 2022

## New day script

A script to create a new cargo package from the [`dayN/`](dayN/) template. 

```Bash
./new_day.sh [number]
```

## Flamegraph

This is more of a note to myself, but if you're on WSL2 and want to use [cargo flamegraph](https://github.com/flamegraph-rs/flamegraph) you can use the following command where you likely need to change the `5.4.0-126` version number, as well as the `-F` frequency of often to make a sample:
 
```
PERF=/usr/lib/linux-tools/5.4.0-126-generic/perf flamegraph -F 100000 -- ./target/release/day14b
```
