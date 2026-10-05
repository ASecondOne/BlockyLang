# Getting started

## What you need

- The `blocky_lang` program (from this repo: `cargo build`, then use `target/debug/blocky_lang`).
- A **Blocky project**: a folder that contains `blocky.toml`.

Commands look for `blocky.toml` in the **current directory**. They do not take a project path argument.

## Project layout

```text
my_project/
  blocky.toml
  src/
    main.block
```

### `blocky.toml`

Created by `init`. Typical contents:

```toml
[unimportes]

[imports]
```

These sections are **not used** yet. The file only marks the folder as a Blocky project.

### `src/*.block`

Every file under `src/` with the `.block` extension is loaded and run as **one combined program**. Prefer a single `main.block` for now.

Example:

```blocky
<execute>
    println "start";

    if true, {
        println "true works";
    }

    println "end";
</execute>
```

## CLI commands

### `init`

```bash
blocky_lang init
```

If the current directory has **no** `blocky.toml`:

1. Writes `blocky.toml`.
2. Creates `src/`.
3. Writes `src/main.block` with:

```blocky
<execute>
    println "Hello Block";
</execute>
```

4. Prints where the project was made.

If `blocky.toml` already exists, prints `Project already exists` and does nothing.

### `run`

```bash
blocky_lang run
blocky_lang run --debug
```

Requires `blocky.toml` in the current directory. Otherwise prints `Project not found`.

Reads every `src/*.block` file, then parses, checks, and runs the program. On failure it prints an error and exits with code `1`.

Any second argument other than `--debug` prints an unknown-argument error and exits `1`.

### `--debug`

With `run --debug`, before normal output you see:

1. A dump of the parsed program structure.
2. A dump of the program after names and keywords are resolved.
3. A separator, then the program’s own `println` output.

### Unknown commands

Anything other than `init` or `run` as the first argument prints `Unknown Command: ...`.

## Running the sample project

From the repo:

```bash
cargo build
cd test_foulder
../target/debug/blocky_lang run
```

## Current limitations

- `blocky.toml` import settings are ignored.
- Multiple `.block` files are merged into one program.
- There is no `--help` flag.
