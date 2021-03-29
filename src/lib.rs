#![no_std]

//! Allows setting linker arugments at compile time without a build script.
//! Currently only supports Windows MSVC toolchains.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! link_args = "0.6"
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
//! link_args::windows::stack_size!(0x800000);
//! ```
//!
//! ## Add a default library
//!
//! Add "kernel32.lib" to the libraries that are serached for symbols.
//!
//! ```rust
//! link_args::windows::default_lib!("kernel32.lib");
//! ```
//!
//! ## Use the `windows!` macro
//! 
//! The [`windows!`] macro lets you set multiple arguments at once.
//!
//! ```rust
//! link_args::windows! {
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
//! link_args::windows! {
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
//! <style>#macros + table > tbody > tr:not(:first-child) { display: none !important; }</style>
//!

mod msvc_impl;



/// Set linker arguments for the Windows toolchain
pub mod windows {
    #[doc(inline)]
    pub use crate::windows_raw as raw;

    #[doc(inline)]
    pub use crate::windows_msvc_stack_size as stack_size;
    #[doc(inline)]
    pub use crate::windows_msvc_default_lib as default_lib;

    /// Helpers for constructing MSVC linker arguments.
    pub mod msvc {
        // These are mostly exported so I can use links.
        pub use crate::msvc_impl::LinkArgs;
        pub use crate::msvc_impl::ArgSize;
    }
}
