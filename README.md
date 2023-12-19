# The Bay Programming Language

bay-lang is my own personal experiment in creating a high quality programming language, the code and design documents here will evolve as my understanding of optimization, compiler architecture, type theory, and other topics deepen and I apply them to this codebase.

## Design Goal

In the end, Bay is meant to provide the following:
- Low level control over the computer similar to languages like Rust, Zig, and C.
- High level type safety and abstractions similar to languages like Rust, Zig, Swift, OCaml, and Haskell.
- Self-introspection and compile time code generation abilities like Rust's macro system and Zig's comptime.
- Runtime compilation and embedding to allow using Bay within itself as an embedded scripting language.
- A imperative-functional hybrid coding style.
- Provable correctness integrated at the type system and compiler level without resorting to external tools.
- A rich standard library providing most, if not all, common use cases and functionality integrated directly into the language.
