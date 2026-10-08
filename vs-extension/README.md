# Blocky Lang for VS Code

This local extension adds `.block` language coloring, indentation and bracket behavior, built-in keyword completions and hover help, and static diagnostics for malformed block tags, unknown block types, unclosed strings, and unmatched braces or parentheses. Diagnostics appear in the Problems panel and inline after the affected line; an Error Lens extension is not required.

It also contributes a rust-orange gear with a `B` as the language icon for `.block` files, and type/access inlay hints after declarations when their type and modifiers are recognizable. For example, `let a = "a";` appears with `: String`; declarations show the access state at that point, so a later modifier change does not retroactively add `:M`. Modifier-changing lines such as `mut a;` and `a.set_AcMods("mutabl");` show the before-and-after access state (for example, `: Number → Number:M`). These hints never show the internal `OneTimeMutabl` modifier and do not change source text.

The diagnostics are lightweight editor checks, not a full language server or interpreter run. They do not yet resolve variable names or check every keyword argument.

## Install

From the repository root, run:

```bash
bash vs-extension/scripts/install.sh
```

The script looks for `code` or `codium`, then creates a symlink at the standard user extension location. Set `VSCODE_EXTENSIONS_DIR` to use a different extensions directory. It refuses to overwrite an existing destination. Reload VS Code after installation.

The gear-and-`B` icon is contributed for `.block` files without selecting a replacement file icon theme, so other file types keep the icons from the user's active theme. Type hints can be toggled with `blocky.inlayHints.types`; inline diagnostics can be toggled with `blocky.inlineDiagnostics`.

To remove the installation, remove the exact symlink path printed by the script. The extension source remains in this repository.

## Supported language features

- `<define>` and `<execute>` block tags
- Built-in keyword coloring, hover descriptions, and completions
- Strings, numbers, booleans, assignment, calls, dot notation, and closures
- Basic syntax diagnostics and configurable inline diagnostic decorations
- Inferred type and compact mutable-access hints for literal values, typed declarations, `.new()` values, known variables, and selected built-ins
- A language icon for `.block` files, leaving other file icons to the active theme

Blocky currently has no source comment syntax, so this extension does not color comment tokens.
