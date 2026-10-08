# Syntax

Blocky source lives in `.block` files.

## Blocks

A block is an HTML-like tag pair. The tag name is the block type:

```blocky
<execute>
    println "hi";
</execute>
```

```blocky
<define>
    let x;
</define>
```

Rules:

- Opening and closing tags must match (`<execute>` … `</execute>`).
- You cannot open a new block before closing the current one.
- Only known block types are allowed (`define` and `execute` today). Unknown tags fail before the program runs.

See [blocks.md](blocks.md).

## Lines and `;`

Inside a block, each statement ends with `;`. Inside a closure `{ ... }`, `;` also separates statements.

Empty lines are ignored.

```blocky
<execute>
    println "one";
    println "two";
</execute>
```

## Keyword calls

A call has a keyword name and zero or more arguments.

### Space form

```blocky
println "hello";
if true, { println "yes"; }
```

The first space separates the keyword from its arguments. Arguments are split on commas.

### Parentheses form

```blocky
println("hello");
println type(x);
n = inc_one(5);
```

`foo(bar, baz)` is a call. **Parentheses are not for grouping.** This fails:

```blocky
println (5);
```

### Nested calls

Pass a call as an argument using space form or `foo(bar)` (no extra grouping parens):

```blocky
println type x;
println type(99);
println inc_one 5;
println inc_one(5);
```

### Full keyword path

You can use the short name (`println`) or the full path:

```blocky
i_core::stdout::println "via path";
```

### Dot form

`left.right(...)` puts `left` as the **first argument** of `right`:

```blocky
x.println();
println x.type();
println "hello".type();
```

Dot calls also work on string literals. The receiver is passed as the first argument, so `"String".new()` is equivalent to `new "String"`.

## Literals

| Form | Meaning |
|------|---------|
| `"text"` | String |
| `42`, `-1` | Number |
| `true` / `false` | Boolean |

Anything else that is not a keyword call is treated as a **variable name**.

## Assignment `=`

`=` assigns the right-hand side into the left-hand side (usually a variable):

```blocky
<define>
    let x;
</define>
<execute>
    x = 10;
    x = "hi";
    x = inc_one 5;
</execute>
```

- Assigning into a variable stores a value.
- Assigning into a bare value (for example `5 = 10`) fails.
- Variables have a one-time initialization allowance. Later assignments require the `Mutabl` access modifier and must preserve the variable's value kind.

In `<define>`, give a variable an explicit type with a comma and optionally add `mut` to permit ongoing assignment:

```blocky
<define>
    let name, String = "Blocky";
    mut name;
</define>
```

## Closures `{ ... }`

Braces group statements into a closure. The body is not run until something (usually `if`) runs it:

```blocky
if true, {
    println "first line in closure";
    println "second line in closure";
}
```

See [closures-and-scope.md](closures-and-scope.md).

## Current limitations

- No math or comparison operators beyond assignment `=` and call/dot syntax.
- No grouping parentheses.
- A bare `println;` (no argument) is not treated as a call.
- There is no comment syntax.
