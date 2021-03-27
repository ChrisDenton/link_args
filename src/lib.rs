#![no_std]

//! Allows setting linker arugments at compile time without a build script.
//! Currently only supports Windows MSVC toolchains.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! link_args = "0.5"
//! ```
//!
//! # Examples
//!
//! Put these examples at the root of your `main.rs` or `lib.rs`.
//! 
//! ## Set the size of the stack
//!
//! Reserve 8 MiB (8,388,608 bytes) of virtual memory for the stack. This should
//! only be set for crates that produce a `.exe` or `.dll` binary.
//!
//! ```rust
//! link_args::windows_msvc::stack_size!(0x800000);
//! ```
//!
//! ## Add a default library
//!
//! Add "mylibrary.lib" to the libraries that are serached for symbols.
//!
//! ```rust
//! link_args::windows_msvc::default_lib!("kernel32.lib");
//! ```
//!
//! ## Use the `windows_msvc` macro
//! 
//! The [`windows_msvc!`] macro lets you set multiple arguments at once.
//!
//! ```rust
//! link_args::windows_msvc! {
//!     stack_size(0x800000);
//!     default_lib("kernel32.lib");
//! }
//! ```
//!
//! If you use unsafe linker arguments the you must mark the whole block as
//! `unsafe`.
//! 
//! ```rust
//! // Only set these in release mode.
//! #[cfg(not(debug_assertions))]
//! link_args::windows_msvc! {
//!     // Some of these linker args are unsafe so we have to use
//!     // an `unsafe` block.
//!     unsafe {
//!         // Link the ucrt dynamically and vcruntime statically.
//!         default_lib("ucrt", "libvcruntime", "libcmt");
//!         // Disable the other C runtime libraries.
//!         no_default_lib(
//!             "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
//!             "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
//!             "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
//!         );
//!     }
//! }
//! ```
//!
//! <style>#macros + table > tbody > tr:not(:last-child) { display: none !important; }</style>
//!

mod msvc_impl;

// The "official" public interface.
/// Construct and use linker directives for Windows MSVC toolchains.
pub mod windows_msvc {
    // These are mostly exported so I can use links.
    pub use crate::msvc_impl::LinkArgs;
    pub use crate::msvc_impl::ArgSize;
    #[doc(inline)]
    pub use crate::impl_msvc_stack_size as stack_size;
    #[doc(inline)]
    pub use crate::impl_msvc_default_lib as default_lib;
    #[doc(inline)]
    pub use crate::impl_msvc_raw as raw;
}
