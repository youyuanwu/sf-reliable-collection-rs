use std::{env, path::Path};

fn main() {
    if cfg!(windows) {
        // add link dir for fabric support libs. This is propagated to downstream targets
        let dir = String::from("build");
        let package_root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let abs_dir = package_root + "\\..\\..\\..\\" + &dir;
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&abs_dir).display()
        );
    }
}
