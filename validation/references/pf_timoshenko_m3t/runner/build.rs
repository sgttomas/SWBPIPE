// Bakes the product commit into the runner (refutation round 3): build.sh sets PFM3T_PRODUCT_HEAD.
fn main() {
    println!("cargo:rerun-if-env-changed=PFM3T_PRODUCT_HEAD");
    let head = std::env::var("PFM3T_PRODUCT_HEAD").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=PFM3T_PRODUCT_HEAD={head}");
}
