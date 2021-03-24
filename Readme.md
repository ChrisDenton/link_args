Send arguments to the linker from within `main.rs` or `lib.rs`.
Currently only supports Windows MSVC targets.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
link_args = "0.4"
```

# Example

```rust
// Reserve 8 MiB for the stack.
link_args::msvc::stack_size!(0x800000);

// Only set these in release mode.
#[cfg(not(debug_assertions))]
link_args::msvc! {
    // Link the ucrt dynamically and vcruntime statically.
    default_lib("ucrt", "libvcruntime", "libcmt");
    // Disable the other C variants library.
    no_default_lib(
        "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
        "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
        "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
    );
}
```
