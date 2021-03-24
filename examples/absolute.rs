// Reserve 8 MiB for the stack.
link_args::msvc::stack_size!(0x800000);

// Link the ucrt dynamically and vcruntime statically.
link_args::msvc::default_lib!("ucrt", "libvcruntime", "libcmt");
link_args::msvc::no_default_lib!(
    "libvcruntimed.lib", "vcruntime.lib", "vcruntimed.lib",
    "libcmtd.lib", "msvcrt.lib", "msvcrtd.lib",
    "libucrt.lib", "libucrtd.lib", "ucrtd.lib",
);

fn main() {
    println!("Hello world!");
}