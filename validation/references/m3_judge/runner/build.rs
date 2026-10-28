// Bakes the product commit into the runner: build.sh sets M3J_PRODUCT_HEAD.
fn main() {
    println!("cargo:rerun-if-env-changed=M3J_PRODUCT_HEAD");
    let head = std::env::var("M3J_PRODUCT_HEAD").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=M3J_PRODUCT_HEAD={head}");
}
