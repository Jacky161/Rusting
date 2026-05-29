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
