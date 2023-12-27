I set a goal for myself to solve this year's Advent of Code (2023) in under a second using Rust. This has been done by [other](https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/) [people](https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/) in prior years, and I know there were others out there this year who were similarly optimizing their solutions. In particular, I found some like-minded developers in [Chris Biscardi's](https://github.com/ChristopherBiscardi) Discord server who were also optimizing AOC2023 using Rust. Many thanks to those who shared their ideas!

In the end, I managed to solve all of 2023 in [just under 31ms](https://github.com/kcaffrey/aoc2023?tab=readme-ov-file#benchmarks) (solutions benchmarked separately and then summed together). The two main challenges along the way were finding the right algorithms and data structures, and then optimizing my implementations. The latter was more important than I realized up front; I was often able to achieve a 5x or better speed improvement with no algorithmic changes by optimizing the implementation. I'll discuss some of the general optimization techniques first, then highlight some of the algorithmic choices for individual days.

# Ground Rules
1. 100% safe Rust[^1]. Part of my goal was to learn optimization techniques I could apply in day-to-day usage. On certain days optimizations were available to use unsafe memory access to skip bounds checking, but these are not optimizations I would use on anything but the most performance-sensitive code, and even there I would think twice before using `unsafe`.
2. Solutions should work for any users input. Some days (8, 20, and 21) involved some reverse-engineering and guessing at valid assumptions about the input, so this rule was more of a guideline, but when possible I tried not to make undue assumptions about my input.

[^1]: Some of [my dependencies](https://github.com/kcaffrey/aoc2023/blob/e62f0bc380bdd9f2d1fb93df937f94bba4d2c986/Cargo.toml#L29-L37) (like Rayon) use unsafe, but when possible I used crates that were 100% safe (like using [`tinyvec`](https://github.com/Lokathor/tinyvec) instead of [`smallvec`](https://github.com/servo/rust-smallvec)).

# General Optimization Tips

1. Memory is slow. Reducing allocations by reusing buffers or reorganizing data structures can make a huge difference. I used [DHAT](https://github.com/nnethercote/dhat-rs) to profile allocations and find opportunities for improvements on many days.
2. Memory is really slow. Some data structures may have better asymptotic performance, but often a plain array (or `Vec`) offers better practical performance, due to cache locality and other similar reasons.
3. Hash sets and maps may have O(1) insertion and retrieval, but the hashing operations are expensive. This means the constant factors can be quite high. If the number of elements is small, a `Vec` can outperform a `HashSet` even if you have to scan every element on insertion and retrieval. Likewise, a `HashSet<usize>` can be replaced with `Vec<bool>` if the maximum value I am inserting is reasonably small. Swapping hash sets/maps for vectors often was a 2-3x speedup. If these optimizations were not possible (due to the domain of the items I am hashing being too large, for example), I found that [fx-hash](https://github.com/cbreeden/fxhash) (a drop-in replacement for the built-in hash sets/maps) offered large performance gains.
4. String operations in safe Rust have guardrails preventing you from shooting yourself in the foot on utf-8 inputs (not all characters in utf-8 are 1 byte wide!). However, I assumed that in AoC, all inputs would be encoded in ASCII. For problems where parsing was the bottleneck, operating on `&[u8]` instead of `&str` offered modest performance improvements (at the cost of ergonomics).
5. Reduce instructions in the hot path. For instance, some days involved search algorithms where either DFS or BFS could be used interchangeably; on these days using a queue (`VecDeque`) for BFS involves more work than a stack (`Vec`) as the queue needs to handle the complexity of wrapping around in the underlying vector. Perhaps BFS is a more natural choice, but occasionally changing to DFS to use a `Vec` instead of `VecDeque` offered large gains. This nature of optimization is sometimes more art than science, as you need to keep in mind that the compiler is optimizing your code as well. I usually started with the simplest implementation and then benchmarked various ideas to improve on that. Some would pan out, others wouldn't.

# Puzzle Summaries

> [!WARNING]
> Spoilers abound below. If you are still working on some of the puzzles and don't want to read spoilers, stop here!

I'll have a short commentary for each day's puzzle below, including what I think are some key insights to solve the puzzle quickly. The focus will be primarily on algorithmic choices, but if there was some important detail in the implementation I'll call that out too. I doubt that my solutions were optimal for every day. Better solutions almost certainly exist for some puzzles, such as day 10 where I hadn't heard of the algorithm that many people used to solve it. On other puzzles, there may have been multiple different approaches that all had similar performance characteristics. This isn't intended to be a comprehensive guide, but instead a brief summary of my solutions and approaches.

## [Day 1](https://adventofcode.com/2023/day/1)

Part 1 was straightforward, but Part 2 was unexpectedly tricky for a day 1 puzzle. The potential for numbers to "overlap" in the input (such as "oneight") was the main challenge. Luckily, there is a [linear approach](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/01.rs) that sidesteps the problem entirely: Keep a list of strings to match against (e.g. `["one", "two", ...]`), and then for every character in the input, see if the substring starting at that character starts with one of those strings. Find the first and last match, and then you're done. Optimizations include early termination and searching from either end (to skip most of the middle of the string). While I didn't implement this, a [trie](https://en.wikipedia.org/wiki/Trie) or [FST](https://en.wikipedia.org/wiki/Finite-state_transducer) likely would have been even better for matching against the known set of strings.

## [Day 2](https://adventofcode.com/2023/day/2)

This was mostly an exercise in parsing. I could have written a faster solution that skipped utf-8 validation and used byte arrays instead of strings, but I only started doing that in later days. The primary optimization [I applied](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/02.rs) was solving each line as I went, eliminating the need for any memory allocations.

## [Day 3](https://adventofcode.com/2023/day/3)

Another fairly straightforward puzzle (as expected in the early days). [My solution](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/03.rs) scanned the input top to bottom and left to right looking for numbers. When a number was found and I encountered the first non-numeric character following the number (or the end of the row!), I looked at all the characters surrounding the number to find symbols. From there, the rest of the solution was straightforward. There was a possible optimization I didn't pursue on part 2 where I looked for gears instead and only scanned for numbers that touched the gears. This seemed like it would reduce the amount of work performed a fair amount, but also was more complicated and not needed for the ~40-50µs it may have saved.

## [Day 4](https://adventofcode.com/2023/day/4)

Part 1 was another exercise in parsing, but part 2 was a nice twist. [I ended up](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/04.rs) keeping a vector that denoted how many copies of each scratch card existed. If scratch card `i` had `n` winning numbers and `c` copies in the vector, then I would add `c` to the `i+1` through `i+n`th elements of the vector. The answer was the sum of each entry in that vector at the end.

## [Day 5](https://adventofcode.com/2023/day/5)

This was the first "hard" day of the year. Brute force was reasonable for part 1, but too slow[^2] for part 2. The key to part 2 was to [operate on ranges directly](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/05.rs), rather than keeping track of individual seeds. Each pair of numbers in the seed input would result in a range of seeds. Then each range within a map applied to a seed range could result in a maximum of 3 output seed ranges (some portion of the range that comes before the mapping, the mapped portion, and whatever is left afterwards). Because of this, the total number of seed ranges after applying all the mappings is linear in the size of the input. My solution keeps track of a vector of seed ranges (starting with a single element), and applies mappings one by one to expand the vector of seed ranges. At the end, I can take the minimum "start" value of the ranges.

[^2]: Brute force still worked to solve the puzzle, but it would have blown the runtime budget by a large margin.

## [Day 6](https://adventofcode.com/2023/day/6)

An interesting puzzle with a [constant time mathematical solution](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/06.rs). Solving the equations of motion for the boat and setting up an inequality to beat the best distance from the other boats results in a quadratic equation as a function of "charge time". The quadratic formula provides the answer after that.

## [Day 7](https://adventofcode.com/2023/day/7)

Nothing fancy for day 7, but getting a working solution involved sidestepping some potential pitfalls from edge cases. One implementation detail that I used to optimize [the solution](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/07.rs) was recognizing that a hand could only contain 5 cards, so there was no need for a dynamic vector. Instead, a fixed-size array (which in Rust can be stack allocated) was sufficient. Thus, the only allocation was collecting all the hands into a vector to sort them.

## [Day 8](https://adventofcode.com/2023/day/8)

The first "reverse engineering of the input" day! The solution hinged on recognizing that every ghost would eventually enter a cycle (and if a solution existed, their cycle would contain one of the destinations[^3]). Most people (including [myself](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/08.rs)) assumed that the length of the cycle was the same as the number of steps to reach the cycle, and took the LCM of the latter values. A more general solution (which I later coded up as "part 3") would allow the starting offsets to differ from the cycle length (requiring the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem) to solve). The most general solution (which I didn't bother with) would allow for multiple destinations within a ghosts cycle. This would result in each ghost having multiple possible starting offsets and a single cycle length. Trying all permutations of the starting offsets seems like it would work, but a better general solution may exist.

[^3]: If a ghost's cycle didn't contain a destination, a solution still might exist before the ghost enters their cycle. But then the problem would be trivial.

## [Day 9](https://adventofcode.com/2023/day/9)

It seemed like there might have been some mathematical approach to this puzzle, but I simply calculated the answer directly as per the instructions. I [calculated](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/09.rs) the differences in place to avoid unnecessary allocations, and recognized that the answer was simply the sum of the final difference for each iteration (which I could keep a running total for as I went). Part 2 was the same, but with the numbers reversed. Slapping on a `.rev()` did the trick!

## [Day 10](https://adventofcode.com/2023/day/10)

Part 1 was a slog of parsing and special cases, but nothing overly difficult. I made my life easier by inferring what pipe the starting position must have been, making it trivial to walk along the loop and count the steps until I returned to the start. Part 2 was a larger challenge. I later found out that something called the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) exists which can easily calculate the area of a closed polygon (such as exists in this puzzle), so long as you are careful about self overlaps. At the time, though, I had never heard of this. Therefore, I implemented [flood fill](https://github.com/kcaffrey/aoc2023/blob/086ae68bbe2a015728b8eb3fd624c788a69d2209/src/bin/10.rs#L15). It was a giant mess, but I was careful about keeping track of the "inside" region so as to make the flood fill efficient. Later I saw someone on Reddit talking about line scanning, and it clicked that I know the cell that would be one column to the left of the first column is always in the exterior region, and every time you cross a pipe you flip from outside to inside and vice versa. You can scan column by column and row by row and add up all the cells that are inside the pipe loop by this method. I tried that, got the wrong answer, and then sat down with a pen and paper until I realized that you have to exclude pipes turning in one of the two of the north/south directions for it all to work out. The result was [much shorter](https://github.com/kcaffrey/aoc2023/blob/d4d8323b3cbfd4977490baad8ff0669852ef2332/src/bin/10.rs#L13) (and over 2x faster, to boot)!

## [Day 11](https://adventofcode.com/2023/day/11)

Another day where part 1 is straightforward, but the numbers are "too large" to brute force part 2. [My solution](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/11.rs) kept track of how many galaxies were in each row and column, then walked through the rows (and likewise for the columns) to produce row and column coordinates for each galaxy, adding a running offset as I went to account for the expansion. The trick I applied beyond this was using a linear solution to find the sum of distances rather than quadratic, after applying some algebra and iterating over the galaxies in a careful order to make sure that the coordinates were sorted implicitly.

## [Day 12](https://adventofcode.com/2023/day/12)

Brute force worked for part 1, but dynamic programming is required for part 2. It was a particularly tricky problem to solve, with a complex recurrence relation. The natural solution involved keeping track of the length of the current run of damaged springs when recursing, but this increased the state space (and reduced the effectiveness of memoization). [My solution](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/12.rs) involved changing the recurrence to implicitly track whether runs of damaged springs matched the required counts as well as searching bottom-up iteratively instead of top-down recursively.

## [Day 13](https://adventofcode.com/2023/day/13)

I didn't use any special tricks for this puzzle. Part 2 was very similar to part 1, except it involved finding a mirroring where "error count" is 1 instead of 0. I [performed the search](https://github.com/kcaffrey/aoc2023/blob/main/src/bin/13.rs) in a way that would early terminate after the second error was found (in Rust, this was with an iterator that counted column errors for a row split where I used `.take(2).count() == 1; the second error would let us fail immediately).
