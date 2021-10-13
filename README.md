# Atlantis

Automating Atlantis' pearl processing production.

## How to run

First, make sure Rust is installed on your computer.

Then clone the repo to a local directory

```
> git clone https://github.com/Matyasz/atlantis.git
```

Next navigate into the repo and build the project

```
> cd atlantis
> cargo build
```

This will produce a binary, which will be located at `<repo root dir>/target/debug/atlantis` which can accept inputs from `stdin` and will output instructions to `stdout`.

## Optimization Strategy

I have implemented a very basic strategy for the pearl processing pipeline, based on a few rules.
1. Nautiloids always prioritize passing finished pearls back to the gatekeeper.
2. Nautiloids will prioritize passing if their neighbor has no pearls and they have more than one, or if the neighbor can process it quicker than them.
3. A nautiloid will not pass a pearl a neighbor that will process it slower, unless that neighbor has no pearls on their desk.
4. If they have more than one pearl on their desk and choose to process, a nautiloid will always process the pearl that they can finish the quickest.
5. If they have more than one pearl on their desk and choose to pass, a nautiloid will always pass the pearl that the neighbor can finish the quickest.
6. If a nautiloid would be willing to pass to more than one other worker, then they will prioritize by which could process one of their pearls the fastest. Then by lowest number of pearls, then by highest number of other neighbors to pass to, optimizing the chance that the new pearl holder is next to a


### Advantages

- good

### Disadvantages

- bad


## Bonus documentation

Thanks to the magic of Rust, you can also build a complete set of documentation as a website (complete with hyperlinks and including the documentation of all dependencies) by running

```
> cargo doc --open
```

When it is finished generating, it will automatically open in your browser. Later, you can navigate to `<repo root dir>/target/doc/atlantis` where you will find an `index.html` containing the landing page for the documentation.

### Assumptions

This code comes with a few assumptions about the input data, documented here.

- All IDs (for both nautiloids and pearls) will be nonnegative integers. For these, I have chosen to represent as type `u32` in the code.
- Similarly, the layer thicknesses must also be a nonnegative integer, `u32`.
- All IDs are unique.
- A worker can pass or nom any pearl on their desk, however if they choose to nom then they much nom the outermost layer of that pearl, which is assumed to be the first in the list.

### Improvements

The code has a handful of areas that could be improved, documented here.

- As mentioned in the docstring, the get_worker_map could be significantly more efficient.
- `unwrap` is likely used too often. Better error handling for many of these instances could have been implemented.
- A number of methods in `pearl_pipeline` could be broken out and made into more, smaller, more testable functions.
- Maybe move pearls based on who can process the outermost layer fastest, not just who can process the whole pearl the fastest.
- In the `determine_actions` method, `clone` was used a few times to settle a dispute with teh borrow checker. Likely a few traits could have been implemented to solve that problem without cloning any data, this would have been a more idiomatic way of working in Rust, as well as creating a lighter memory footprint.
- More rules could be added to the optimization of the pearl processing pipeline. Some that come to mind are:
    1. When passing, find the shortest path to a nautiloid that can process one of their pearls the fastest.
    2. Maybe prioritize passing pearls of a given color to the worker that can process them the quickest, keeping the pears of that color evenly distributed across workers of that flavor.
    3. Weight the decision to pass based on how far from the gatekeeper a worker and their neighbor are, knowing that if you pass to a worker further away from the gatekeeper then it will take longer to get back.
    4. Have workers prioritize passing if they have sufficiently more pearls than their neighbor. This would keep the distribution of pearls roughly even across the pipeline, possibly preventing workers far from the gate keeper from sitting idle for too long.
    5. Any decisions based on "number of pearls" could be replaced with "total time to process all pearls currently on the desk."
    6. Honestly this whole problem screams graph theory, but no obvious algorithm jumped out at me from the start. There is likely a much more efficient method based on some sort of modified travelling salesman problem, where all the travel routes are the same length, and the salesman actually has different tasks to perform in each city before returning home and different amounts of support in each city. And also there are a lot of salespeople doing these tasks. I bet the USPS has solved this problem.