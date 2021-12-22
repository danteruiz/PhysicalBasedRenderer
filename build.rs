use std::env;
fn main() {
    let env_path = env::var("GLFW_LIB_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", env_path);
    println!("cargo:rustc-link-lib=dylib=glfw3");
}
