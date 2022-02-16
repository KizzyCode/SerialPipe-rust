use cc::Build;


/// Selects the platform appropriate shim
fn select_shim() -> &'static str {
    #[cfg(target_family = "unix")]
    {
        return "./shim/shim_unix.c";
    }

    // Fallback panic
    #[allow(unreachable_code)]
    {
        panic!("Your current target platform has no libc-network-shim available")
    }
}


/// Compile and link the shim
fn main() {
    let shim = select_shim();
    Build::new()
        .file(shim)
        .warnings(true)
        .extra_warnings(true)
        .warnings_into_errors(true)
        .compile("shim");
}
