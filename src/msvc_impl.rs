mod buffer;
mod macros;

use buffer::Buffer;

/// Constants and functions to help to calculate the byte length of an argument.
pub struct ArgSize;
impl ArgSize {
    /// The size of `STACK` directive with a `reserve` value.
    pub const STACK_SIZE: usize = "/STACK:0x00000000 ".len();
    /// The size of `STACK` directive with `reserve` and `commit` values.
    pub const STACK_SIZE_WITH_COMMIT: usize = "/STACK:0x00000000,0x00000000 ".len();
    /// The size of the `NODEFAULTLIB` directive without any values.
    pub const DISABLE_ALL_DEFAULT_LIBS: usize = "/NODEFAULTLIB ".len();
    /// The size of the `DEFAULTLIB` directive.
    pub const fn default_lib(lib: &str) -> usize {
        "/DEFAULTLIB: \"\"".len() + lib.len()
    }
    /// The size of the `NODEFAULTLIB` directive with a value.
    pub const fn no_default_lib(lib: &str) -> usize {
        "/NODEFAULTLIB: \"\"".len() + lib.len()
    }
}

/// Helps to construct MSVC linker arguments.
pub struct LinkArgs<const CAPACITY: usize> {
    buffer: Buffer::<CAPACITY>
}
impl<const CAPACITY: usize> LinkArgs<CAPACITY> {
    /// The `STACK` directive.
    ///
    /// `reserve` is the number of bytes of virtual memory to reserve for the
    /// stack.
    pub const fn stack_size(mut self, reserve: u32) -> Self {
        self.buffer = self.buffer
            .push_directive("STACK")
            .push_value_hex(reserve)
            .push_seperator();
        self
    }
    /// The `STACK` directive with explicit commit value.
    ///
    /// `reserve` is the number of bytes of virtual memory to reserve for the
    /// stack. `commit` is the number of byte of physical memory to allocate for
    /// the stack when the program starts.
    pub const fn stack_size_with_commit(mut self, reserve: u32, commit: u32) -> Self {
        self.buffer = self.buffer
            .push_directive("STACK")
            .push_values_hex(&[reserve, commit])
            .push_seperator();
        self
    }
    /// The `DEFAULTLIB` directive. Adds a library to use.
    /// 
    /// Libraries specified on the command line will override default libraries if
    /// there is a conflict.
    pub const fn default_lib(mut self, lib: &str) -> Self {
        self.buffer = self.buffer
            .push_directive("DEFAULTLIB")
            .push_value_quoted(lib)
            .push_seperator();
        self
    }
    /// The `NODEFAULTLIB` directive. Prevents a default library from being used.
    ///
    /// Overrides `default_lib`. Libraries specified on the command line will
    /// bypass `no_default_lib`.
    ///
    /// This can be unsafe if used with `default_lib` to replace symbols.
    /// 
    /// # Examples
    ///
    /// Prevent kernel32 from being linked.
    ///
    /// ```rust
    /// # #[cfg(not(debug_assertions))]
    /// link_args::msvc::no_default_lib!("kernel32");
    /// ```
    pub const unsafe fn no_default_lib(mut self, lib: &str) -> Self {
        self.buffer = self.buffer
            .push_directive("NODEFAULTLIB")
            .push_value_quoted(lib)
            .push_seperator();
        self
    }
    /// The `NODEFAULTLIB` directive wihout arguments. Prevent any default lib
    /// from being used.
    ///
    /// Completely disables any and all use of `default_lib`. This is safe
    /// because if a symbol is unavaliable the program will fail to compile.
    pub const fn disable_all_default_libs(mut self) -> Self {
        self.buffer = self.buffer.push_directive("NODEFAULTLIB").push_seperator();
        self
    }
    /// One or more raw arguments, seperated by a space.
    ///
    /// Many arguments that work on the command line will not work here.
    ///
    /// # Examples
    ///
    /// ## The `windows_msvc!` macro.
    ///
    /// ```rust
    /// link_args::windows_msvc!{
    ///     unsafe {
    ///         raw("/ENTRY:mainCRTStartup /STACK:0x800000");
    ///     }
    /// }
    /// ```
    ///
    /// ## The `windows_msvc::raw!` macro.
    ///
    /// ```rust
    /// link_args::windows_msvc::raw!(unsafe "/ENTRY:mainCRTStartup /STACK:0x800000");
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
    pub const unsafe fn raw(mut self, raw: &str) -> Self {
        self.buffer = self.buffer.push(raw.as_bytes()).push_seperator();
        self
    }

    /// Create an empty argument list with the `CAPACITY` of the type.
    pub const fn new() -> Self {
        Self {
            buffer: Buffer::new()
        }
    }
    /// Get the length in bytes.
    pub const fn len(&self) -> usize {
        self.buffer.len
    }
    /// Consume the `LinkArgs` and return its byte buffer.
    pub const fn into_array(self) -> [u8; CAPACITY] {
        self.buffer.buffer
    }
}
