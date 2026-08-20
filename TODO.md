# Blocky language TODO

- [x] Separate a variable's declared type from its current value
  - [x] Example: `let String a;`
  - [x] The declared type is `String`, while the current value starts as `undefined`.
  - [x] Later, reject assignments with incompatible types.

- [ ] Add a function type
  - Store function parameters and body as an expression.
  - Support declarations such as `<function input> ... </function>`.
  - Support calls such as `a("hello")`.

- [ ] Add conditions 
  - if stuff