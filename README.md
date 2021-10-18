# Atlantis

Automating Atlantis' pearl processing production pipeline for producing pristine pearls.

## How to run

First, make sure the Rust programming language is [installed on your computer](https://www.rust-lang.org/tools/install).

Then clone the repo to a local directory

```bash
git clone https://github.com/Matyasz/atlantis.git
```

Next navigate into the repo and build the project

```bash
cd atlantis
cargo build --release
```
This will take about 30 seconds on an average laptop (8th gen Intel CORE i5 running Manjaro Linux). You can omit the `--release` flag to speed up compile time, but this will significantly slow down runtime.

From this same directory, you can run the tests with 

```bash
cargo test
```

This will produce a binary, which will be located at `<repo root dir>/target/release/atlantis` (replace `release` with `debug` if you omitted the `--release flag` earlier) which can accept inputs from `stdin` and will output instructions to `stdout`.

## Optimization Strategy

I have implemented a very basic strategy for the pearl processing pipeline, based on a few rules.
1. Nautiloids always prioritize passing finished pearls back to the gatekeeper.
2. If they do not have a fully processed pearl, then the nautiloid will check to see if it it a good idea to pass on of their pearls to a neighboring nautiloid. The nautiloid will only pass a pearl to a neighbor with an *empty desk* who meets one of the following conditions:
    - The neighbor can process the pearl *faster* than the worker currently in possession of it.
    - The neighbor can process the pearl *as fast as* the worker currently in possession of it AND the worker with the pearl has more than one pearl so that after passing they can both work to process pearls.
3. If a nautiloid also does not have a viable neighbor to pass a pearl to, then they will nom one of their pearls. In doing so, they will prioritize passing the pearl which they can process fastest.

Using this method, the score from an `average-run` seems to hover between 9.5 and 11.

### Advantages

- This algorithm is simple and easy to implement and understand
- The way the `ability_map` is read from a `.json` file is both general in how we could add more flavors and colors in the future, and also pulls out these constants from the code and stores them in an appropriate filetype improving code maintainability.
- Debugging is made easier by the fact that it is always clear what action a nautiloid must take

### Disadvantages

- Being so simple, this algorithm leave plenty of room for nuance and optimization, all detailed below in the `Improvements` section

## Bonus documentation

Thanks to the magic of Rust, you can also build a complete set of documentation as a website (complete with hyperlinks and including the documentation of all dependencies) by running

```bash
cargo doc --open
```

When it is finished generating, it will automatically open in your browser. Later, you can navigate to `<repo root dir>/target/doc/atlantis` where you will find an `index.html` containing the landing page for the documentation.

## Assumptions

This code comes with a few assumptions about the input data, documented here.

- All IDs (for both nautiloids and pearls) will be nonnegative integers. For these, I have chosen to represent as type `u32` in the code.
- Similarly, the layer thicknesses must also be a nonnegative integer, `u32`.
- All IDs are unique.

## Improvements

The code has a handful of areas that could be improved, documented here.

- In a production setting far more examination of the input data would be done,here most of the input is assumed to be valid.
- The `get_neighbor_map` could be significantly more efficient.
- `unwrap` is likely used too often. Better error handling by using more idiomatic Rust for many of these instances could have been implemented.
- More methods should return an `Option` type, again leading to more idiomatic Rust code.
- A number of methods in `pearl_pipeline` could be broken out and made into more, smaller, more testable functions.
- More rules could be added to the optimization of the pearl processing pipeline. Some that come to mind are:
    1. When passing, find the shortest path to a nautiloid that can process one of their pearls the fastest.
    2. Instead of keeping track of the path the pearls take and passing them back that way, just determine the shortest path back to the gate keeper.
    3. Weight the decision to pass based on how far from the gatekeeper a worker and their neighbor are, knowing that if you pass to a worker further away from the gatekeeper then it will take longer to get back.
    4. Have workers prioritize passing if they have sufficiently more pearls than their neighbor. This would keep the distribution of pearls roughly even across the pipeline, possibly preventing workers far from the gate keeper from sitting idle for too long.
    5. Any decisions based on "number of pearls" could be replaced with "total time to process all pearls currently on the desk" instead.
    6. Maybe move pearls based on who can process the outermost layer fastest, not just who can process the whole pearl the fastest.
    7. Honestly this whole problem screams graph theory, but no obvious algorithm jumped out at me from the start. There is likely a much more efficient method based on some sort of modified travelling salesman problem, where all the travel routes are the same length, and the salesman actually has different tasks to perform in each city before returning home and different amounts of support in each city. And also there are a lot of salespeople doing these tasks. I bet the USPS has solved this problem.