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

## Current Design State
- Strongly typed and heavily safety focused, however leans heavily on compiler inference and optimization to make things efficient and enable a very developer friendly interface.
- Everything is self contained, meaning the compiler/interpreter is part of the standard library and the actual CLI is just a CLI that calls the standard library. It’s also batteries included so everything else is also included with the language in the standard library.
- Start with an interpreted backend because it’s super easy, then transition to a self-hosted compiled backend once the frontend is complete.
- Use an imperative style of programming inspired by Rust and Swift to make it very developer friendly.
- Nullability with types like i64? and unwrapping with ? operator like in JS.
    - Method chaining on the null value can be performed using the `:` operator, in this case the ok value must be of type `T?` or type `T`, but the err value can be anything. The compiler will coerce the non-null value as needed to have the correct expression return type.
- Named function arguments and argument pattern matching.
- Package/module definitions and metadata (Cargo.toml and other configs) will all be in code.
- Package naming will use namespaces, and modules. Generally it’ll follow the same syntax as Rust, however it also supports namespaces and nested namespaces like `@v19::@ui::button`, which looks for a button module in the `ui` namespace inside the `v19` namespace. Absolute imports using `::std::path::Path` will resolve to the root namespace, making them equivalent to `@::std::path::Path`, and imports without a prefix like `std::path::Path` will first look for a matching `std` module in the current scope, then move up to the root namespace.
- Errors will be declared using the `err` keyword as follows, at runtime they will all have an identical size. One word for the tag saying which error it is, one word for the reference to any data inside, null if no data, the format for which is defined in the `err` keyword definition, then another word pointing to the last item in the traceback linked list, whenever the error is propagated by the language up a try context it pushes the current place onto the linked list. The linked list is by default only enabled in dev mode, then turned off in production. Can also just explore how Rust does backtraces and see if there’s a nicer way to do it at purely compile time.
    - Behavior wise, methods can be chained on the ok value using the `.` operator, if so the error types will be merged into a larger union. The `->` operator can be used to chain on the err value, however the returned ok type must match the type on the result going in.
    - Error types will be of type `ok, err` where either side can be the never type (`!`) which I think so the zero type in type theory. The err type can also be an empty union, I don’t know what the syntax will be yet though. By default, error sets or unions will be non-exhaustive, meaning more can be added without it being a break change. To represent no errors now but more later, the empty set/union can be used, a never type indicates an error is impossible.
- Built-in templating language and support to create your own templates. These templates can then be fed to macros or functions or anything else to allow the creation of things like an HTML or CSS type-checked DSL or anything else.
- By default, a value of type `T` is an immutable reference counted object, however the ownership and borrowing rules can be used with a type `$T` as the owned type and `&T` as the immutable references.
    - Mutability is opt in a the struct, attribute, or attribute group level and consists of three types `mut` which requires an exclusive `mut T` hold on the shared value, or an owned `$T` value. This essentially functions like normal mutability. The second type is `mux` which is a compiler-implemented mutex, and the final is `rw` as a compiler-implemented rwlock. These locks can be obtained using the `.lock`, `.read`, and `.write` postfix keywords.
    - Attribute groups can be declared with the syntax `mut group: x + y + z`, all the constituents must use the same mutability construct as the group declaration.
