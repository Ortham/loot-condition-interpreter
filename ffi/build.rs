#[cfg(feature = "ffi-headers")]
mod ffi_headers {
    extern crate cbindgen;

    use std::env;
    use std::fs;

    use self::cbindgen::generate;

    pub fn generate_headers() {
        let crate_dir = env::var("CARGO_MANIFEST_DIR")
            .expect("could not get value of CARGO_MANIFEST_DIR env var");

        fs::create_dir_all("include").expect("could not create include directory");

        generate(&crate_dir)
            .expect("could not generate C/C++ header file")
            .write_to_file("include/loot_condition_interpreter.h");
    }
}

fn main() {
    #[cfg(feature = "ffi-headers")]
    ffi_headers::generate_headers();
}
