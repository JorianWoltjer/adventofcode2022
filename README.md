# Advent of Code 2022

This year I tried every challenge, while learning [Rust](https://www.rust-lang.org/).  
In the end there were only 5/50 puzzles that I **missed**:

* Day 16 B: Part A was hard enough 😅
* Day 19 A & B: Day 16 with extra steps
* Day 24 B: Should work, but gives the wrong answer. Probably some edge case
* Day 25 B: Only unlocked after completing all 49 other puzzles

I know that if I want to complete all the challenges next year, I'll need to learn more about [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) and how to optimize it. I also realized that the [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) algorithm is very important to know well, as I used it in quite a few solutions. Other than that, it was pretty successful for only having started with Rust a few months ago. 

As the puzzles got harder each day, I eventually started making drawings for a few ([15](day15/), [16](day16/), [18](day18/), [21](day21/), [22](day22/)) to test ideas visually instead of trying to imagine it. In some days you'll see a **drawing** that makes the puzzle or idea more understandable. 

## Highlights

Some of my most clean and favorite solutions:

* [Day 6](day6/src/lib.rs): Highly optimized buffer reader that would work with a real stream as well
* [Day 7](day7/src/lib.rs): A recursive data structure using `Rc<RefCell<>>` which wasn't easy in Rust
* [Day 12](day12/src/lib.rs): First time using Breadth-first search and clean code overall
* [Day 13](day13/src/lib.rs): A very clean and *rust*y `compare()` function that is fast and minimal
* [Day 16 A](day16/src/lib.rs): After over 6 hours of working on this puzzle, I finally solved it with a big recursive algorithm and some shortcuts
* [Day 23](day23/src/lib.rs): Very simple/short code to solve the whole puzzle with a neat observation
* [Day 24 A](day24/src/lib.rs): A very efficient algorithm that finds the answer in only 60 milliseconds
* [Day 25 A](day25/src/lib.rs): A logical and simple to follow algorithm with detailed comments

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
