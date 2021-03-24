// NOTE TO SELF: These macros are horrible copy/pasta. Rewrite.

//! Send arguments to the linker from within `main.rs`.
//! Currently only supports Windows MSVC.
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
//! ## `msvc` macros
//! 
//! ```rust
//! // Reserve 8 MiB for the stack.
//! link_args::msvc::stack_size!(0x800000);
//! 
//! // Only set these in release mode.
//! #[cfg(not(debug_assertions))]
//! link_args::msvc! {
//!     // Link the ucrt dynamically and vcruntime statically.
//!     default_lib("ucrt", "libvcruntime", "libcmt");
//!     // Disable the other C runtime libraries.
//!     no_default_lib(
//!         "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
//!         "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
//!         "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
//!     );
//! }
//! ```
//! 
//! ## Raw arguments
//! 
//! ```rust
//! link_args::msvc::raw!(unsafe "/STACK:0x800000 /ENTRY:mainCRTStartup");
//! ```
//! 
//! This reserves 8 MiB (8,388,608 bytes) for the stack and sets
//! the application's entry point to `mainCRTStartup`.
//!
//! <style>#macros + table > tbody > tr:not(:first-child) { display: none !important; }</style>
//!

#![no_std]

/// Embeds raw linker arguments for msvc targets.
///
/// Many arguments that work on the command line will not work here.
///
/// # Example
///
/// ```rust
/// link_args::msvc::raw!(unsafe "/STACK:0x800000 /ENTRY:mainCRTStartup");
/// ```
///
/// # Possible arguments
///
/// * [/DEFAULTLIB](https://docs.microsoft.com/en-us/cpp/build/reference/defaultlib-specify-default-library?view=msvc-160)
/// * [/NODEFAULTLIB](https://docs.microsoft.com/en-us/cpp/build/reference/nodefaultlib-ignore-libraries?view=msvc-160)
/// * [/STACK](https://docs.microsoft.com/en-us/cpp/build/reference/stack-stack-allocations?view=msvc-160)
/// * [/HEAP](https://docs.microsoft.com/en-us/cpp/build/reference/heap-set-heap-size?view=msvc-160)
/// * [/SUBSYSTEM](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-160)
/// * [/EXPORT](https://docs.microsoft.com/en-us/cpp/build/reference/export-exports-a-function?view=msvc-160)
/// * [/INCLUDE](https://docs.microsoft.com/en-us/cpp/build/reference/include-force-symbol-references?view=msvc-160)
/// * [/MANIFESTDEPENDENCY](https://docs.microsoft.com/en-us/cpp/build/reference/manifestdependency-specify-manifest-dependencies?view=msvc-160)
/// * [/MERGE](https://docs.microsoft.com/en-us/cpp/build/reference/merge-combine-sections?view=msvc-160)
/// * [/SECTION](https://docs.microsoft.com/en-us/cpp/build/reference/section-specify-section-attributes?view=msvc-160)
/// * [/ENTRY](https://docs.microsoft.com/en-us/cpp/build/reference/entry-entry-point-symbol?view=msvc-160)
///
/// # Limitations
///
/// Different versions of the MSVC linker may support (or not support) different
/// embeded arguments. Unsupported arguments or values may be silently ignored
/// by the linker.
#[macro_export]
macro_rules! msvc_raw {
    (unsafe { $args:expr }) => {
        $crate::msvc::raw!(unsafe $args);
    };
    (unsafe $args:expr) => {
        $crate::msvc::raw!(>>>INTERNAL<<<, $args.as_bytes());
    };
    (>>>INTERNAL<<<, $___:expr) => {
        const _: () = {
            const ARGS: &[u8] = $___;
            #[cfg(all(windows, target_env = "msvc"))]
            #[link_section = ".drectve"]
            #[used]
            static DIRECTIVE: [u8; ARGS.len() + 1] = {
                let mut array = [0; ARGS.len() + 1];
                let mut index = 0;
                while index < ARGS.len() {
                    array[index] = ARGS[index];
                    index += 1;
                }
                array[index] = b' ';
                array
            };
        };
    };
}

#[doc(hidden)]
pub const fn has_quote(s: &[u8]) -> bool {
    let mut index = 0;
    while index < s.len() {
        if s[index] == b'"' { return true; }
        index += 1;
    }
    false
}

/// Add one or more default libraries.
#[macro_export]
macro_rules! msvc_defaultlib {
    ($library:expr) => {
        const _: () = {
            const _Str_: &str = $library;
            if _Str_.len() > 0 && !$crate::has_quote(_Str_.as_bytes()) {
                const S: &str = concat!("/DEFAULTLIB:\"", $library, "\"");
                $crate::msvc::raw!(unsafe S);
            }
        };
    };
    ($($library:expr),+$(,)?) => {
        $(
            $crate::msvc::default_lib!($library);
        )+
    };
}

