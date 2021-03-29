Allows setting linker arugments at compile time without a build script.
Currently only supports Windows MSVC toolchains.

Minimum Rust version: 1.51

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
link_args = "0.6"
```

# Examples

## Set the stack size

```rust
// Reserve 8 MiB for the stack.
link_args::windows_msvc::stack_size!(0x800000);
```

## Add a library

```rust
link_args::windows_msvc::default_lib!("kernel32.lib");
```
