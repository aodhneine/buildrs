# buildrs
`buildrs` is a standalone build system for rust, that aims to replace cargo in much more sane way.

## Overview
Project consists of only two files - `build.rs`, which specifies build steps, and `_build.rs`, which
contains abstractions over rustc and build process to make it easier.

```rust
#[path = "_build.rs"]
mod build;
```

## Building
```
$ rustc build.rs
$ ./build
```
