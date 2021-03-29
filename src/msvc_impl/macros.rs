/// Embeds raw linker arguments for Windows targets.
///
/// Many arguments that work on the command line will not work here. See
/// [`LinkArgs::raw`](crate::windows::msvc::LinkArgs::raw) for more information.
///
/// # Example
///
/// ```rust
/// link_args::windows::raw!(unsafe "/STACK:0x800000 /ENTRY:mainCRTStartup");
/// ```
#[macro_export]
macro_rules! windows_raw {
    (unsafe $raw_args:expr) => {
        #[cfg(windows)]
        const _:() = {
            enum ns {}
            impl ns {
                const raw_args: &'static [u8] = $raw_args.as_bytes();
                const args: [u8; ns::raw_args.len()+1] = {
                    let mut bytes = [0; ns::raw_args.len() + 1];
                    let mut index = 0;
                    while index < ns::raw_args.len() {
                        bytes[index] = ns::raw_args[index];
                        index += 1;
                    }
                    bytes[index] = b' ';
                    bytes
                };
            }
            $crate::impl_msvc_bytes!(
                ns::args.len(),
                ns::args
            );
        };
    };
}


/// Turn the given bytes into a linker directive without any processing.
///
/// This will not check for errors such as invalid arguments.
/// The bytes should end with a space (` `) otherwise to seperate it from any
/// further arguments that may be added.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_msvc_bytes {
    ($size:expr, $bytes:expr) => {
        const _: () = {
            // This cfg restraint can be loosend if we support another target_env.
            #[cfg(all(windows, target_env = "msvc"))]
            #[link_section = ".drectve"]
            #[used]
            static DIRECTIVE: [u8; $size] = $bytes;
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
/// link_args::windows::stack_size!(0x800000);
/// ```
///
/// Reserve 8 MiB for the stack and allocate 4 MiB as soon as the program starts.
///
/// ```rust
/// link_args::windows::stack_size!(0x800000, 0x400000);
/// ```
#[macro_export]
macro_rules! windows_msvc_stack_size {
    ($reserve:expr) => {
        const _: () = {
            $crate::impl_msvc_bytes!(
                $crate::windows::msvc::ArgSize::STACK_SIZE,
                $crate::windows::msvc::LinkArgs::new().stack_size($reserve).into_array()
            );
        };
    };
    ($reserve:expr, $commit:expr) => {
        const _: () = {
            $crate::impl_msvc_bytes!(
                $crate::windows::msvc::ArgSize::STACK_SIZE_WITH_COMMIT,
                $crate::windows::msvc::LinkArgs::new().stack_size_with_commit($reserve, $commit).into_array()
            );
        };
    };
}

/// Adds one or more default libraries.
///
/// Default libraries will be used to find symbols when they are not found in
/// libraries specified on the command line.
#[macro_export]
macro_rules! windows_msvc_default_lib {
    ($($lib:expr),+) => {
        $crate::impl_msvc_bytes!(
            $crate::impl_msvc_arg_size!(default_lib($($lib),+)),
            $crate::impl_msvc_args!($crate::windows::msvc::LinkArgs::new(), default_lib($($lib),+)).into_array()
        );
    };
}

/// Set a group of arguments for the Windows linker.
///
/// The following safe arguments can be set:
///
///  * [`stack_size`](crate::windows::msvc::LinkArgs::stack_size)
///  * [`default_lib`](crate::windows::msvc::LinkArgs::default_lib)
///
/// The following unsafe arguments can be set:
/// 
///  * [`no_default_lib`](crate::windows::msvc::LinkArgs::no_default_lib)
///  * [`disable_all_default_libs`](crate::windows::msvc::LinkArgs::disable_all_default_libs)
///  * [`raw`](crate::windows::msvc::LinkArgs::raw)
///
/// # Examples
///
/// # Safe arguments
///
/// ```rust
/// link_args::windows! {
///     stack_size(0x80000);
///     default_lib("kernel32.lib", "Shell32.lib");
/// }
/// ```
///
/// # Unsafe arguments
///
/// ```no_run
/// link_args::windows! {
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
macro_rules! windows {
    (unsafe {
        $($tt:tt(
            $($expr:expr),*
            $(,)?
        ));+;
    }) => {
        #[cfg(target_env="msvc")]
        const _: () = {
            use $crate::{impl_msvc_arg_size, impl_msvc_args, impl_msvc_bytes, windows::msvc::LinkArgs};
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
        #[cfg(target_env="msvc")]
        const _: () = {
            use $crate::{impl_msvc_arg_size, impl_msvc_args, impl_msvc_bytes, windows::msvc::LinkArgs};
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

/// Build the linker arguments using a macro.
#[doc(hidden)]
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

/// Calculate the size of linker arguments using a macro.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_msvc_arg_size {
    // These are (probably) safe.
    (stack_size($reserve:expr)) => {
        $crate::windows::msvc::ArgSize::STACK_SIZE
    };
    (stack_size($reserve:expr, $commit:expr)) => {
        $crate::windows::msvc::ArgSize::STACK_SIZE_WITH_COMMIT
    };
    (default_lib($($lib:expr),+)) => {
        0$(
            +$crate::windows::msvc::ArgSize::default_lib($lib)
        )+
    };
    // These are unsafe.
    (no_default_lib($($lib:expr),+)) => {
        0$(
            +$crate::windows::msvc::ArgSize::no_default_lib($lib)
        )+
    };
    (disable_all_default_libs()) => {
        $crate::windows::msvc::ArgSize::DISABLE_ALL_DEFAULT_LIBS
    };
    (raw($lib:expr)) => {
        $lib.len() + 1
    };
}
