use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");

    // https://github.com/rust-lang/rust/pull/83820
    let mut link_args = vec![
        // Only static links are used and prevent linking with the shared libraries.
        "-no-pie",
        "-static",
        // Disable system startup files or libraries when linking. This means
        // that the linker will not include files like `crt0.o` and some of the
        // system standard libraries.
        // See https://gcc.gnu.org/onlinedocs/gcc/Link-Options.html
        //
        // The `-nostdlib` flag is much like a combination of `-nostartfiles` and
        // `-nodefaultlibs`.
        //
        // Since `_start` is defined in the system startup files, with this option
        // we can use our own `_start` function to override the program entry point.
        "-nostdlib",
        "-ffreestanding",
    ];

    match target_arch.as_str() {
        "x86" => {
            link_args.push("-mregparam=3");
            link_args.push("-Wl,-Ttext=0xa0000000");
        }
        "x86_64" => link_args.push("-Wl,-Ttext=0x600000000000"),
        "arm" => link_args.push("-Wl,-Ttext=0x10000000"),
        "aarch64" => link_args.push("-Wl,-Ttext=0x10000000"),
        _ => (),
    };

    for arg in link_args {
        println!("cargo::rustc-link-arg={}", arg)
    }
}
