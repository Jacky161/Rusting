# Notes

All variables are denoted by `${}`.

## Cargo Commands

### Creating a Project
```bash
cargo new ${PROJECT_NAME}
```

### Building/Running a Project

```bash
# Add --release for release build
cargo build  # compiles to ./target/debug/${PROJECT_NAME}

cargo run
cargo check  # Checks for compilation errors
```

### Dependencies

In `cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"  # same as rand = "^0.8.5"
```

Follows semver. Any version >=0.8.5 and <0.9.0.

Run `cargo update` to update dependencies.

### Packages, Crates, and Modules

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module


#### Crates

Smallest unit of code that the Rust compiler considers at a time.

Binary crates: programs compiled to an executable.
Library crates: Define functionality to be shared with multiple projects.


#### Modules and Submodules

If you declare a module with `mod x;`, the compiler looks for it in:

- Inline, within curly bracles replacing the semicolon.
- In the file `src/<path to current module>/x.rs`
- In the file `src/<path to current module>>/x/mod.rs`
