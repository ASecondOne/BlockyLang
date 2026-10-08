# Keywords

Keywords can be called by short name or full path. Calls accept space, parentheses, and dot notation; `receiver.method()` passes the receiver as the first argument. See [syntax.md](syntax.md).

## `if`

**Full path:** `i_core::conditons::if`

Takes a boolean and a closure. Runs the closure when the boolean is `true`; does nothing when it is `false`. A typed but unset boolean is an error.

```blocky
<execute>
    if true, {
        println "runs";
    }
    if false, {
        println "does not run";
    }
</execute>
```

## `print` and `println`

**Full paths:** `i_core::stdout::print`, `i_core::stdout::println`

Each takes a value or variable. `print` writes without a newline; `println` adds a newline. An unset value displays as `Undefined`.

```blocky
<execute>
    print "Hello ";
    println "Blocky";
    println 42;
</execute>
```

## `inc_one`

**Full path:** `i_core::math::inc_one`

Takes a number value or a variable containing a number and returns a new number one greater. It does not mutate a variable by itself; assign the result back to update one. An unset number is an error.

```blocky
<define>
    let count = 4;
</define>
<execute>
    println inc_one 5;
    count.set_AcMods("mutabl");
    count = inc_one count;
    println count;
</execute>
```

## `let`

**Full path:** `i_core::datastore::var::let`

Declares one variable and is allowed only in `<define>`. The first argument is the variable name. An optional second argument after a comma declares its type: `String`, `Number`, or `Bool`. An initializer may follow the type. Empty declarations initialize to `Undefined` or an unset typed value, consuming the internal one-time initialization permission.

```blocky
<define>
    let unset;
    let answer = 42;
    let title, String;
    let retries, Number = 3;
</define>
```

## `mut`

**Full path:** `i_core::datastore::access_modifires::mut`

Takes one variable and adds the ongoing `Mutabl` modifier without replacing any other modifiers. It is allowed in `<define>` and `<execute>` blocks. `OneTimeMutabl` is reserved for initialization.

```blocky
<define>
    let count, Number = 1;
    mut count;
</define>
<execute>
    count = 2;
</execute>
```

## `type`

**Full path:** `i_core::datastore::var::type`

Returns the variable/value kind as a string: `String`, `Number`, `Boolean`, or `Undefined`. Typed unset values report their declared kind.

```blocky
<define>
    let text = "String".new();
</define>
<execute>
    println text.type();
    println type 42;
</execute>
```

## `origin`

**Full path:** `i_core::datastore::var::origin`

Returns the variable's declaration path as a string, typically `define/name`.

```blocky
<define>
    let answer = 42;
</define>
<execute>
    println answer.origin();
</execute>
```

## `new`

**Full path:** `i_core::value::new`

Takes a string naming a type and returns an unset value of that type. Supported names are `String`, `Number`, and `Bool` (case-insensitive). The dot form is convenient because the type string is passed as the receiver: `"String".new()`.

```blocky
<define>
    let text = "String".new();
    let count = "Number".new();
    let enabled = "Bool".new();
</define>
```

The returned value displays as `Undefined` until assigned. Assigning into a typed unset variable still requires its matching type; grant `Mutabl` before a later assignment.

## `get_AcMods` and `set_AcMods`

**Full paths:** `i_core::datastore::access_modifires::get_AcMods`, `i_core::datastore::access_modifires::set_AcMods`

`get_AcMods` returns the access modifiers as a string. `set_AcMods` takes a variable followed by modifier strings; `"mutabl"` enables ongoing assignment. `OneTimeMutabl` is an internal initializer permission and is not set through this keyword.

```blocky
<define>
    let count = 1;
</define>
<execute>
    count.set_AcMods("mutabl");
    println count.get_AcMods();
    count = 2;
</execute>
```

## `transfer`

**Full path:** `i_core::datastore::var::transfer`

Takes one variable and must run inside a closure. It copies the local variable's current value and access modifiers to its immediate origin. In nested closures, transfer one level at a time.

```blocky
<define>
    let count = 1;
</define>
<execute>
    count.set_AcMods("mutabl");
    if true, {
        count = 2;
        transfer count;
    }
    println count;
</execute>
```

See [closures-and-scope.md](closures-and-scope.md) for local-copy behavior and nested transfers.

## Current limits

- Keywords are built in; user-defined keywords are not supported.
- `set_AcMods` currently recognizes only the string `"mutabl"`; `mut name;` is the short form for adding it.
