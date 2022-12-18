# Advent of Code 2022

Spoilers - I strongly recommend you skip this, and try to learn for yourself.

# Motivation

Done this for a couple of years. For some reason we can no longer just take the challenge on spec...
One year we tried to visualise everything (resulting in some excessive blender gymnastics), this year - I've decided to start in Rust (given zero prior experience).

Come back at the end of Dec to see if I _finish_ in Rust...

# I want in

Sign up here: [https://adventofcode.com/2022] and drop them a few dollars if you have fun.

# Learnings and Review

As of Day16 - I like rust... it can be picky, but it is also helpful (the vscode extension is excellent).
It's fast, and better - by the time I write something that _compiles_, I've not spent hours debugging some off-by-one clobbering of memory or whatever...

I'm 2 days behind - but the puzzles have been good, it is taking me longer in a language I'm learning as I go... and the difficulty seems to ramp up the last couple of days...

```sh
for i in $(seq -w 16) ; do echo "=== Day $i ===" ; cd day$i ; time ./target/release/day$i | grep Part ; cd .. ; done 2>&1 | grep -E "Day|real"
=== Day 01 ===
real    0m0.002s
=== Day 02 ===
real    0m0.002s
=== Day 03 ===
real    0m0.002s
=== Day 04 ===
real    0m0.003s
=== Day 05 ===
real    0m0.002s
=== Day 06 ===
real    0m0.002s
=== Day 07 ===
real    0m0.003s
=== Day 08 ===
real    0m0.004s
=== Day 09 ===
real    0m0.002s
=== Day 10 ===
real    0m0.002s
=== Day 11 ===
real    0m0.015s
=== Day 12 ===
real    0m0.328s
=== Day 13 ===
real    0m0.004s
=== Day 14 ===
real    0m0.008s
=== Day 15 ===
real    0m0.494s
=== Day 16 ===
real    0m0.541s
```

One downside debug-builds can emit order-of-magnitude slower code... compare:
```
$ for i in 12 15 16 ; do echo "=== Day $i ===" ; cd day$i ; time ./target/debug/day$i ; time ./target/release/day$i ; cd .. ; done 2>&1 | grep -E "Day|real"
=== Day 12 ===
real    0m4.424s
real    0m0.336s
=== Day 15 ===
real    0m11.176s
real    0m0.496s
=== Day 16 ===
real    0m5.209s
real    0m0.538s
```
