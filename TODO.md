# Blocky language TODO

- [ ] Separate a variable's declared type from its current value
  - Example: `let String a;`
  - The declared type is `String`, while the current value starts as `undefined`.
  - Later, reject assignments with incompatible types.

- [ ] Add a function type
  - Store function parameters and body as an expression.
  - Support declarations such as `<function input> ... </function>`.
  - Support calls such as `a("hello")`.