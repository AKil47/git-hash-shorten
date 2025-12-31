fn main() {
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=crypt32");
    println!("cargo:rustc-link-lib=user32");
    // Some git2 versions/setups might also need:
    println!("cargo:rustc-link-lib=rpcrt4");
    println!("cargo:rustc-link-lib=ole32");
}
