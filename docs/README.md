# Blocky Lang docs

Blocky Lang is a small toy language. Programs use `<define>` and `<execute>` blocks, semicolon-separated statements, typed values, variables, keyword calls, and closures.

## Quick start

Build the interpreter, then create or open a Blocky project folder (a folder with `blocky.toml`):

```bash
cargo build
# from an empty folder:
/path/to/blocky_lang/target/debug/blocky_lang init
/path/to/blocky_lang/target/debug/blocky_lang run
```

`init` creates `blocky.toml` and `src/main.block`. `run` runs every `*.block` file under `src/`.

```bash
/path/to/blocky_lang/target/debug/blocky_lang run --debug
```

Minimal program:

```blocky
<execute>
    println "Hello Block";
</execute>
```

## Doc index

| File | What it covers |
|------|----------------|
| [getting-started.md](getting-started.md) | Project layout, CLI (`init`, `run`, `--debug`) |
| [syntax.md](syntax.md) | Blocks, lines, calls, literals, `=`, `.`, closures `{ ... }` |
| [blocks.md](blocks.md) | `<define>` / `<execute>`, which keywords each allows, run order |
| [keywords.md](keywords.md) | Every keyword: arguments, behavior, errors, examples |
| [values-and-variables.md](values-and-variables.md) | Strings, numbers, booleans, variables, `let`, assignment |
| [closures-and-scope.md](closures-and-scope.md) | Closures, local copies, `transfer`, run-once behavior |

## Current limitations (short list)

- A closure can only run **once**.
- Parentheses are for calls like `foo(bar)`, **not** for grouping — `println (5)` fails.
- `mut name;` adds ongoing assignment permission; `set_AcMods` currently recognizes only `"mutabl"`. `OneTimeMutabl` is reserved for initialization.
- `blocky.toml` `[imports]` / `[unimportes]` are not used yet.
- All `src/*.block` files are treated as one combined program.
- There is no comment syntax.
