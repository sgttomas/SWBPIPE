//! Reach scan of the native-smoke result injection (`native_smoke_inject.rs`;
//! RT-U4, DEL-04-05 `Work/criterion-9-measurement.md`): a product build, which
//! never enables the `native-smoke` feature, cannot reach it. Runs in every
//! `cargo test` of this crate, with or without the feature.
//!
//! Each source file is parsed (`syn`, as the user-release reach scan does) and
//! its syntax tree walked. A use of the injection — an identifier naming the
//! module, its entry or its environment constants, a string literal naming its
//! environment variables, or a macro's or non-doc attribute's tokens naming any
//! of them (`env!`, `include!`, `#[path = …]`) — is accepted only inside a node
//! whose own attributes include exactly `#[cfg(feature = "native-smoke")]`: an
//! item, an impl item, a `let`, a macro statement, or an `if`, block, call,
//! method call, macro or `match` expression statement. Anything else naming the
//! injection fails the scan, including a gate on the wrong node (an unrelated
//! statement on the same line) or a gate on another condition. Comments and doc
//! attributes are not code and are ignored. A file that does not parse fails.
//!
//! Also: the module is declared once, in `lib.rs`; no feature of this crate
//! implies `native-smoke`, and it is not a default. The release-binary check
//! (the variables' names absent from a product build) is in the PR record.

use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};
use syn::{Attribute, Expr, ImplItem, Item, Stmt};

const NEEDLES: [&str; 6] = [
    "SWBPIPE_NATIVE_SMOKE_INJECT_RESULT",
    "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT",
    "native_smoke_inject",
    "injected_outcome",
    "INJECT_RESULT_ENV",
    "INJECT_ROOT_ENV",
];
const EXEMPT: [&str; 2] = ["native_smoke_inject.rs", "native_smoke_reach_scan_tests.rs"];

fn names_the_injection(text: &str) -> bool {
    NEEDLES.iter().any(|needle| text.contains(needle))
}

/// Exactly `#[cfg(feature = "native-smoke")]` among `attrs`.
fn gated(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && matches!(&attr.meta, syn::Meta::List(list) if list.tokens.to_string() == "feature = \"native-smoke\"")
    })
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::ExternCrate(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::ForeignMod(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::TraitAlias(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        Item::Use(i) => &i.attrs,
        _ => &[],
    }
}

fn impl_item_attrs(item: &ImplItem) -> &[Attribute] {
    match item {
        ImplItem::Const(i) => &i.attrs,
        ImplItem::Fn(i) => &i.attrs,
        ImplItem::Type(i) => &i.attrs,
        ImplItem::Macro(i) => &i.attrs,
        _ => &[],
    }
}

/// The attributes of the expression statements a gate is accepted on; any
/// other expression's gate is not recognized (so its contents count as ungated).
fn expr_attrs(expr: &Expr) -> &[Attribute] {
    match expr {
        Expr::If(e) => &e.attrs,
        Expr::Block(e) => &e.attrs,
        Expr::Call(e) => &e.attrs,
        Expr::MethodCall(e) => &e.attrs,
        Expr::Macro(e) => &e.attrs,
        Expr::Match(e) => &e.attrs,
        _ => &[],
    }
}

#[derive(Default)]
struct Scan {
    depth: usize,
    found: Vec<String>,
}

impl Scan {
    fn within(&mut self, gate: bool, walk: impl FnOnce(&mut Self)) {
        self.depth += usize::from(gate);
        walk(self);
        self.depth -= usize::from(gate);
    }
    fn note(&mut self, what: &str, text: String) {
        if self.depth == 0 && names_the_injection(&text) {
            self.found.push(format!("{what}: {text}"));
        }
    }
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_item(&mut self, node: &'ast Item) {
        self.within(gated(item_attrs(node)), |scan| visit::visit_item(scan, node));
    }
    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        self.within(gated(impl_item_attrs(node)), |scan| visit::visit_impl_item(scan, node));
    }
    fn visit_stmt(&mut self, node: &'ast Stmt) {
        let gate = match node {
            Stmt::Local(local) => gated(&local.attrs),
            Stmt::Macro(mac) => gated(&mac.attrs),
            Stmt::Expr(expr, _) => gated(expr_attrs(expr)),
            Stmt::Item(_) => false,
        };
        self.within(gate, |scan| visit::visit_stmt(scan, node));
    }
    fn visit_ident(&mut self, node: &'ast proc_macro2::Ident) {
        self.note("identifier", node.to_string());
    }
    fn visit_lit_str(&mut self, node: &'ast syn::LitStr) {
        self.note("string", node.value());
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.note("macro tokens", node.tokens.to_string());
        visit::visit_macro(self, node);
    }
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        // Doc text is not code; every other attribute is (`#[path = "…"]` is a string
        // the default walk visits; a list's tokens are read here).
        if node.path().is_ident("doc") {
            return;
        }
        if let syn::Meta::List(list) = &node.meta {
            self.note("attribute", list.tokens.to_string());
        }
        visit::visit_attribute(self, node);
    }
}

