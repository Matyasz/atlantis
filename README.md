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

## Bonus documentation

Thanks to the magic of Rust, you can also build a complete set of documentation as a website (complete with hyperlinks and including the documentation of all dependencies) by running

```
> cargo doc --open
```

When it is finished generating, it will automtically open in your browser. Later, you can navigate to `<repo root dir>/target/doc/atlantis` where you will find an `index.html` containing the landing page for the documentation.

### Assumptions

This code comes with a few assumptions about the input data, documented here.

- All IDs (for both nautiloids and pearls) will be nonnegative integers. For these, I have chosen to represent as type `u32` in teh code.
- Similarly, the layer thicknesses must also be a nonnegative integer, `u32`

### Improvements

The code has a handful of areas that could be improved, documented here.

- As mentioned in the docstring, the get_worker_map could be sigfnificantly more efficient.