/// Prevent one or more libraries being loaded unless explicitly added on the
/// command line.
///
/// This will override any library that is also specified in [msvc::default_lib].
///
/// # Examples
///
/// Prevent kernel32 from being linked.
///
/// ```rust
/// # #[cfg(not(debug_assertions))]
/// link_args::msvc::no_default_lib!("kernel32");
/// ```
///
/// Prevent any library being linked except through the command line:
///
/// ```rust
/// # #[cfg(not(debug_assertions))]
/// link_args::msvc::no_default_lib!();
/// ```
#[macro_export]
macro_rules! msvc_nodefaultlib {
    () => {
        $crate::msvc::raw!(unsafe "/NODEFAULTLIB");
    };
    ($library:expr) => {
        const _: () = {
            const _Str_: &str = $library;
            if _Str_.len() > 0 && !$crate::has_quote(_Str_.as_bytes()) {
                const S: &str = concat!("/NODEFAULTLIB:\"", $library, "\"");
                $crate::msvc::raw!(unsafe S);
            }
        };
    };
    ($($library:expr),+$(,)?) => {
        $(
            $crate::msvc::no_default_lib!($library);
        )+
    };
}

/// Set how much virtual memory is avaliable for the stack.
///
/// You can also optionally allocate physical memory upfront. Be aware that
/// Rust's `std::thread` can and will override these settings for all but the
/// main thread.
///
/// # Examples
///
/// Reserve 8 MiB of virtual memory for the stack.
///
/// ```rust
/// link_args::msvc::stack_size!(0x800000);
/// ```
///
/// Reserve 8 MiB for the stack and allocate 4 MiB as soon as the program starts.
///
/// ```rust
/// link_args::msvc::stack_size!(0x800000, 0x400000);
/// ```
#[macro_export]
macro_rules! msvc_stack_size {
    ($reserve:expr, $commit:expr) => {
        const _: () = {
            #[cfg(all(windows, target_env = "msvc"))]
            #[link_section = ".drectve"]
            #[used]
            static DIRECTIVE: [u8; 29] = {
                let lookup = [
                    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
                    b'9', b'a', b'b', b'c', b'd', b'e', b'f',
                ];
                let mut reserve: u32 = $reserve;
                let mut commit: u32 = $commit;
                let mut array = *b"/STACK:0x00000000,0x00000000 ";
                let mut index = 16;
                while index > 16-8 {
                    array[index] = lookup[(reserve & 0xf) as usize];
                    reserve >>= 4;
                    index -= 1;
                }
                let mut index = 27;
                while index > 27-8 {
                    array[index] = lookup[(commit & 0xf) as usize];
                    commit >>= 4;
                    index -= 1;
                }
                array
            };
        };
    };
    ($reserve:expr) => {
        const _: () = {
            #[cfg(all(windows, target_env = "msvc"))]
            #[link_section = ".drectve"]
            #[used]
            static DIRECTIVE: [u8; 18] = {
                let lookup = [
                    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
                    b'9', b'a', b'b', b'c', b'd', b'e', b'f',
                ];
                let mut reserve: u32 = $reserve;
                let mut array = *b"/STACK:0x00000000 ";
                let mut index = 16;
                while index > 16-8 {
                    array[index] = lookup[(reserve & 0xf) as usize];
                    reserve >>= 4;
                    index -= 1;
                }
                array
            };
        };
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! msvc_internal {
    (>>>INTERNAL<<<, stack_size($reserve:expr$(, $commit:expr)?)) => {
        $crate::msvc::stack_size!($reserve$(, $commit)?);
    };
    (>>>INTERNAL<<<, default_lib($($args:expr),+)) => {
        $crate::msvc::default_lib!($($args),+);
    };
    (>>>INTERNAL<<<, no_default_lib($($args:expr),+)) => {
        $crate::msvc::no_default_lib!($($args),+);
    };
    (>>>INTERNAL<<<, no_default_lib) => {
        $crate::msvc::no_default_lib!();
    };
    (>>>INTERNAL<<<, raw($tt:tt)) => {
        $crate::msvc::raw!();
    };
}

/// Set a group of arguments for the msvc linker.
///
/// The macro can have one or more lines in the form `argument(value1, value1);`.
///
/// # Example
///
/// ```rust
/// link_args::msvc! {
///     // Link the ucrt dynamically and vcruntime statically.
///     default_lib("ucrt", "libvcruntime", "libcmt");
///     // Disable the other C runtime libraries.
///     no_default_lib(
///         "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
///         "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
///         "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
///     );
/// }
/// ```
#[macro_export]
macro_rules! msvc {
    ($($tt:tt($($($args:expr),+$(,)?)?));+$(;)?) => {
        $(
            $crate::msvc_internal!(>>>INTERNAL<<<, $tt$(($($args),+))?);
        )+
    };
}

pub mod msvc {
    #[doc(inline)]
    pub use crate::msvc_stack_size as stack_size;

    #[doc(inline)]
    pub use crate::msvc_defaultlib as default_lib;

    #[doc(inline)]
    pub use crate::msvc_nodefaultlib as no_default_lib;

    #[doc(inline)]
    pub use crate::msvc_raw as raw;
}
