# Values and variables

## Values

| Kind | Written as | Printed as |
|------|------------|------------|
| String | `"hello"` | `hello` |
| Number | `42`, `-1` | `42` |
| Boolean | `true`, `false` | `true` / `false` |
| Undefined | (default for new variables) | `Undefined` |

```blocky
<execute>
    println "hello";
    println 42;
    println true;
</execute>
```

## Variables

### Declaring

Variables must be declared with `let` inside a `<define>` block before you use them:

```blocky
<define>
    let x;
    let n;
</define>
```

You can also assign in the same `let` line:

```blocky
<define>
    let a = 5;
    let s = "hi";
</define>
```

Using a name that was never declared fails before the program runs:

```text
Resolver error: Variables do not exist: y
```

### Assigning

Use `=` to store a value:

```blocky
<define>
    let x;
</define>
<execute>
    x = 10;
    x = "hi";
    x = true;
    x = inc_one 5;
</execute>
```

You can only assign **into a variable**. Assigning into a bare value fails.

### Reading

Pass the variable to keywords such as `println`, `type`, or `origin`:

```blocky
<define>
    let x;
</define>
<execute>
    println x;
</execute>
```

```text
Undefined
```

```blocky
<define>
    let x;
</define>
<execute>
    x = 10;
    println x;
    println type x;
    println origin x;
</execute>
```

```text
10
Number
define/x
```

### Two variables

```blocky
<define>
    let a;
    let b;
</define>
<execute>
    a = 1;
    b = inc_one 1;
    println a;
    println b;
</execute>
```

```text
1
2
```

## Inspecting values

- `type …` — returns `String`, `Number`, `Boolean`, or `Undefined`
- `origin …` — returns where the variable was declared (for example `define/x`)
- `get_AcMods …` — access modifiers (empty for now)

## Scope note

At the top level of `<execute>`, assignments change the variable itself. Inside a closure `{ ... }`, assignments usually change a **local copy** unless you `transfer` — see [closures-and-scope.md](closures-and-scope.md).

## Current limitations

- No access modifiers yet (`get_AcMods` prints blank).
- No way to undeclare or rename a variable.
