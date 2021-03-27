link_args::windows_msvc! {
    unsafe {
        // Reserve 8 MiB for the stack.
        stack_size(0x800000, 0x400000);
        
        // Link the ucrt dynamically and vcruntime statically.
        default_lib("ucrt", "libvcruntime", "libcmt");
    }
}

link_args::windows_msvc! {
    unsafe {
        // Disable the other C runtime libraries.
        no_default_lib(
            "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
            "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
            "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
        );
    }
}

fn main() {
    println!("Hello world!");
}
