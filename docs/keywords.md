# Keywords

You can call a keyword by its **short name** (for example `println`) or by its **full path** (for example `i_core::stdout::println`).

---

## `if`

**Full path:** `i_core::conditons::if`  
(The path really is spelled `conditons` — the short name `if` still works.)

**Arguments:** exactly 2

1. A condition: `true` or `false`.
2. A closure `{ ... }` — run only when the condition is true.

**Behavior:** If the condition is `true`, runs the closure. If `false`, does nothing.

**Errors:**

- Wrong number of arguments: `` `if` requires a condition and a closure ``
- Condition not a boolean: `` `if` condition has to be `true` or `false` ``
- Second argument not a closure: `` `if` requires a closure as its second argument ``

**Example:**

```blocky
<execute>
    if true, {
        println "yes";
    }
    if false, {
        println "no";
    }
</execute>
```

```text
yes
```

Nested `if`:

```blocky
<execute>
    if true, {
        if true, {
            println "nested true works";
        }
        if false, {
            println "nested false should not print";
        }
    }
</execute>
```

---

## `println`

**Full path:** `i_core::stdout::println`

**Arguments:** one value or variable to print

**Behavior:** Prints the argument followed by a newline.

| Argument | Printed as |
|----------|------------|
| String | the text |
| Number | digits |
| Boolean | `true` / `false` |
| Variable | its current value (or `Undefined` if unset) |

**Errors:**

- Missing argument (when actually called with no args): `` `println` requires one argument ``
- Cannot display the argument: `` `println` could not display its argument ``

**Examples:**

```blocky
<execute>
    println "hello block";
    println 42;
    println true;
</execute>
```

```text
hello block
42
true
```

```blocky
<define>
    let x;
</define>
<execute>
    x = 10;
    println x;
</execute>
```

```text
10
```

Dot form:

```blocky
<define>
    let x;
</define>
<execute>
    x = "hi";
    x.println();
</execute>
```

```text
hi
```

---

## `inc_one`

**Full path:** `i_core::math::inc_one`

**Arguments:** one number — either a number literal, or a variable whose current value is a number.

**Behavior:** Returns that number plus one. Does **not** change a variable in place; assign the result if you want to update it (`n = inc_one n`).

**Errors:**

- Missing argument: `` `inc_one` requires one argument ``
- Argument is neither a number value nor a variable: `` `inc_one` requires a number or a variable ``
- Variable or value is not a number (for example a string): `` `inc_one` requires a number ``

**Examples:**

Number literal:

```blocky
<execute>
    println inc_one 5;
    println inc_one(5);
</execute>
```

```text
6
6
```

Variable as argument:

```blocky
<define>
    let n = 5;
</define>
<execute>
    println inc_one n;
</execute>
```

```text
6
```

Update a variable by assigning the result back (does not mutate in place by itself):

```blocky
<define>
    let n = 5;
</define>
<execute>
    n = inc_one n;
    n = inc_one n;
    println n;
</execute>
```

```text
7
```

Inside a closure (still reads the variable; still does not assign unless you write `=`):

```blocky
<define>
    let n = 5;
</define>
<execute>
    n = inc_one n;
    n = inc_one n;
    if true, {
        println inc_one n;
    }
</execute>
```

```text
8
```

A variable that does not hold a number fails:

```blocky
<define>
    let s = "hi";
</define>
<execute>
    println inc_one s;
</execute>
```

```text
Execution error: `inc_one` requires a number
```

---

## `let`

**Full path:** `i_core::datastore::var::let`

**Arguments:** one variable name

**Allowed only in** `<define>` blocks.

**Behavior:** Declares the variable so the rest of the program may use it. Does not assign a value (new variables start as `Undefined`).

**Errors:**

- Missing / invalid name: `` `let` requires a variable name ``
- Used in `<execute>`: `` Keyword `let` is not allowed in `<execute>` blocks ``
- Using a name that was never declared: `` Variables do not exist: name ``

**Examples:**

Declare only (value starts as `Undefined`):

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

Declare and assign in one line (allowed in `<define>`):

```blocky
<define>
    let a = 5;
</define>
<execute>
    println a;
</execute>
```

```text
5
```

---

## `type`

**Full path:** `i_core::datastore::var::type`

**Arguments:** one variable or value

**Behavior:** Returns a string naming the kind of value: `Boolean`, `String`, `Number`, or `Undefined`.

**Errors:**

- Missing argument: `` `type` requires a variable name ``
- Not a variable or value: `` `type` requires a variable or value ``

**Example:**

```blocky
<define>
    let x;
</define>
<execute>
    x = "hi";
    println type x;
    x = 7;
    println type x;
    println type true;
    println type 99;
</execute>
```

```text
String
Number
Boolean
Number
```

Unset variable:

```blocky
<define>
    let x;
</define>
<execute>
    println type x;
</execute>
```

```text
Undefined
```

---

## `origin`

**Full path:** `i_core::datastore::var::origin`

**Arguments:** one variable

**Behavior:** Returns a string describing where the variable was declared. After `let` in a define block, that is typically `define/<name>`.

**Errors:**

- Missing / not a variable: `` `origin` requires a variable name ``

**Example:**

```blocky
<define>
    let x;
</define>
<execute>
    println origin x;
</execute>
```

```text
define/x
```

---

## `transfer`

**Full path:** `i_core::datastore::var::transfer`

**Arguments:** exactly one variable

**Behavior:** Must be called **inside a closure**. Copies the local variable’s value back to the outer variable. Needed because assignments inside a closure normally only change a local copy (see [closures-and-scope.md](closures-and-scope.md)).

**Errors:**

- Wrong number of arguments: `` `transfer` requires one variable `` / `` `transfer` accepts one variable ``
- Outside a closure: `` `transfer` must be called inside a closure ``

**Example:**

```blocky
<define>
    let x;
</define>
<execute>
    x = 1;
    if true, {
        x = 99;
        transfer x;
    }
    println x;
</execute>
```

```text
99
```

Without `transfer`, the outer value stays the same:

```blocky
<define>
    let x;
</define>
<execute>
    x = 1;
    if true, {
        x = 99;
        println x;
    }
    println x;
</execute>
```

```text
99
1
```

---

## `get_AcMods`

**Full path:** `i_core::datastore::access_modifires::get_AcMods`

**Arguments:** one variable

**Behavior:** Returns a string describing the variable’s access modifiers. No access modifiers exist yet, so the result is an empty string — `println get_AcMods x` prints a blank line.

**Errors:**

- Missing argument: `` `get_AcMods` requires one argument ``

**Example:**

```blocky
<define>
    let x;
</define>
<execute>
    println get_AcMods x;
</execute>
```

Output: one empty line.

---

## Current limitations

- You cannot define your own keywords yet; only the list above exists.
- `get_AcMods` has nothing useful to show until access modifiers exist.