/// Every use of the injection in `source` outside the native-smoke gate.
fn ungated(source: &str) -> Result<Vec<String>, syn::Error> {
    let file = syn::parse_file(source)?;
    let mut scan = Scan::default();
    scan.visit_file(&file);
    Ok(scan.found)
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_result_injection_is_used_only_under_the_native_smoke_gate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    assert!(files.len() > 20, "the scan reads the crate's sources");
    let mut problems = Vec::new();
    let mut declarations = 0;
    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy();
        if EXEMPT.contains(&name.as_ref()) {
            continue;
        }
        let text = std::fs::read_to_string(file).unwrap();
        declarations += text.lines().filter(|line| line.trim() == "mod native_smoke_inject;").count();
        let relative = file.strip_prefix(root).unwrap().display().to_string();
        match ungated(&text) {
            Ok(found) => problems.extend(found.into_iter().map(|use_| format!("{relative}: {use_}"))),
            Err(error) => problems.push(format!("{relative}: does not parse ({error})")),
        }
    }
    assert!(problems.is_empty(), "the injection is used outside the native-smoke gate:\n{}", problems.join("\n"));
    assert_eq!(declarations, 1, "the module is declared once");
    let lib = std::fs::read_to_string(root.join("src/lib.rs")).unwrap();
    assert!(lib.contains("#[cfg(feature = \"native-smoke\")]\nmod native_smoke_inject;\n"), "lib.rs declares it under the gate");

    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let features = manifest.split("[features]").nth(1).and_then(|rest| rest.split("\n[").next()).expect("a [features] table");
    let implying: Vec<&str> = features
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && line.contains("native-smoke") && line.trim() != "native-smoke = []")
        .collect();
    assert!(implying.is_empty(), "no feature or default implies native-smoke: {implying:?}");
}

#[test]
fn gated_uses_pass() {
    let accepted = [
        "#[cfg(feature = \"native-smoke\")]\nmod native_smoke_inject;",
        "#[cfg(feature = \"native-smoke\")] use crate::native_smoke_inject::injected_outcome;",
        "fn run() { let x = || { #[cfg(feature = \"native-smoke\")] if let Some(o) = crate::native_smoke_inject::injected_outcome(p, r, c) { return o; } 0 }; }",
        "impl R { #[cfg(feature = \"native-smoke\")] fn hook(&self) { crate::native_smoke_inject::injected_outcome(); } }",
        "fn f() { #[cfg(feature = \"native-smoke\")] let path = std::env::var_os(\"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\"); }",
        "/// Mentions native_smoke_inject and SWBPIPE_NATIVE_SMOKE_INJECT_RESULT in a doc comment.\nfn f() {} // and native_smoke_inject in a comment",
    ];
    for source in accepted {
        assert_eq!(ungated(source).unwrap(), Vec::<String>::new(), "{source}");
    }
}

#[test]
fn ungated_uses_fail() {
    let refused = [
        // The gate removed.
        "mod native_smoke_inject;",
        "fn f() { let path = std::env::var_os(\"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\"); }",
        "fn f() { crate::native_smoke_inject::injected_outcome(p, r, c); }",
        // A gated dummy statement, then the use on the same line.
        "fn f() { #[cfg(feature = \"native-smoke\")] let _a = 0; let path = std::env::var_os(\"SWBPIPE_NATIVE_SMOKE_INJECT_ROOT\"); }",
        "#[cfg(feature = \"native-smoke\")] fn a() {} fn b() { injected_outcome(); }",
        "fn f() { #[cfg(feature = \"native-smoke\")] let _a = 0; if let Some(o) = injected_outcome() { drop(o) } }",
        // A gate on another condition.
        "#[cfg(any(test, feature = \"native-smoke\"))] mod native_smoke_inject;",
        "#[cfg(not(feature = \"native-smoke\"))] mod native_smoke_inject;",
        "#[cfg_attr(feature = \"native-smoke\", allow(dead_code))] mod native_smoke_inject;",
        // A gate on an expression whose gate the scan does not accept.
        "fn f() { let v = #[cfg(feature = \"native-smoke\")] (std::env::var(\"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\")); }",
        // Reached through a macro or another module's path.
        "fn f() { let v = env!(\"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\"); }",
        "#[path = \"native_smoke_inject.rs\"] mod other;",
        "const NAME: &str = \"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\";",
    ];
    for source in refused {
        assert!(!ungated(source).unwrap().is_empty(), "accepted, should be refused: {source}");
    }
    assert!(ungated("fn f( {").is_err(), "a file that does not parse is a failure, not a pass");
}
