# Contributing to rustybox

Hello and thanks for your interest in rustybox! We're happy to have you here.

## Removing `unsafe`s

A broad metric for how well the rustybox project is progressing is the number of `unsafe`s across the codebase. Because rustybox started out as a c2rust transpile of busybox, we've inherited a lot of these. If you're interested in contributing but not quite sure where to start, playing `unsafe`-golf is one option. Pick a command that you find interesting and see if you can remove an `unsafe` somewhere in that module. Sometimes this can be done with "local" thinking, and other times it requires a more "global" understanding of the code.

## Imports

Prefer

```rust
use libc::gid_t;
use libc::uid_t;
```

over

```rust
use libc::{gid_t, uid_t};
```

Why: Using one-line-one-import makes global find/replace refactors much, much easier. And this project requires a lot of find/replace refactors.

## Replacing `extern "C"` bits with `use` imports strategy

c2rust does a lot of importing via `extern "C"` that can be more idiomatically written with rust `use` statements. One approach to these kinds of refactors is the following:

1. Do a global find/replace to remove all instances of the `extern "C"` version, eg.

```rust
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
```

2. Do another global find/replace all to replace

```rust
use libc;
```

with

```rust
use libc;
use crate::foo::bar::bb_internal_getpwnam;
```

This will introduce a lot of unused imports but the project should compile.

3. Finally, run `cargo fix` to automatically remove all of the unused imports.
