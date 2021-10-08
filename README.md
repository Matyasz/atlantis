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