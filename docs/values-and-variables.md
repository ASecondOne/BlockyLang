# Values and variables

## Values

Blocky has strings, numbers, booleans, and `Undefined`:

| Kind | Literal or constructor | Display |
|------|------------------------|---------|
| String | `"hello"` | `hello` |
| Number | `42`, `-1` | `42` |
| Boolean | `true`, `false` | `true` / `false` |
| Undefined | an empty `let`, or an unset typed value | `Undefined` |

Typed unset values can be created with `.new()`:

```blocky
<define>
    let text = "String".new();
    let count = "Number".new();
    let enabled = "Bool".new();
</define>
<execute>
    println text;
    println count;
    println enabled;
    println text.type();
    println count.type();
    println enabled.type();
</execute>
```

The first three lines display `Undefined`; the type checks display `String`, `Number`, and `Boolean`. Set a typed value with a matching value kind. To assign after its declaration initializer, grant `Mutabl` first.

## Declaring variables

Declare variables in `<define>` with `let`:

```blocky
<define>
    let name;
    let score = 10;
    let title, String;
    let retries, Number = 3;
    let ready, Bool = false;
</define>
```

The first argument is always the variable name; the optional second argument after a comma is its type (`String`, `Number`, or `Bool`). An initializer may follow the type. An untyped empty declaration initializes to `Undefined`. The internal `OneTimeMutabl` permission is used for initialization and is not a modifier to pass to `set_AcMods`.

Every variable must be declared before use. An unknown name causes a resolver error and prevents execution.

## Assigning values

Use `=` to redirect a value into a variable. The first assignment establishes the value kind for an untyped variable; a typed declaration already fixes it. Later assignments must use the same kind and require `Mutabl`:

```blocky
<define>
    let score = 10;
</define>
<execute>
    score.set_AcMods("mutabl");
    score = 20;
    println score;
</execute>
```

For a typed unset variable, the assigned value must match its declared type:

```blocky
<define>
    let title = "String".new();
</define>
<execute>
    title.set_AcMods("mutabl");
    title = "Blocky";
    println title;
</execute>
```

Assigning a different value kind is an execution error. Values themselves are not assignment targets.

## Reading and inspecting variables

`println` displays a variable's current value. The `type` keyword returns `String`, `Number`, `Boolean`, or `Undefined`; `origin` returns the declaration path, usually `define/<name>`.

```blocky
<define>
    let score = 10;
</define>
<execute>
    println score;
    println score.type();
    println score.origin();
</execute>
```

## Access modifiers

Use `mut name;` to add the `Mutabl` modifier concisely. `set_AcMods("mutabl")` is also available and replaces the modifier set. `get_AcMods` returns a string describing the current set. `OneTimeMutabl` is an internal initialization permission and should not be passed to `set_AcMods`.

```blocky
<define>
    let score = 10;
    mut score;
</define>
<execute>
    println score.get_AcMods();
</execute>
```

Inside a closure, variable changes are local unless transferred outward. See [closures-and-scope.md](closures-and-scope.md).
