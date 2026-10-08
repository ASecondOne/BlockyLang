# Closures and scope

## What a closure is

Braces create a closure — a group of statements:

```blocky
{
    println "first line in closure";
    println "second line in closure";
}
```

A closure is **not** run just because it appears as an argument. Keywords decide when (and if) it runs. That is how `if` works: it only runs the closure when the condition is `true`.

```blocky
<execute>
    if true, {
        println "first line in closure";
        println "second line in closure";
    }
</execute>
```

## Local copies inside closures

When a closure runs, variables you touch inside it become **local copies**. Assignments inside the closure change the copy, not the outer variable — unless you call `transfer`.

### Without `transfer`

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

### With `transfer`

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

`transfer` must be called **inside** a closure. Outside one it fails with `` `transfer` must be called inside a closure ``.

## Nested closures and `transfer`

Each `if` has its own closure, so nested closures get nested local states. `transfer` moves a local variable's value up exactly one level. To reach the top-level variable from a nested closure, transfer once in the inner closure and again in its parent:

```blocky
<define>
    let count = 1;
</define>
<execute>
    count.set_AcMods("mutabl");
    if true, {
        count = inc_one count;
        if true, {
            count = inc_one count;
            transfer count;
        }
        println count;
        transfer count;
    }
    println count;
</execute>
```

```text
3
3
```

The inner closure changes its copy from `2` to `3`, then its `transfer` updates the enclosing closure's copy. The second `transfer` updates the top-level variable. Without the outer transfer, the top-level value would remain `1`.

## A closure can only run once

After a closure has been run, it cannot be run again. Separate closures (for example two different `if` bodies) are fine; reusing the **same** closure a second time is not supported yet. This matters if you later add loops that try to run one closure many times.

## Current limitations

- Closures do not return a value to the caller.
- A given closure body can only run once.
- `transfer` is the only way to push a local change back outward.
