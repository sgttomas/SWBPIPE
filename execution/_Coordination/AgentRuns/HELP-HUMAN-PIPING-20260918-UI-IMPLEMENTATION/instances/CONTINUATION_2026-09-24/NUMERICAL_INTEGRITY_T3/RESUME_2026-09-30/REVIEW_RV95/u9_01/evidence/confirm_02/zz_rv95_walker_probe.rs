
// RV95 scratch (never committed): the walker on a synthetic tree whose resolution rustc confirms.
#[test]
#[ignore]
fn zz_rv95_walker_probe() {
    let dir = std::path::PathBuf::from(std::env::var("RV95_TREE").expect("RV95_TREE"));
    let mut got: Vec<String> = non_test_modules(&dir).into_iter().map(|(n, _)| n).collect();
    got.sort();
    println!("RV95_WALKER_NON_TEST {}", got.join(","));
}
