
#[macro_export]
macro_rules! impl_msvc_bytes {
    ($size:expr, $bytes:expr) => {
        const _: () = {
            #[cfg(all(windows, target_env = "msvc"))]
            #[link_section = ".drectve"]
            #[used]
            static DIRECTIVE: [u8; $size] = $bytes;
        };
    };
}

/// Embeds raw linker arguments for msvc targets.
///
/// Many arguments that work on the command line will not work here. See
/// [`LinkArgs::raw`](crate::windows_msvc::LinkArgs::raw) for more information.
///
/// # Example
///
/// ```rust
/// link_args::windows_msvc::raw!(unsafe "/STACK:0x800000 /ENTRY:mainCRTStartup");
/// ```
#[macro_export]
macro_rules! impl_msvc_raw {
    (unsafe $raw_args:expr) => {
        const _:() = {
            enum ns {}
            impl ns {
                const raw_args: &'static str = $raw_args;
            }
            $crate::impl_msvc_bytes!(
                ns::raw_args.len() + 1,
                unsafe {
                    $crate::windows_msvc::LinkArgs::new()
                        .raw(ns::raw_args)
                        .into_array()
                }
            );
        };
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
/// link_args::windows_msvc::stack_size!(0x800000);
/// ```
///
/// Reserve 8 MiB for the stack and allocate 4 MiB as soon as the program starts.
///
/// ```rust
/// link_args::windows_msvc::stack_size!(0x800000, 0x400000);
/// ```
#[macro_export]
macro_rules! impl_msvc_stack_size {
    ($reserve:expr) => {
        const _: () = {
            $crate::impl_msvc_bytes!(
                $crate::windows_msvc::ArgSize::STACK_SIZE,
                $crate::windows_msvc::LinkArgs::new().stack_size($reserve).into_array()
            );
        };
    };
    ($reserve:expr, $commit:expr) => {
        const _: () = {
            $crate::impl_msvc_bytes!(
                $crate::windows_msvc::ArgSize::STACK_SIZE_WITH_COMMIT,
                $crate::windows_msvc::LinkArgs::new().stack_size_with_commit($reserve, $commit).into_array()
            );
        };
    };
}

/// Adds one or more default libraries.
///
/// Default libraries will be used to find symbols when they are not found in
/// libraries specified on the command line.
#[doc(no_inline)]
#[macro_export]
macro_rules! impl_msvc_default_lib {
    ($($lib:expr),+) => {
        $crate::impl_msvc_bytes!(
            $crate::impl_msvc_arg_size!(default_lib($($lib),+)),
            $crate::impl_msvc_args!($crate::windows_msvc::LinkArgs::new(), default_lib($($lib),+)).into_array()
        );
    };
}

/// Set a group of arguments for the msvc linker.
///
/// The following safe arguments can be set:
///
///  * [`stack_size`](crate::windows_msvc::LinkArgs::stack_size)
///  * [`default_lib`](crate::windows_msvc::LinkArgs::default_lib)
///
/// The following unsafe arguments can be set:
/// 
///  * [`no_default_lib`](crate::windows_msvc::LinkArgs::no_default_lib)
///  * [`disable_all_default_libs`](crate::windows_msvc::LinkArgs::disable_all_default_libs)
///  * [`raw`](crate::windows_msvc::LinkArgs::raw)
///
/// # Examples
///
/// # Safe arguments
///
/// ```rust
/// link_args::windows_msvc! {
///     stack_size(0x80000);
///     default_lib("kernel32.lib", "Shell32.lib");
/// }
/// ```
///
/// # Unsafe arguments
///
/// ```rust
/// link_args::windows_msvc! {
///     unsafe {
///         // Prevent some libraries being used unless they specified on the
///         // comamnd line.
///         no_default_lib("kernel32.lib", "Shell32.lib");
///         // This makes the above line redundant.
///         disable_all_default_libs();
///         // Set the entry point.
///         raw("/ENTRY:mainCRTStartup");
///     }
/// }
/// ```
#[macro_export]
macro_rules! windows_msvc {
    (unsafe {
        $($tt:tt(
            $($expr:expr),*
            $(,)?
        ));+;
    }) => {
        const _: () = {
            use $crate::{impl_msvc_arg_size, impl_msvc_args, impl_msvc_bytes, windows_msvc::LinkArgs};
            enum ns {}
            impl ns {
                const SIZE: usize = 0$(+ impl_msvc_arg_size!($tt($($expr),*)))+;
                #[allow(unused_unsafe)]
                const BUFFER: LinkArgs::<{ns::SIZE}> = unsafe {
                    let mut buf = LinkArgs::new();
                    $(
                        buf = impl_msvc_args!(buf, $tt($($expr),*));
                    )+
                    buf
                };
            }
            impl_msvc_bytes!(ns::SIZE, ns::BUFFER.into_array());
        };
    };
    ($($tt:tt(
        $($expr:expr),*
        $(,)?
    ));+;) => {
        const _: () = {
            use $crate::{impl_msvc_arg_size, impl_msvc_args, impl_msvc_bytes, windows_msvc::LinkArgs};
            enum ns {}
            impl ns {
                const SIZE: usize = 0$(+ impl_msvc_arg_size!($tt($($expr),*)))+;
                const BUFFER: LinkArgs::<{ns::SIZE}> = {
                    let mut buf = LinkArgs::new();
                    $(
                        buf = impl_msvc_args!(buf, $tt($($expr),*));
                    )+
                    buf
                };
            }
            impl_msvc_bytes!(ns::SIZE, ns::BUFFER.into_array());
        };
    };
}

#[macro_export]
macro_rules! impl_msvc_args {
    // These are (probably) safe.
    ($args:expr, stack_size($reserve:expr)) => {
        $args.stack_size($reserve)
    };
    ($args:expr, stack_size($reserve:expr, $commit:expr)) => {
        $args.stack_size_with_commit($reserve, $commit)
    };
    ($args:expr, default_lib($($lib:expr),+)) => {
        $args
        $(
            .default_lib($lib)
        )+
    };
    // These are unsafe
    ($args:expr, no_default_lib($($lib:expr),+)) => {
        $args
        $(
            .no_default_lib($lib)
        )+
    };
    ($args:expr, disable_all_default_libs()) => {
        $args.disable_all_default_libs()
    };
    ($args:expr, raw($raw:expr)) => {
        $args.raw($raw)
    };
}
#[macro_export]
macro_rules! impl_msvc_arg_size {
    // These are (probably) safe.
    (stack_size($reserve:expr)) => {
        $crate::windows_msvc::ArgSize::STACK_SIZE
    };
    (stack_size($reserve:expr, $commit:expr)) => {
        $crate::windows_msvc::ArgSize::STACK_SIZE_WITH_COMMIT
    };
    (default_lib($($lib:expr),+)) => {
        0$(
            +$crate::windows_msvc::ArgSize::default_lib($lib)
        )+
    };
    // These are unsafe.
    (no_default_lib($($lib:expr),+)) => {
        0$(
            +$crate::windows_msvc::ArgSize::no_default_lib($lib)
        )+
    };
    (disable_all_default_libs()) => {
        $crate::windows_msvc::ArgSize::DISABLE_ALL_DEFAULT_LIBS
    };
    (raw($lib:expr)) => {
        $lib.len() + 1
    };
}

pub use windows_msvc;
