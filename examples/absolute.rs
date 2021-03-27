// Reserve 8 MiB for the stack.
link_args::windows_msvc::stack_size!(0x800000);

// Link the ucrt dynamically and vcruntime statically.
link_args::windows_msvc::default_lib!("ucrt", "libvcruntime", "libcmt");

fn main() {
    println!("Hello world!");
}
