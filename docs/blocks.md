# Blocks

A Blocky program is a list of blocks. Each block has a type that controls **which keywords are allowed** and **when the block runs**.

## Built-in block types

| Block | When it runs | Allowed keywords |
|-------|--------------|------------------|
| `<define>` | First | `let` declarations, `mut` modifiers, and the `new` value constructor used in initializers |
| `<execute>` | Second | Everything **except** `let` |

### Examples

Declare, then use:

```blocky
<define>
    let x;
</define>
<execute>
    x = 10;
    println x;
</execute>
```

Output:

```text
10
```

`let` inside `<execute>` is not allowed:

```blocky
<execute>
    let x;
</execute>
```

```text
Resolver error in `<execute>` expression 1: Keyword `let` is not allowed in `<execute>` blocks
Resolver error: Variables do not exist: x
```

`println` inside `<define>` is not allowed:

```blocky
<define>
    println "no";
</define>
```

```text
Resolver error in `<define>` expression 1: Keyword `println` is not allowed in `<define>` blocks
```

Unknown block type:

```blocky
<foo>
    println "x";
</foo>
```

```text
Resolver error: Unknown block type `<foo>`
```

## Run order

Blocks run by type order, not by file order:

1. All `<define>` blocks run first.
2. All `<execute>` blocks run next.

So this still works even though `<execute>` appears first:

```blocky
<execute>
    println x;
</execute>
<define>
    let x;
</define>
```

Output:

```text
Undefined
```

(`x` was declared but never assigned.)

A block type can also be marked “do not run at all.” None of the built-in types use that today.

## Current limitations

- Only `define` and `execute` exist.
- Allowed keywords per block are fixed; you cannot change them from `blocky.toml`.
