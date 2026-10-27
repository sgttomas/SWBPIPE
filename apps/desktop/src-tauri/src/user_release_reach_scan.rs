//! `user_release_reach_scan` (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.8, §6): who can reach the renderer's
//! user-released class rule. The scan parses the production source of four
//! crates (`apps/desktop/src-tauri`, `core/reporting/report_package`,
//! `core/reporting/pdf_emitter`, `core/reporting/report_renderer`) with `syn`
//! and attributes every scanned name to its innermost lexical scope owner.
//!
//! - **Production source.** From each crate's `src/lib.rs` (and `src-tauri`'s
//!   `src/main.rs`) it follows every `mod` item, outlined and inline, and
//!   resolves the files itself. An item is test-only when one of its outer
//!   attributes is exactly `#[cfg(test)]`; every other item, whatever its
//!   `cfg`, is production. A `#[path]` on a production `mod`, a `cfg_attr`
//!   applying `path` at any depth and under any predicate (refused, not
//!   followed: the scan does not evaluate predicates), any `include!`, an
//!   `include_str!`/`include_bytes!` that is not one string literal or names
//!   a `.rs` file, a missing or ambiguous module file, and a file that does
//!   not parse are findings.
//! - **Spellings** (Codex round 1 at #1682). Identifiers are compared without
//!   `r#` (in the three reporting crates a raw identifier is itself a finding,
//!   below) and paths without a leading `::`; a prefix of only `crate`, `self`
//!   and `super` is the crate's own. Every other spelling fails closed:
//!   `<T as Trait>::` paths to scanned names, `Self::` and alias prefixes to
//!   the `src-tauri` renderer pair, other aliases of the renderer and PDF
//!   crates, imports of `ProducerEntry::UserRelease`, `DerivedUserRelease` in
//!   a type alias, attributes on its definition, and `unsafe` code in the
//!   three reporting crates are findings.
//! - **Macros.** No `macro_rules!` or `macro` item except `app_invoke_handler`,
//!   pinned by the SHA-256 of its source text; `#[macro_export]`,
//!   `#[macro_use]` and any `Item::Verbatim` are findings. Every macro
//!   invocation, in any position and inside another macro's tokens, must be
//!   on the list derived from the basis source, by its path as written.
//! - **Attributes, derives and imports.** Attribute paths, `serde` keys,
//!   `tauri::command` uses and derives are on closed lists, applied alike to
//!   the attributes a `cfg_attr` applies, at any depth; `extern crate`,
//!   glob imports and imports that rebind a listed macro, derive or crate
//!   name are findings; `report_package` and `pdf_emitter` re-export no
//!   renderer or PDF crate, and `src-tauri` imports neither, nor a scanned
//!   function.
//! - **Scanned names** occur only as their definition, as a direct call by an
//!   allowed caller that is the call's innermost scope owner (each exactly
//!   once), and, for `DerivedUserRelease`, in a type position or its one
//!   struct literal in `check_derived`. The producer entry argument is
//!   `&ProducerEntry::General` at the public roots, the forwarded `entry`
//!   inside `assemble_wire_request_in`, and `&ProducerEntry::UserRelease(derived)`
//!   only in `assemble_derived`, the one place `UserRelease` is constructed.
//! - **Other names of `DerivedUserRelease`** (Codex round 2 at #1682). No
//!   trait is implemented for it (the allow-list is empty), and an impl for it
//!   declares no associated type, so no associated type assigns it `Self`. In
//!   its defining module, the only code that can name its private field, a
//!   struct literal's path, outside a macro or in a macro's tokens, must be
//!   one plain name of a struct the `report_package` crate defines that no
//!   type alias, associated type or import rename shares, or `Self` in an
//!   impl of such a name: a `<T as Trait>::` path, generic arguments, a path
//!   of more than one segment and a leading `::` are findings.
//! - **Constructions** of `ProducerEntry::UserRelease` are its call, its value,
//!   its braced form (`UserRelease { 0: derived }`) and its name in any
//!   unparsed macro tokens. A `matches!` is parsed (`expr, pattern (if
//!   expr)?`) and visited as code, so only its pattern is read as a pattern,
//!   and an expression inside the pattern is visited as one (Codex round 2
//!   and code review r3 at #1682).
//! - **Conservative rules** (code review r3 at #1682). Rather than model each
//!   form, these close whole classes, failing closed: in the defining subtree,
//!   every type alias (item, block, impl or trait `type`, inside macro tokens
//!   included) is a finding, the allow-list being empty; only the macros on
//!   [`SUBTREE_MACROS`] may be invoked there, their tokens parsed by their
//!   input grammar and visited as code, and tokens that do not parse, or a
//!   brace group in any other tokens there, are findings. In the three
//!   reporting crates every raw identifier (allow-list empty) and every brace
//!   group in generic arguments, found on each read file's tokens, and every
//!   const generic argument but a literal, are findings. Anywhere, syntax
//!   `syn` gives as `Verbatim` is a finding.
//!
//! What it does not establish: code that builds a document without calling
//! the scanned functions, and a listed external macro whose version would
//! emit a call without naming it (§2.8, "What the scan does not
//! establish"). Standard claim fence applies (claims taxonomy per DEC-081).

use proc_macro2::{Span, TokenStream, TokenTree};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use syn::visit::{self, Visit};

/// The four crates: (key, source root, manifest).
pub(crate) const CRATES: [(&str, &str); 4] = [
    ("src-tauri", "apps/desktop/src-tauri"),
    ("report_package", "core/reporting/report_package"),
    ("pdf_emitter", "core/reporting/pdf_emitter"),
    ("report_renderer", "core/reporting/report_renderer"),
];

const RENDERER_CRATE: &str = "open_pipe_stress_report_renderer";
const PDF_CRATE: &str = "open_pipe_stress_pdf_emitter";
const PACKAGE_CRATE: &str = "open_pipe_stress_report_package";

/// The renderer's two document functions, matched in `src-tauri` only as the
/// path `open_pipe_stress_report_renderer::<name>` (the bare
/// `render_calculation_report` there is the Tauri command's).
const RENDERER_PAIR: [&str; 2] = ["render_calculation_report", "assemble_report_sections"];

/// One row of the table: the scanned function, the file that defines it, and
/// its allowed callers (file, free function).
type Row = (&'static str, &'static str, &'static [(&'static str, &'static str)]);

pub(crate) const ROWS: [Row; 12] = [
    (
        "render_calculation_report",
        "report_renderer/src/lib.rs",
        &[
            ("pdf_emitter/src/lib.rs", "emit_calculation_report_pdf"),
            ("pdf_emitter/src/lib.rs", "assemble_full_report_package"),
            ("src-tauri/src/lib.rs", "render_calculation_report_value"),
        ],
    ),
    (
        "assemble_report_sections",
        "report_renderer/src/lib.rs",
        &[("report_renderer/src/lib.rs", "render_calculation_report"), ("pdf_emitter/src/lib.rs", "emit_pdf_document")],
    ),
    ("emit_pdf_document", "pdf_emitter/src/lib.rs", &[("pdf_emitter/src/lib.rs", "pdf_from_render")]),
    (
        "pdf_from_render",
        "pdf_emitter/src/lib.rs",
        &[("pdf_emitter/src/lib.rs", "emit_calculation_report_pdf"), ("pdf_emitter/src/lib.rs", "assemble_full_report_package")],
    ),
    ("emit_calculation_report_pdf", "pdf_emitter/src/lib.rs", &[]),
    ("assemble_full_report_package", "pdf_emitter/src/lib.rs", &[("report_package/src/lib.rs", "assemble_report_package_container_in")]),
    (
        "assemble_report_package_container_in",
        "report_package/src/lib.rs",
        &[("report_package/src/lib.rs", "assemble_report_package_container"), ("report_package/src/wire.rs", "assemble_wire_request_in")],
    ),
    (
        "check_request_in",
        "report_package/src/bindings.rs",
        &[("report_package/src/bindings.rs", "check_request"), ("report_package/src/wire.rs", "assemble_wire_request_in")],
    ),
    (
        "assemble_wire_request_in",
        "report_package/src/wire.rs",
        &[("report_package/src/wire.rs", "assemble_wire_request"), ("report_package/src/user_release.rs", "assemble_derived")],
    ),
    ("check_derived", "report_package/src/user_release.rs", &[("report_package/src/user_release.rs", "assemble_derived")]),
    ("assemble_derived", "report_package/src/user_release.rs", &[("src-tauri/src/report_package_bridge.rs", "assemble_user_released_request")]),
    (
        "assemble_user_released_request",
        "src-tauri/src/report_package_bridge.rs",
        &[("src-tauri/src/report_package_save.rs", "admit_user_released")],
    ),
];

const DERIVED: &str = "DerivedUserRelease";
const DERIVED_FILE: &str = "report_package/src/user_release.rs";

/// The trait impls admitted for `DerivedUserRelease`, by trait path as
/// written: none exist today. Any other impl of a trait for it is a finding
/// (Codex round 2 at #1682, P2): an associated type it assigns `Self` names
/// the type under another path, which a struct literal could then construct.
const DERIVED_TRAIT_IMPLS: [&str; 0] = [];

/// The type aliases admitted in `DerivedUserRelease`'s defining subtree
/// (item, block, impl and trait `type` declarations, in any position), by
/// name: none exist today (code review r3 at #1682). Any other is a finding:
/// an alias can name the type under a name the literal rule would admit.
const SUBTREE_TYPE_ALIASES: [&str; 0] = [];

/// The macros the defining subtree may invoke, by path as written (code
/// review r3 at #1682). Each one's tokens are parsed into `syn`'s syntax tree
/// by its input grammar ([`parse_listed_macro`]) and visited as code, so every
/// rule applies inside them as outside; tokens that do not parse are a
/// finding. Any other macro there is a finding.
const SUBTREE_MACROS: [&str; 5] = ["format", "format_args", "vec", "matches", "json"];

/// The raw identifiers (`r#name`) admitted in the three reporting crates, by
/// name without `r#`: none exist today (code review r3 at #1682). Any other,
/// in any position of a file the scan reads, is a finding.
const RAW_IDENTIFIERS: [&str; 0] = [];

/// Whether `file` is in `DerivedUserRelease`'s defining module (its file, or a
/// file of a module beneath it): the only code that can name its private
/// field, so the only code that can write its literal.
fn in_defining_module(file: &str) -> bool {
    file == DERIVED_FILE || file.starts_with("report_package/src/user_release/")
}

/// Whether a type names `DerivedUserRelease` anywhere (`&DerivedUserRelease`,
/// `Wrapper<DerivedUserRelease>`, `<DerivedUserRelease as Trait>::Assoc`).
fn type_names_derived(ty: &syn::Type) -> bool {
    struct Finder(bool);
    impl<'ast> Visit<'ast> for Finder {
        fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
            if ident_text(ident) == DERIVED {
                self.0 = true;
            }
        }
        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            fn walk(tokens: TokenStream, found: &mut bool) {
                for tree in tokens {
                    match tree {
                        TokenTree::Ident(ident) if ident_text(&ident) == DERIVED => *found = true,
                        TokenTree::Group(group) => walk(group.stream(), found),
                        _ => {}
                    }
                }
            }
            walk(mac.tokens.clone(), &mut self.0);
        }
    }
    let mut finder = Finder(false);
    finder.visit_type(ty);
    finder.0
}

/// An enclosing `impl`'s self type, as a `Self { .. }` literal in it reads.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ImplSelf {
    /// It names `DerivedUserRelease`: `Self { .. }` constructs one.
    Derived,
    /// One plain identifier (with any generic arguments): `Self { .. }`
    /// constructs that type when it is a struct the crate defines.
    Named(String),
    /// Any other type, or a trait's default methods (no self type).
    Other,
}

/// The names `DerivedUserRelease`'s defining module may write as a struct
/// literal, read from the `report_package` crate: its structs, less every
/// name that could be an alias of another type (type aliases, associated
/// types, import renames).
#[derive(Default)]
struct TypeNames {
    structs: BTreeSet<String>,
    aliases: BTreeSet<String>,
}

/// The producer's internal entries whose entry argument is read, and what it
/// must be in each caller: (callee, caller, entry).
const ENTRY_RULES: [(&str, &str, Entry); 6] = [
    ("check_request_in", "check_request", Entry::General),
    ("assemble_wire_request_in", "assemble_wire_request", Entry::General),
    ("assemble_report_package_container_in", "assemble_report_package_container", Entry::General),
    ("check_request_in", "assemble_wire_request_in", Entry::Forwarded),
    ("assemble_report_package_container_in", "assemble_wire_request_in", Entry::Forwarded),
    ("assemble_wire_request_in", "assemble_derived", Entry::UserRelease),
];

/// The one allowed macro definition: (file, name, SHA-256 of its text).
const MACRO_DEFINITIONS: [(&str, &str, &str); 1] =
    [("src-tauri/src/lib.rs", "app_invoke_handler", "45c3d325100524094e8e57c33a3a1d13e12abb5cdf2abfed51224be5adbe1ed1")];

/// The macro invocations admitted, by path as written (§2.8 macro rule 2).
const MACRO_INVOCATIONS: [&str; 22] = [
    "format",
    "vec",
    "format_args",
    "matches",
    "write",
    "eprintln",
    "include_str",
    "assert",
    "unreachable",
    "debug_assert",
    "println",
    "debug_assert_ne",
    "panic",
    "env",
    "json",
    "params",
    "serde_json::json",
    "rusqlite::params",
    "serde::forward_to_deserialize_any",
    "tauri::generate_handler",
    "tauri::generate_context",
    "app_invoke_handler",
];

/// The attribute paths admitted (§2.8 macro rule 3).
const ATTRIBUTES: [&str; 11] = ["cfg", "cfg_attr", "allow", "inline", "cold", "global_allocator", "derive", "doc", "default", "serde", "tauri::command"];
/// The derives admitted, written bare.
const DERIVES: [&str; 10] = ["Clone", "Copy", "Debug", "Default", "Eq", "PartialEq", "Ord", "PartialOrd", "Serialize", "Deserialize"];
/// The `serde` keys admitted without a value, and with one.
const SERDE_FLAGS: [&str; 5] = ["default", "deny_unknown_fields", "flatten", "skip", "untagged"];
const SERDE_VALUED: [&str; 4] = ["rename", "rename_all", "skip_serializing_if", "tag"];
const SKIP_SERIALIZING_IF: [&str; 2] = ["Option::is_none", "std::ops::Not::not"];
/// The crate roots of the listed external paths, and the items an import may
/// bring in under a listed name: (bound name, the import path that may bind it).
const LISTED_ROOTS: [&str; 4] = ["serde", "serde_json", "rusqlite", "tauri"];
/// `serde::de::Deserialize` and `serde::ser::Serialize` are the traits the
/// derives implement; importing them binds the type namespace only, never the
/// derive macro, so they are listed sources of those names too.
const LISTED_IMPORTS: [(&str, &str); 6] = [
    ("json", "serde_json::json"),
    ("params", "rusqlite::params"),
    ("Serialize", "serde::Serialize"),
    ("Deserialize", "serde::Deserialize"),
    ("Serialize", "serde::ser::Serialize"),
    ("Deserialize", "serde::de::Deserialize"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    General,
    Forwarded,
    UserRelease,
}

/// A lexical scope owner.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Owner {
    Module,
    /// A free `fn` item.
    Fn(String),
    /// A method or associated `fn` in an `impl` or trait.
    Method(String),
    Closure,
    Async,
    Const,
    Static,
    Macro,
}

/// What a scanned name's occurrence is.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind {
    Definition,
    Call,
    TypeUse,
    Literal,
    Other(&'static str),
}

#[derive(Clone, Debug)]
struct Occurrence {
    name: String,
    file: String,
    line: usize,
    kind: Kind,
    owner: Owner,
}

/// The sources the scan reads: crate files by `<crate key>/src/...` path, and
/// each crate's manifest text and whether it has a `src/bin` directory.
#[derive(Clone, Default)]
pub(crate) struct Sources {
    pub(crate) files: BTreeMap<String, String>,
    pub(crate) manifests: BTreeMap<String, String>,
    pub(crate) bin_dirs: BTreeSet<String>,
}

fn crate_of(file: &str) -> &str {
    file.split('/').next().unwrap_or("")
}

fn line_of(span: Span) -> usize {
    span.start().line
}

fn is_test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| match &attr.meta {
        syn::Meta::List(list) => list.path.is_ident("cfg") && list.tokens.to_string() == "test",
        _ => false,
    })
}

/// An identifier as the scan compares it: a raw identifier without its `r#`
/// (`r#name` and `name` are the same identifier; Codex round 1 at #1682, P2).
fn ident_text(ident: &proc_macro2::Ident) -> String {
    syn::ext::IdentExt::unraw(ident).to_string()
}

/// A path's segments as the scan compares them ([`ident_text`]), without a
/// leading `::`: in the 2018+ editions `::name` and `name` name the same
/// external crate, and a local item cannot be named after `::`.
fn segments(path: &syn::Path) -> Vec<String> {
    path.segments.iter().map(|segment| ident_text(&segment.ident)).collect()
}

/// [`segments`], joined by `::`: the normalized text of every path the scan
/// matches against its lists (attributes, derives, `serde` keys, macro
/// invocations, the entry argument).
fn path_text(path: &syn::Path) -> String {
    segments(path).join("::")
}

/// Whether a path prefix names only the current crate's own modules
/// (`crate`, `self`, `super`, as the scan resolves modules): in `src-tauri`
/// such a path to a renderer-pair name is the local item of that name.
fn local_prefix(prefix: &[String]) -> bool {
    prefix.iter().all(|segment| matches!(segment.as_str(), "crate" | "self" | "super"))
}

/// Every attribute `attrs` apply, `cfg_attr` flattened at any depth (each
/// conditional attribute counted whatever its predicate).
fn applied_paths(attrs: &[syn::Attribute]) -> Vec<String> {
    fn flatten(meta: &syn::Meta, out: &mut Vec<String>) {
        let path = path_text(meta.path());
        if let (syn::Meta::List(list), "cfg_attr") = (meta, path.as_str()) {
            if let Ok(metas) = list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated) {
                for inner in metas.iter().skip(1) {
                    flatten(inner, out);
                }
            }
        }
        out.push(path);
    }
    let mut out = Vec::new();
    for attr in attrs {
        flatten(&attr.meta, &mut out);
    }
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The byte offset of a span position (1-based line, 0-based column in chars).
fn byte_offset(text: &str, line: usize, column: usize) -> Option<usize> {
    let start = if line == 1 { 0 } else { text.match_indices('\n').nth(line - 2)?.0 + 1 };
    let rest = &text[start..];
    Some(start + rest.char_indices().nth(column).map_or(rest.len(), |(at, _)| at))
}

/// One file's scan.
struct FileScan<'a> {
    file: String,
    text: &'a str,
    findings: &'a mut Vec<String>,
    occurrences: &'a mut Vec<Occurrence>,
    /// Outlined modules to resolve: (declaring file, inline module path, name, line).
    modules: &'a mut Vec<(String, Vec<String>, String, usize)>,
    owners: Vec<Owner>,
    inline: Vec<String>,
    /// Entry arguments seen at calls of the producer internals: (callee, caller owner, entry, line).
    entries: &'a mut Vec<(String, Owner, Option<Entry>, String, usize)>,
    /// `UserRelease` constructions: (owner, file, line).
    constructions: &'a mut Vec<(Owner, String, usize)>,
    /// Admitted macro definitions seen: (file, name).
    macros: &'a mut Vec<(String, String)>,
    /// For each enclosing `impl`, its self type ([`ImplSelf`]): a
    /// `Self { .. }` literal there constructs it.
    impl_self: Vec<ImplSelf>,
    /// The `report_package` crate's struct and alias names.
    type_names: &'a mut TypeNames,
    /// Struct literals in the defining module that name one plain identifier,
    /// checked against [`TypeNames`] once every file is read: (file, line,
    /// name, how it was written).
    plain_literals: &'a mut Vec<(String, usize, String, &'static str)>,
    /// Inside a type alias (`type X = ...;`), where naming `DerivedUserRelease`
    /// would give it a name the literal rule does not match.
    in_type_alias: bool,
    /// Visiting a listed macro's parsed tokens ([`parse_listed_macro`]): the
    /// findings say so.
    in_macro: bool,
}

impl FileScan<'_> {
    fn crate_key(&self) -> &str {
        crate_of(&self.file)
    }

    /// Whether this file is one of the three reporting crates, which hold no
    /// `unsafe` code: `unsafe` could build a `DerivedUserRelease` without
    /// naming it (`std::mem::zeroed`, `transmute`), so it is refused there.
    fn is_core(&self) -> bool {
        self.crate_key() != "src-tauri"
    }

    /// A type alias's or an associated type's name in `report_package`. In the
    /// defining subtree every one not on [`SUBTREE_TYPE_ALIASES`] is a finding,
    /// whatever it names and wherever it is declared (code review r3 at #1682).
    fn alias(&mut self, ident: &proc_macro2::Ident) {
        let name = ident_text(ident);
        if in_defining_module(&self.file) && !SUBTREE_TYPE_ALIASES.contains(&name.as_str()) {
            self.finding(line_of(ident.span()), format!("the type alias {name} in the defining module of {DERIVED}, which is not on the alias allow-list"));
        }
        if self.crate_key() == "report_package" {
            self.type_names.aliases.insert(name);
        }
    }

    /// Code `syn` gives as `Verbatim` (syntax it does not model): a finding,
    /// never skipped.
    fn unparsed(&mut self, tokens: &TokenStream, what: &str) {
        let line = tokens.clone().into_iter().next().map_or(0, |tree| line_of(tree.span()));
        self.finding(line, format!("{what} syn gives as Verbatim (syntax the scan does not model)"));
    }

    /// The token rules over a whole file of the three reporting crates (code
    /// review r3 at #1682): every raw identifier not on [`RAW_IDENTIFIERS`],
    /// and every brace group in generic arguments ([`generic_blocks`]), in
    /// any position, macro tokens, attributes and inline test modules
    /// included. A file that does not lex is a finding.
    fn token_rules(&mut self) {
        let Ok(tokens) = self.text.parse::<TokenStream>() else {
            self.finding(0, "a file that does not lex");
            return;
        };
        let mut found = Vec::new();
        lexical_findings(tokens, &mut found);
        for (line, what) in found {
            self.finding(line, what);
        }
    }

    fn unsafe_found(&mut self, line: usize, what: &str) {
        if self.is_core() {
            self.finding(line, format!("{what} in a reporting crate"));
        }
    }

    fn owner(&self) -> Owner {
        self.owners.last().cloned().unwrap_or(Owner::Module)
    }

    fn finding(&mut self, line: usize, what: impl std::fmt::Display) {
        self.findings.push(format!("{}:{line}: {what}", self.file));
    }

    /// Whether `name` is a scanned name matched bare in this crate.
    fn scanned_bare(&self, name: &str) -> bool {
        let scanned = ROWS.iter().any(|(function, _, _)| *function == name) || name == DERIVED;
        scanned && !(self.crate_key() == "src-tauri" && RENDERER_PAIR.contains(&name))
    }

    fn record(&mut self, name: &str, line: usize, kind: Kind) {
        let owner = self.owner();
        self.occurrences.push(Occurrence { name: name.to_string(), file: self.file.clone(), line, kind, owner });
    }

    fn with_owner(&mut self, owner: Owner, body: impl FnOnce(&mut Self)) {
        self.owners.push(owner);
        body(self);
        self.owners.pop();
    }

    /// A scanned name used through `path` in this crate (a call, a type or
    /// anything else): the name, or `None`.
    ///
    /// Names are compared normalized ([`segments`]). In `src-tauri` a
    /// renderer-pair name is the local item when the path is bare or its
    /// prefix is only `crate`, `self` or `super`; through any other prefix
    /// (the renderer crate, `::` and the crate, `Self`, an alias, a
    /// re-export) it is scanned, failing closed.
    fn scanned_in_path(&self, path: &syn::Path) -> Option<String> {
        let segments = segments(path);
        let (last, prefix) = segments.split_last()?;
        if self.crate_key() == "src-tauri" && RENDERER_PAIR.contains(&last.as_str()) {
            return (!local_prefix(prefix)).then(|| last.clone());
        }
        self.scanned_bare(last).then(|| last.clone())
    }

    /// Screens a token stream the scan does not parse (an attribute's
    /// arguments, or a macro's outside the defining subtree or that did not
    /// parse): every scanned name in it, every `UserRelease` as a
    /// construction, and every nested macro invocation's path against the
    /// list. In the defining subtree a brace group in such tokens is a finding
    /// (code review r3 at #1682): it could be a struct literal or a block the
    /// scan would have to model.
    fn screen_tokens(&mut self, tokens: &TokenStream, check_invocations: bool) {
        let trees: Vec<TokenTree> = tokens.clone().into_iter().collect();
        for (index, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Group(group) => {
                    if group.delimiter() == proc_macro2::Delimiter::Brace && in_defining_module(&self.file) {
                        self.finding(line_of(group.span()), format!("a brace group in tokens the scan does not parse, in the defining module of {DERIVED}"));
                    }
                    self.screen_tokens(&group.stream(), check_invocations)
                }
                TokenTree::Ident(ident) => {
                    let name = ident_text(ident);
                    let line = line_of(ident.span());
                    if self.scanned_bare(&name) {
                        self.record(&name, line, Kind::Other("named in a macro's or an attribute's tokens"));
                    } else if self.crate_key() == "src-tauri" && RENDERER_PAIR.contains(&name.as_str()) && index >= 3 {
                        // `<prefix> :: <pair name>` as tokens, the prefix other than
                        // `crate`, `self` or `super` (as `scanned_in_path`).
                        if let (TokenTree::Ident(previous), TokenTree::Punct(a), TokenTree::Punct(b)) = (&trees[index - 3], &trees[index - 2], &trees[index - 1]) {
                            if a.as_char() == ':' && b.as_char() == ':' && !local_prefix(&[ident_text(previous)]) {
                                self.record(&name, line, Kind::Other("named in a macro's or an attribute's tokens"));
                            }
                        }
                    }
                    if name == "UserRelease" {
                        self.constructions.push((Owner::Macro, self.file.clone(), line));
                    }
                    // A nested invocation: `<path> ! <group>`.
                    if check_invocations {
                        if let (Some(TokenTree::Punct(bang)), Some(TokenTree::Group(_))) = (trees.get(index + 1), trees.get(index + 2)) {
                            if bang.as_char() == '!' {
                                let mut segments = vec![name.clone()];
                                let mut at = index;
                                while at >= 3 {
                                    match (&trees[at - 1], &trees[at - 2], &trees[at - 3]) {
                                        (TokenTree::Punct(a), TokenTree::Punct(b), TokenTree::Ident(previous)) if a.as_char() == ':' && b.as_char() == ':' => {
                                            segments.insert(0, ident_text(previous));
                                            at -= 3;
                                        }
                                        _ => break,
                                    }
                                }
                                let path = segments.join("::");
                                if path != "macro_rules" && !MACRO_INVOCATIONS.contains(&path.as_str()) {
                                    self.finding(line, format!("the macro {path}! is not on the invocation list"));
                                }
                                if path == "include" {
                                    self.finding(line, "include! brings in source the scan does not read");
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn check_attribute(&mut self, attr: &syn::Attribute) {
        self.check_meta(&attr.meta, line_of(attr.pound_token.span), false);
    }

    /// One attribute, direct or applied by a `cfg_attr` (`through`), by the
    /// same rules at any depth (Codex round 1 at #1682, P2): the attribute
    /// list, `#[macro_export]` and `#[macro_use]`, the derive list, the
    /// `serde` keys, `tauri::command`'s form, a nested `cfg_attr` checked in
    /// turn, and each other list's tokens screened. A `path` applied by a
    /// `cfg_attr` is refused whatever its predicate: the scan does not
    /// evaluate predicates, so it could not know which file the module loads.
    fn check_meta(&mut self, meta: &syn::Meta, line: usize, through: bool) {
        let path = path_text(meta.path());
        if path == "macro_export" || path == "macro_use" {
            self.finding(line, format!("#[{path}]"));
            return;
        }
        if through && path == "path" {
            self.finding(line, "a cfg_attr that applies #[path]");
            return;
        }
        if !ATTRIBUTES.contains(&path.as_str()) {
            let via = if through { " (through cfg_attr)" } else { "" };
            self.finding(line, format!("the attribute #[{path}]{via} is not on the attribute list"));
            return;
        }
        match (meta, path.as_str()) {
            (syn::Meta::List(list), "cfg_attr") => self.check_cfg_attr(list, line),
            (syn::Meta::List(list), "derive") => {
                let parsed = list.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated);
                match parsed {
                    Ok(derives) => {
                        for derive in derives {
                            let name = path_text(&derive);
                            if !DERIVES.contains(&name.as_str()) {
                                self.finding(line, format!("the derive {name} is not on the derive list"));
                            }
                        }
                    }
                    Err(_) => self.finding(line, "a derive list that does not parse"),
                }
            }
            (syn::Meta::List(list), "serde") => self.check_serde(list, line),
            (syn::Meta::Path(_), "tauri::command") | (syn::Meta::Path(_), "default") => {}
            (_, "tauri::command") => self.finding(line, "#[tauri::command] with arguments"),
            (syn::Meta::List(list), _) => self.screen_tokens(&list.tokens, true),
            (syn::Meta::NameValue(value), _) => self.visit_expr(&value.value),
            _ => {}
        }
    }

    /// `cfg_attr(predicate, attribute, ...)`: each attribute it applies is
    /// checked as a direct one ([`Self::check_meta`]), nested `cfg_attr`s
    /// included.
    fn check_cfg_attr(&mut self, list: &syn::MetaList, line: usize) {
        let parsed = list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
        let Ok(metas) = parsed else {
            self.finding(line, "a cfg_attr that does not parse");
            return;
        };
        for meta in metas.iter().skip(1) {
            self.check_meta(meta, line, true);
        }
    }

    fn check_serde(&mut self, list: &syn::MetaList, line: usize) {
        let parsed = list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
        let Ok(metas) = parsed else {
            self.finding(line, "a serde attribute that does not parse");
            return;
        };
        for meta in metas {
            let key = path_text(meta.path());
            match &meta {
                syn::Meta::Path(_) if SERDE_FLAGS.contains(&key.as_str()) => {}
                syn::Meta::NameValue(value) if SERDE_VALUED.contains(&key.as_str()) => {
                    if key == "skip_serializing_if" {
                        let literal = match &value.value {
                            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(text), .. }) => Some(text.value()),
                            _ => None,
                        };
                        if !literal.as_deref().is_some_and(|text| SKIP_SERIALIZING_IF.contains(&text)) {
                            self.finding(line, "a skip_serializing_if naming a function outside the two admitted");
                        }
                    }
                }
                _ => self.finding(line, format!("the serde key {key} is not on the serde key list")),
            }
        }
    }

    fn check_attributes(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            self.visit_attribute(attr);
        }
    }

    /// A production macro invocation: its path against the list, the
    /// `include*!` rules, and its tokens. In the defining subtree only the
    /// macros on [`SUBTREE_MACROS`] are admitted, and their tokens are parsed
    /// and visited as code; a `matches!` anywhere is parsed and visited the
    /// same way, so only its pattern is read as a pattern (Codex round 2 and
    /// code review r3 at #1682). Tokens that do not parse are a finding and
    /// are screened as tokens, constructions counted; every other macro's
    /// tokens are screened ([`Self::screen_tokens`]).
    fn check_invocation(&mut self, mac: &syn::Macro) {
        let path = path_text(&mac.path);
        let line = line_of(mac.bang_token.span);
        if !MACRO_INVOCATIONS.contains(&path.as_str()) {
            self.finding(line, format!("the macro {path}! is not on the invocation list"));
        }
        if path == "include" {
            self.finding(line, "include! brings in source the scan does not read");
        }
        if path == "include_str" || path == "include_bytes" {
            let literal = syn::parse2::<syn::LitStr>(mac.tokens.clone()).ok().map(|text| text.value());
            match literal {
                Some(text) if !text.ends_with(".rs") => {}
                _ => self.finding(line, format!("{path}! takes one string literal that names no .rs file")),
            }
        }
        let subtree = in_defining_module(&self.file);
        let listed = SUBTREE_MACROS.contains(&path.as_str());
        if subtree && !listed {
            self.finding(line, format!("the macro {path}! in the defining module of {DERIVED} is not on its parsed-macro list"));
        }
        let parsed = if (subtree && listed) || path == "matches" { parse_listed_macro(&path, &mac.tokens) } else { None };
        match parsed {
            Some(body) => self.with_owner(Owner::Macro, |scan| {
                let outer = std::mem::replace(&mut scan.in_macro, true);
                for expr in &body.exprs {
                    scan.visit_expr(expr);
                }
                if let Some(pattern) = &body.pattern {
                    scan.visit_pat(pattern);
                }
                for expr in &body.after {
                    scan.visit_expr(expr);
                }
                scan.in_macro = outer;
            }),
            None => {
                if path == "matches" {
                    self.finding(line, "a matches! that does not parse as an expression, a pattern and a guard");
                } else if subtree && listed {
                    self.finding(line, format!("the tokens of {path}! do not parse by its grammar, in the defining module of {DERIVED}"));
                }
                self.with_owner(Owner::Macro, |scan| scan.screen_tokens(&mac.tokens, true));
            }
        }
    }

    /// A struct literal's path in `DerivedUserRelease`'s defining module
    /// (Codex round 2 at #1682, P2). Only one plain identifier is admitted,
    /// and only as a struct the crate defines that no alias shares
    /// ([`TypeNames`], checked once every file is read), or `Self` in an impl
    /// of such a struct: a qualified `<T as Trait>::` path, generic
    /// arguments, a path of more than one segment (`Self::Assoc`,
    /// `module::Alias`, an enum variant) and a leading `::` could each
    /// resolve to `DerivedUserRelease` without naming it, so each is a
    /// finding. `DerivedUserRelease` itself is held to its literal rule. In a
    /// listed macro's parsed tokens the same rules apply, and the findings
    /// say so.
    fn check_literal_path(&mut self, qself: Option<&syn::QSelf>, path: &syn::Path, line: usize) {
        let written = segments(path).join("::");
        let last = segments(path).last().cloned().unwrap_or_default();
        let generic = path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None));
        let qualified = qself.is_some() || path.leading_colon.is_some() || path.segments.len() != 1;
        if self.in_macro && generic {
            self.finding(line, format!("a struct literal with generic arguments in a macro's tokens ({written}), in the defining module of {DERIVED}"));
        } else if self.in_macro && qualified {
            self.finding(line, format!("a struct literal of a qualified path ending in {last} in a macro's tokens ({written}), in the defining module of {DERIVED}"));
        } else if qself.is_some() {
            self.finding(line, format!("a struct literal through a qualified <T as Trait>:: path ({written}) in the defining module of {DERIVED}"));
        } else if generic {
            self.finding(line, format!("a struct literal with generic arguments ({written}) in the defining module of {DERIVED}"));
        } else if qualified {
            self.finding(line, format!("a struct literal of {written} in the defining module of {DERIVED}, which is not one plain struct name"));
        } else if written == "Self" {
            self.check_self_literal(line, if self.in_macro { "a Self literal in a macro's tokens" } else { "a Self literal" });
        } else if written != DERIVED {
            let how = if self.in_macro { "a struct literal in a macro's tokens" } else { "a struct literal" };
            self.plain_literals.push((self.file.clone(), line, written, how));
        }
    }

    /// `Self { .. }` in the defining module: admitted in an impl of a plain
    /// struct name ([`Self::check_literal_path`]); in an impl of
    /// `DerivedUserRelease` it is that type's literal, held to its rule; in
    /// any other impl or a trait's default method it is a finding.
    fn check_self_literal(&mut self, line: usize, how: &'static str) {
        match self.impl_self.last().cloned() {
            Some(ImplSelf::Derived) => {}
            Some(ImplSelf::Named(name)) => self.plain_literals.push((self.file.clone(), line, name, how)),
            _ => self.finding(line, format!("{how} outside an impl of one plain struct name, in the defining module of {DERIVED}")),
        }
    }

    /// A production `macro_rules!` definition: only the pinned one.
    fn check_macro_definition(&mut self, item: &syn::ItemMacro) {
        let name = item.ident.as_ref().map(ident_text).unwrap_or_default();
        let line = line_of(item.mac.bang_token.span);
        let start = item.mac.path.segments.first().map(|segment| segment.ident.span().start());
        let end = match &item.mac.delimiter {
            syn::MacroDelimiter::Paren(delimiter) => delimiter.span.close().end(),
            syn::MacroDelimiter::Brace(delimiter) => delimiter.span.close().end(),
            syn::MacroDelimiter::Bracket(delimiter) => delimiter.span.close().end(),
        };
        let text = start.and_then(|start| {
            let from = byte_offset(self.text, start.line, start.column)?;
            let to = byte_offset(self.text, end.line, end.column)?;
            self.text.get(from..to)
        });
        let pinned = MACRO_DEFINITIONS.iter().find(|(file, macro_name, _)| *file == self.file && *macro_name == name);
        match (pinned, text) {
            (Some((_, _, sha)), Some(text)) if sha256_hex(text.as_bytes()) == *sha => {
                self.macros.push((self.file.clone(), name.clone()));
                // Admitted only with no `mod` token and no `$param ::`.
                let mut admitted = true;
                fn walk(tokens: TokenStream, admitted: &mut bool) {
                    let trees: Vec<TokenTree> = tokens.into_iter().collect();
                    for (index, tree) in trees.iter().enumerate() {
                        match tree {
                            TokenTree::Group(group) => walk(group.stream(), admitted),
                            TokenTree::Ident(ident) if *ident == "mod" => *admitted = false,
                            TokenTree::Punct(dollar) if dollar.as_char() == '$' => {
                                if let (Some(TokenTree::Ident(_)), Some(TokenTree::Punct(colon)), Some(TokenTree::Punct(second))) =
                                    (trees.get(index + 1), trees.get(index + 2), trees.get(index + 3))
                                {
                                    if colon.as_char() == ':' && second.as_char() == ':' {
                                        *admitted = false;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                walk(item.mac.tokens.clone(), &mut admitted);
                if !admitted {
                    self.finding(line, format!("the pinned macro {name} holds a mod token or a $parameter before ::"));
                }
            }
            (Some(_), _) => self.finding(line, format!("the macro {name} differs from its pinned text (review it and replace its pin)")),
            (None, _) => self.finding(line, format!("the macro definition {name} is not on the allow-list")),
        }
        self.with_owner(Owner::Macro, |scan| scan.screen_tokens(&item.mac.tokens, true));
    }

    fn check_use(&mut self, item: &syn::ItemUse) {
        let line = line_of(item.use_token.span);
        let public = !matches!(item.vis, syn::Visibility::Inherited);
        let mut bound = Vec::new();
        fn walk(tree: &syn::UseTree, prefix: Vec<String>, out: &mut Vec<(Vec<String>, Option<String>, bool)>) {
            match tree {
                syn::UseTree::Path(path) => {
                    let mut next = prefix;
                    next.push(ident_text(&path.ident));
                    walk(&path.tree, next, out);
                }
                syn::UseTree::Name(name) => {
                    let mut full = prefix;
                    full.push(ident_text(&name.ident));
                    out.push((full, None, false));
                }
                syn::UseTree::Rename(rename) => {
                    let mut full = prefix;
                    full.push(ident_text(&rename.ident));
                    out.push((full, Some(ident_text(&rename.rename)), false));
                }
                syn::UseTree::Glob(_) => out.push((prefix, None, true)),
                syn::UseTree::Group(group) => {
                    for item in &group.items {
                        walk(item, prefix.clone(), out);
                    }
                }
            }
        }
        walk(&item.tree, Vec::new(), &mut bound);
        let crate_key = self.crate_key().to_string();
        for (path, rename, glob) in bound {
            let joined = path.join("::");
            if glob {
                self.finding(line, format!("a glob import of {joined}"));
                continue;
            }
            let last = path.last().cloned().unwrap_or_default();
            if let (Some(rename), "report_package") = (&rename, crate_key.as_str()) {
                // A renamed import could name any type under a struct's name.
                self.type_names.aliases.insert(rename.clone());
            }
            let name = rename.clone().unwrap_or_else(|| if last == "self" { path.iter().rev().nth(1).cloned().unwrap_or_default() } else { last.clone() });
            let listed_name = MACRO_INVOCATIONS.contains(&name.as_str()) || DERIVES.contains(&name.as_str()) || LISTED_ROOTS.contains(&name.as_str());
            let from_source = LISTED_IMPORTS.iter().any(|(bound_name, source)| *bound_name == name && *source == joined && rename.is_none())
                || (LISTED_ROOTS.contains(&name.as_str()) && path.len() == 1 && rename.is_none())
                || (LISTED_ROOTS.contains(&name.as_str()) && last == "self" && path.len() == 2 && path[0] == name && rename.is_none());
            if listed_name && !from_source {
                self.finding(line, format!("the import {joined} binds the listed name {name}"));
            }
            // The path from its first segment that is not `crate`, `self` or
            // `super`: `crate::pdf` names the crate root's `pdf` alias.
            let resolved: &[String] = &path[path.iter().take_while(|segment| matches!(segment.as_str(), "crate" | "self" | "super")).count()..];
            let names_crate = |krate: &str| resolved.first().is_some_and(|first| first == krate);
            match crate_key.as_str() {
                "report_package" | "pdf_emitter" => {
                    let renderer = names_crate(RENDERER_CRATE) || names_crate("renderer");
                    let pdf = names_crate(PDF_CRATE) || names_crate("pdf");
                    // The renderer and PDF crates are bound only under the basis
                    // aliases, so a re-export through another name is found.
                    let crate_itself = resolved.len() == 1 || (resolved.len() == 2 && last == "self");
                    let alias = if renderer { "renderer" } else { "pdf" };
                    if (renderer || pdf) && crate_itself && name != alias {
                        self.finding(line, format!("the import {joined} binds the renderer or PDF crate as {name}"));
                    }
                    let exception = joined == format!("{RENDERER_CRATE}::serde_error_kind") && rename.is_none();
                    if public && (renderer || pdf) && !exception {
                        self.finding(line, format!("a {} use of {joined} re-exports the renderer or PDF crate", "non-private"));
                    }
                }
                "src-tauri" => {
                    if names_crate(RENDERER_CRATE) || names_crate(PDF_CRATE) || path.iter().any(|segment| ROWS.iter().any(|(function, _, _)| function == segment)) {
                        self.finding(line, format!("src-tauri imports {joined}"));
                    }
                }
                _ => {}
            }
            // `UserRelease` under another name would construct it unseen.
            if last == "UserRelease" {
                self.finding(line, format!("the import {joined} binds ProducerEntry::UserRelease"));
            }
            if path.iter().any(|segment| self.scanned_bare(segment)) {
                let scanned = path.iter().find(|segment| self.scanned_bare(segment)).cloned().unwrap_or_default();
                self.record(&scanned, line, Kind::Other("named in a use"));
            }
        }
        let _ = public;
    }

    /// The entry argument (the one before `admitted`) of a call of a producer
    /// internal.
    fn entry_of(args: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>) -> (Option<Entry>, String) {
        let Some(argument) = args.iter().rev().nth(1) else { return (None, "no entry argument".to_string()) };
        let path_of = |expr: &syn::Expr| match expr {
            syn::Expr::Path(path) => Some(path_text(&path.path)),
            _ => None,
        };
        match argument {
            syn::Expr::Path(path) if path_text(&path.path) == "entry" => (Some(Entry::Forwarded), "entry".to_string()),
            syn::Expr::Reference(reference) => match &*reference.expr {
                syn::Expr::Path(path) if path_text(&path.path) == "ProducerEntry::General" => (Some(Entry::General), "&ProducerEntry::General".to_string()),
                syn::Expr::Call(call)
                    if path_of(&call.func).as_deref() == Some("ProducerEntry::UserRelease")
                        && call.args.len() == 1
                        && path_of(&call.args[0]).as_deref() == Some("derived") =>
                {
                    (Some(Entry::UserRelease), "&ProducerEntry::UserRelease(derived)".to_string())
                }
                _ => (None, "another expression".to_string()),
            },
            _ => (None, "another expression".to_string()),
        }
    }
}

impl<'ast> Visit<'ast> for FileScan<'_> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        let attrs: &[syn::Attribute] = match item {
            syn::Item::Const(i) => &i.attrs,
            syn::Item::Enum(i) => &i.attrs,
            syn::Item::ExternCrate(i) => &i.attrs,
            syn::Item::Fn(i) => &i.attrs,
            syn::Item::ForeignMod(i) => &i.attrs,
            syn::Item::Impl(i) => &i.attrs,
            syn::Item::Macro(i) => &i.attrs,
            syn::Item::Mod(i) => &i.attrs,
            syn::Item::Static(i) => &i.attrs,
            syn::Item::Struct(i) => &i.attrs,
            syn::Item::Trait(i) => &i.attrs,
            syn::Item::TraitAlias(i) => &i.attrs,
            syn::Item::Type(i) => &i.attrs,
            syn::Item::Union(i) => &i.attrs,
            syn::Item::Use(i) => &i.attrs,
            _ => &[],
        };
        if is_test_only(attrs) {
            return;
        }
        match item {
            syn::Item::Verbatim(tokens) => {
                let line = tokens.clone().into_iter().next().map_or(0, |tree| line_of(tree.span()));
                self.finding(line, "an item syn gives as Verbatim (a macro 2.0 item)");
            }
            syn::Item::ExternCrate(extern_crate) => {
                self.check_attributes(&extern_crate.attrs);
                self.finding(line_of(extern_crate.crate_token.span), format!("extern crate {}", extern_crate.ident));
            }
            _ => visit::visit_item(self, item),
        }
    }

    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        self.check_attribute(attr);
    }

    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        let line = line_of(module.mod_token.span);
        let name = ident_text(&module.ident);
        for attr in &module.attrs {
            if path_text(attr.path()) == "path" {
                self.finding(line, format!("a #[path] on the production module {name}"));
            } else {
                // A `path` applied through `cfg_attr` is refused there.
                self.visit_attribute(attr);
            }
        }
        match &module.content {
            Some((_, items)) => {
                self.inline.push(name.clone());
                for item in items {
                    self.visit_item(item);
                }
                self.inline.pop();
            }
            None => self.modules.push((self.file.clone(), self.inline.clone(), name, line)),
        }
    }

    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        self.check_attributes(&function.attrs);
        let name = ident_text(&function.sig.ident);
        let line = line_of(function.sig.ident.span());
        if self.scanned_bare(&name) {
            self.record(&name, line, Kind::Definition);
        }
        if function.sig.unsafety.is_some() {
            self.unsafe_found(line, "an unsafe fn");
        }
        // Direct or through `cfg_attr`, at any depth.
        let command = applied_paths(&function.attrs).iter().any(|path| path == "tauri::command");
        let scanned = ROWS.iter().any(|(row, _, _)| *row == name) || name == DERIVED;
        if command && scanned && !(self.file == "src-tauri/src/lib.rs" && name == "render_calculation_report") {
            self.finding(line, format!("#[tauri::command] marks the scanned function {name}"));
        }
        self.with_owner(Owner::Fn(name), |scan| {
            visit::visit_generics(scan, &function.sig.generics);
            for input in &function.sig.inputs {
                scan.visit_fn_arg(input);
            }
            scan.visit_return_type(&function.sig.output);
            scan.visit_block(&function.block);
        });
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        let attrs: &[syn::Attribute] = match item {
            syn::ImplItem::Const(i) => &i.attrs,
            syn::ImplItem::Fn(i) => &i.attrs,
            syn::ImplItem::Type(i) => &i.attrs,
            syn::ImplItem::Macro(i) => &i.attrs,
            syn::ImplItem::Verbatim(tokens) => {
                self.unparsed(tokens, "an impl item");
                return;
            }
            _ => &[],
        };
        if !is_test_only(attrs) {
            visit::visit_impl_item(self, item);
        }
    }

    fn visit_impl_item_fn(&mut self, function: &'ast syn::ImplItemFn) {
        self.check_attributes(&function.attrs);
        let name = ident_text(&function.sig.ident);
        if function.sig.unsafety.is_some() {
            self.unsafe_found(line_of(function.sig.ident.span()), "an unsafe fn");
        }
        if self.scanned_bare(&name) {
            self.record(&name, line_of(function.sig.ident.span()), Kind::Other("a method of that name"));
        }
        self.with_owner(Owner::Method(name), |scan| {
            visit::visit_generics(scan, &function.sig.generics);
            for input in &function.sig.inputs {
                scan.visit_fn_arg(input);
            }
            scan.visit_return_type(&function.sig.output);
            scan.visit_block(&function.block);
        });
    }

    fn visit_trait_item_fn(&mut self, function: &'ast syn::TraitItemFn) {
        if is_test_only(&function.attrs) {
            return;
        }
        self.check_attributes(&function.attrs);
        let name = ident_text(&function.sig.ident);
        if function.sig.unsafety.is_some() {
            self.unsafe_found(line_of(function.sig.ident.span()), "an unsafe fn");
        }
        if self.scanned_bare(&name) {
            self.record(&name, line_of(function.sig.ident.span()), Kind::Other("a trait method of that name"));
        }
        self.with_owner(Owner::Method(name), |scan| {
            visit::visit_generics(scan, &function.sig.generics);
            for input in &function.sig.inputs {
                scan.visit_fn_arg(input);
            }
            scan.visit_return_type(&function.sig.output);
            if let Some(block) = &function.default {
                scan.visit_block(block);
            }
        });
    }

    fn visit_expr_closure(&mut self, closure: &'ast syn::ExprClosure) {
        self.with_owner(Owner::Closure, |scan| visit::visit_expr_closure(scan, closure));
    }

    fn visit_expr_async(&mut self, block: &'ast syn::ExprAsync) {
        self.with_owner(Owner::Async, |scan| visit::visit_expr_async(scan, block));
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        self.with_owner(Owner::Const, |scan| visit::visit_item_const(scan, item));
    }

    fn visit_impl_item_const(&mut self, item: &'ast syn::ImplItemConst) {
        self.with_owner(Owner::Const, |scan| visit::visit_impl_item_const(scan, item));
    }

    fn visit_item_static(&mut self, item: &'ast syn::ItemStatic) {
        self.with_owner(Owner::Static, |scan| visit::visit_item_static(scan, item));
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        self.check_attributes(&item.attrs);
        let name = ident_text(&item.ident);
        if self.crate_key() == "report_package" {
            self.type_names.structs.insert(name.clone());
        }
        if name == DERIVED {
            self.record(DERIVED, line_of(item.ident.span()), Kind::Definition);
            // No derive (or any attribute but its documentation): a derived
            // `Default` or `Clone` would construct one without a literal.
            for path in applied_paths(&item.attrs) {
                if path != "doc" {
                    self.finding(line_of(item.ident.span()), format!("the {DERIVED} definition carries #[{path}]"));
                }
            }
        } else if self.scanned_bare(&name) {
            self.record(&name, line_of(item.ident.span()), Kind::Other("a struct of that name"));
        }
        visit::visit_generics(self, &item.generics);
        self.visit_fields(&item.fields);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if item.unsafety.is_some() {
            self.unsafe_found(line_of(item.impl_token.span), "an unsafe impl");
        }
        let line = line_of(item.impl_token.span);
        let derived = type_names_derived(&item.self_ty);
        let impl_self = match &*item.self_ty {
            _ if derived => ImplSelf::Derived,
            syn::Type::Path(path) if path.qself.is_none() && path.path.leading_colon.is_none() && path.path.segments.len() == 1 => {
                ImplSelf::Named(ident_text(&path.path.segments[0].ident))
            }
            _ => ImplSelf::Other,
        };
        if derived {
            // Codex round 2 at #1682 (P2): a trait impl for the type, other
            // than the listed ones, and any associated type in an impl for it,
            // could give it another name a literal resolves through.
            if let Some((_, trait_path, _)) = &item.trait_ {
                let name = path_text(trait_path);
                if !DERIVED_TRAIT_IMPLS.contains(&name.as_str()) {
                    self.finding(line, format!("an impl of the trait {name} for {DERIVED}, which is not on the allow-list"));
                }
            }
            for member in &item.items {
                if let syn::ImplItem::Type(assoc) = member {
                    if !is_test_only(&assoc.attrs) {
                        let assigned = if matches!(&assoc.ty, syn::Type::Path(path) if path.qself.is_none() && path.path.is_ident("Self")) { "Self" } else { "a type" };
                        self.finding(line_of(assoc.ident.span()), format!("the associated type {} assigned {assigned} in an impl for {DERIVED}", ident_text(&assoc.ident)));
                    }
                }
            }
        }
        self.impl_self.push(impl_self);
        visit::visit_item_impl(self, item);
        self.impl_self.pop();
    }

    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        if item.unsafety.is_some() {
            self.unsafe_found(line_of(item.trait_token.span), "an unsafe trait");
        }
        // A trait's default methods have no impl self type.
        self.impl_self.push(ImplSelf::Other);
        visit::visit_item_trait(self, item);
        self.impl_self.pop();
    }

    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        self.unsafe_found(line_of(item.abi.extern_token.span), "an extern block");
        visit::visit_item_foreign_mod(self, item);
    }

    fn visit_expr_unsafe(&mut self, block: &'ast syn::ExprUnsafe) {
        self.unsafe_found(line_of(block.unsafe_token.span), "an unsafe block");
        visit::visit_expr_unsafe(self, block);
    }

    fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
        self.alias(&item.ident);
        let saved = std::mem::replace(&mut self.in_type_alias, true);
        visit::visit_item_type(self, item);
        self.in_type_alias = saved;
    }

    fn visit_impl_item_type(&mut self, item: &'ast syn::ImplItemType) {
        self.alias(&item.ident);
        let saved = std::mem::replace(&mut self.in_type_alias, true);
        visit::visit_impl_item_type(self, item);
        self.in_type_alias = saved;
    }

    fn visit_trait_item_type(&mut self, item: &'ast syn::TraitItemType) {
        self.alias(&item.ident);
        let saved = std::mem::replace(&mut self.in_type_alias, true);
        visit::visit_trait_item_type(self, item);
        self.in_type_alias = saved;
    }

    fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
        self.check_attributes(&item.attrs);
        if item.mac.path.is_ident("macro_rules") {
            self.check_macro_definition(item);
        } else {
            self.check_invocation(&item.mac);
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.check_invocation(mac);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        self.check_attributes(&item.attrs);
        self.check_use(item);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        for attr in &call.attrs {
            self.visit_attribute(attr);
        }
        let mut consumed = false;
        if let syn::Expr::Path(path) = &*call.func {
            let line = path.path.segments.last().map_or(0, |segment| line_of(segment.ident.span()));
            if let (Some(qself), Some(name)) = (&path.qself, self.scanned_in_path(&path.path)) {
                // `<T as Trait>::name(..)` names an associated item, never the
                // scanned free function: refused, not counted as its call.
                self.record(&name, line, Kind::Other("named through a qualified <T as Trait>:: path"));
                self.visit_type(&qself.ty);
                consumed = true;
            } else if let Some(name) = self.scanned_in_path(&path.path) {
                self.record(&name, line, Kind::Call);
                if ENTRY_RULES.iter().any(|(callee, _, _)| *callee == name) {
                    let (entry, text) = Self::entry_of(&call.args);
                    let owner = self.owner();
                    self.entries.push((name.clone(), owner, entry, text, line));
                }
                consumed = true;
            } else if path.path.segments.last().is_some_and(|segment| ident_text(&segment.ident) == "UserRelease") {
                self.constructions.push((self.owner(), self.file.clone(), line));
                consumed = true;
            }
            if consumed {
                // The path's own generic arguments are still read.
                for segment in &path.path.segments {
                    self.visit_path_arguments(&segment.arguments);
                }
            }
        }
        if !consumed {
            self.visit_expr(&call.func);
        }
        for argument in &call.args {
            self.visit_expr(argument);
        }
    }

    /// `ProducerEntry::UserRelease` as a value (a function pointer to the
    /// variant's constructor) constructs one when called: counted as a
    /// construction where it is named.
    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if path.path.segments.last().is_some_and(|segment| ident_text(&segment.ident) == "UserRelease") {
            let line = path.path.segments.last().map_or(0, |segment| line_of(segment.ident.span()));
            self.constructions.push((self.owner(), self.file.clone(), line));
        }
        visit::visit_expr_path(self, path);
    }

    fn visit_expr_struct(&mut self, literal: &'ast syn::ExprStruct) {
        let line = literal.path.segments.last().map_or(0, |segment| line_of(segment.ident.span()));
        let last = literal.path.segments.last().map(|segment| ident_text(&segment.ident)).unwrap_or_default();
        // `ProducerEntry::UserRelease { 0: derived }` constructs one, as its
        // call form does (Codex round 2 at #1682, P2).
        if last == "UserRelease" {
            self.constructions.push((self.owner(), self.file.clone(), line));
        }
        if in_defining_module(&self.file) {
            self.check_literal_path(literal.qself.as_ref(), &literal.path, line);
        }
        let names_derived = last == DERIVED;
        // `Self { .. }` in an `impl` of `DerivedUserRelease` is its literal too.
        let self_literal = literal.qself.is_none() && literal.path.segments.len() == 1 && last == "Self" && self.impl_self.last() == Some(&ImplSelf::Derived);
        if names_derived || self_literal {
            self.record(DERIVED, line, Kind::Literal);
            for field in &literal.fields {
                self.visit_expr(&field.expr);
            }
            if let Some(rest) = &literal.rest {
                self.visit_expr(rest);
            }
            return;
        }
        visit::visit_expr_struct(self, literal);
    }

    fn visit_type_path(&mut self, path: &'ast syn::TypePath) {
        if path.path.segments.last().is_some_and(|segment| ident_text(&segment.ident) == DERIVED) {
            let kind = if self.in_type_alias { Kind::Other("named in a type alias") } else { Kind::TypeUse };
            self.record(DERIVED, line_of(path.path.segments.last().unwrap().ident.span()), kind);
            for segment in &path.path.segments {
                self.visit_path_arguments(&segment.arguments);
            }
            return;
        }
        visit::visit_type_path(self, path);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        // In src-tauri the renderer pair is matched only as its qualified path.
        if self.crate_key() == "src-tauri" {
            if let Some(name) = self.scanned_in_path(path) {
                if RENDERER_PAIR.contains(&name.as_str()) {
                    let line = line_of(path.segments.last().unwrap().ident.span());
                    self.record(&name, line, Kind::Other("named in a path other than a direct call"));
                    return;
                }
            }
        }
        visit::visit_path(self, path);
    }

    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        let name = ident_text(ident);
        if self.scanned_bare(&name) {
            self.record(&name, line_of(ident.span()), Kind::Other("named other than as a definition or a direct call"));
        }
    }

    /// In the three reporting crates a const generic argument other than a
    /// literal (a block, `{ .. }`) and an associated-constant argument are
    /// findings (code review r3 at #1682), as the token rule finds them in
    /// any tokens.
    fn visit_generic_argument(&mut self, argument: &'ast syn::GenericArgument) {
        if self.is_core() {
            match argument {
                syn::GenericArgument::Const(expr) if !matches!(expr, syn::Expr::Lit(_)) => {
                    let at = match expr {
                        syn::Expr::Block(block) => line_of(block.block.brace_token.span.open()),
                        _ => 0,
                    };
                    self.finding(at, "a const generic argument other than a literal in a reporting crate");
                }
                syn::GenericArgument::AssocConst(constant) => {
                    self.finding(line_of(constant.ident.span()), "an associated-constant generic argument in a reporting crate");
                }
                _ => {}
            }
        }
        visit::visit_generic_argument(self, argument);
    }

    // Syntax `syn` does not model (`Verbatim`) is a finding wherever it
    // occurs (code review r3 at #1682), never skipped.
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        match expr {
            syn::Expr::Verbatim(tokens) => self.unparsed(tokens, "an expression"),
            _ => visit::visit_expr(self, expr),
        }
    }

    fn visit_type(&mut self, ty: &'ast syn::Type) {
        match ty {
            syn::Type::Verbatim(tokens) => self.unparsed(tokens, "a type"),
            _ => visit::visit_type(self, ty),
        }
    }

    fn visit_pat(&mut self, pat: &'ast syn::Pat) {
        match pat {
            syn::Pat::Verbatim(tokens) => self.unparsed(tokens, "a pattern"),
            _ => visit::visit_pat(self, pat),
        }
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        match item {
            syn::TraitItem::Verbatim(tokens) => self.unparsed(tokens, "a trait item"),
            _ => visit::visit_trait_item(self, item),
        }
    }

    fn visit_foreign_item(&mut self, item: &'ast syn::ForeignItem) {
        match item {
            syn::ForeignItem::Verbatim(tokens) => self.unparsed(tokens, "a foreign item"),
            _ => visit::visit_foreign_item(self, item),
        }
    }

    fn visit_type_param_bound(&mut self, bound: &'ast syn::TypeParamBound) {
        match bound {
            syn::TypeParamBound::Verbatim(tokens) => self.unparsed(tokens, "a bound"),
            _ => visit::visit_type_param_bound(self, bound),
        }
    }
}

/// A listed macro's tokens parsed by its input grammar: the expressions
/// before a pattern, the pattern (`matches!` only) and the expressions after
/// it (its guard). Every token is in one of them, or is the grammar's own
/// punctuation (`,`, `;`, `if`, and `json!`'s `{`, `[`, `:`).
struct MacroBody {
    exprs: Vec<syn::Expr>,
    pattern: Option<syn::Pat>,
    after: Vec<syn::Expr>,
}

/// The tokens of the listed macro `path` ([`SUBTREE_MACROS`]) parsed by its
/// input grammar, or `None` when they do not parse so (code review r3 at
/// #1682):
/// - `format!`, `format_args!`: comma-separated expressions (a named
///   argument `name = value` is an assignment expression);
/// - `vec!`: `expr; expr`, or comma-separated expressions;
/// - `matches!`: `expr, pattern (if expr)? ,?`, as the standard macro's
///   `$expression:expr, $pattern:pat $(if $guard:expr)? $(,)?`;
/// - `json!`: [`json_value`].
fn parse_listed_macro(path: &str, tokens: &TokenStream) -> Option<MacroBody> {
    use syn::parse::{ParseStream, Parser};
    let exprs = |exprs: Vec<syn::Expr>| MacroBody { exprs, pattern: None, after: Vec::new() };
    let list = |tokens: &TokenStream| syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated.parse2(tokens.clone()).ok().map(|list| list.into_iter().collect::<Vec<_>>());
    match path {
        "format" | "format_args" => list(tokens).map(exprs),
        "vec" => {
            let repeat = |input: ParseStream<'_>| -> syn::Result<Vec<syn::Expr>> {
                let element: syn::Expr = input.parse()?;
                input.parse::<syn::Token![;]>()?;
                let count: syn::Expr = input.parse()?;
                Ok(vec![element, count])
            };
            repeat.parse2(tokens.clone()).ok().or_else(|| list(tokens)).map(exprs)
        }
        "matches" => {
            let parser = |input: ParseStream<'_>| -> syn::Result<MacroBody> {
                let scrutinee: syn::Expr = input.parse()?;
                input.parse::<syn::Token![,]>()?;
                let pattern = syn::Pat::parse_multi_with_leading_vert(input)?;
                let mut after = Vec::new();
                if input.peek(syn::Token![if]) {
                    input.parse::<syn::Token![if]>()?;
                    after.push(input.parse::<syn::Expr>()?);
                }
                if !input.is_empty() {
                    input.parse::<syn::Token![,]>()?;
                }
                Ok(MacroBody { exprs: vec![scrutinee], pattern: Some(pattern), after })
            };
            parser.parse2(tokens.clone()).ok()
        }
        "json" => {
            let mut out = Vec::new();
            json_value(tokens.clone().into_iter().collect(), &mut out).map(|()| exprs(out))
        }
        _ => None,
    }
}

/// One `json!` value: a lone `{ .. }` is an object of `key: value` entries
/// separated by commas, the key an expression and the value a `json!` value;
/// a lone `[ .. ]` is an array of comma-separated `json!` values; anything
/// else is one expression. Each key and expression goes to `out`; `None`
/// when any part does not parse so.
fn json_value(tokens: Vec<TokenTree>, out: &mut Vec<syn::Expr>) -> Option<()> {
    if let [TokenTree::Group(group)] = tokens.as_slice() {
        match group.delimiter() {
            proc_macro2::Delimiter::Brace => {
                for entry in json_entries(group.stream())? {
                    let colon = entry_colon(&entry)?;
                    out.push(syn::parse2(entry[..colon].iter().cloned().collect()).ok()?);
                    json_value(entry[colon + 1..].to_vec(), out)?;
                }
                return Some(());
            }
            proc_macro2::Delimiter::Bracket => {
                for entry in json_entries(group.stream())? {
                    json_value(entry, out)?;
                }
                return Some(());
            }
            _ => {}
        }
    }
    out.push(syn::parse2(tokens.into_iter().collect()).ok()?);
    Some(())
}

/// A `json!` object's or array's entries, split at its top-level commas; one
/// trailing comma is admitted, and any other empty entry is `None`.
fn json_entries(tokens: TokenStream) -> Option<Vec<Vec<TokenTree>>> {
    let mut entries = vec![Vec::new()];
    for tree in tokens {
        match &tree {
            TokenTree::Punct(comma) if comma.as_char() == ',' => entries.push(Vec::new()),
            _ => entries.last_mut()?.push(tree),
        }
    }
    if entries.last().is_some_and(Vec::is_empty) {
        entries.pop();
    }
    (!entries.iter().any(Vec::is_empty)).then_some(entries)
}

/// The index of a `json!` object entry's key-value colon: its first
/// top-level `:` that is not half of a path's `::`.
fn entry_colon(entry: &[TokenTree]) -> Option<usize> {
    let mut index = 0;
    while index < entry.len() {
        if let TokenTree::Punct(colon) = &entry[index] {
            if colon.as_char() == ':' {
                let path = colon.spacing() == proc_macro2::Spacing::Joint && matches!(entry.get(index + 1), Some(TokenTree::Punct(next)) if next.as_char() == ':');
                if !path {
                    return Some(index);
                }
                index += 1;
            }
        }
        index += 1;
    }
    None
}

/// The token rules of [`FileScan::token_rules`] over one token sequence and
/// each group in it: (line, finding).
///
/// A brace group directly after `<`, or after `,` or `=` while an angle
/// bracket is open in the sequence, is a block in generic arguments
/// (`Type<{ .. }>`, `path::<A, { .. }>`, a const parameter's `= { .. }`
/// default). The angle count is lexical and conservative: `<` opens unless it
/// is half of `<<` or `<=`; `>` closes unless it ends `->`, `=>` or starts
/// `>=`; `;` and every brace group reset it. A comparison followed by a block
/// argument (`f(a < b, { .. })`) is therefore found too, which fails closed.
fn lexical_findings(tokens: TokenStream, out: &mut Vec<(usize, String)>) {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    let mut open = 0usize;
    for (index, tree) in trees.iter().enumerate() {
        let previous = index.checked_sub(1).map(|at| &trees[at]);
        let previous_char = |chars: &[char]| matches!(previous, Some(TokenTree::Punct(punct)) if chars.contains(&punct.as_char()));
        match tree {
            TokenTree::Ident(ident) => {
                let text = ident.to_string();
                if let Some(raw) = text.strip_prefix("r#") {
                    if !RAW_IDENTIFIERS.contains(&raw) {
                        out.push((line_of(ident.span()), format!("the raw identifier {text} in a reporting crate, which is not on the raw-identifier allow-list")));
                    }
                }
            }
            TokenTree::Punct(punct) => {
                let joint_with = |chars: &[char]| punct.spacing() == proc_macro2::Spacing::Joint && matches!(trees.get(index + 1), Some(TokenTree::Punct(next)) if chars.contains(&next.as_char()));
                let after_joint = |chars: &[char]| matches!(previous, Some(TokenTree::Punct(first)) if chars.contains(&first.as_char()) && first.spacing() == proc_macro2::Spacing::Joint);
                match punct.as_char() {
                    '<' if !joint_with(&['<', '=']) && !after_joint(&['<']) => open += 1,
                    '>' if !after_joint(&['-', '=']) && !joint_with(&['=']) => open = open.saturating_sub(1),
                    ';' => open = 0,
                    _ => {}
                }
            }
            TokenTree::Group(group) => {
                if group.delimiter() == proc_macro2::Delimiter::Brace {
                    if previous_char(&['<']) || (open > 0 && previous_char(&[',', '='])) {
                        out.push((line_of(group.span()), "a brace or block expression in generic arguments in a reporting crate".to_string()));
                    }
                    open = 0;
                }
                lexical_findings(group.stream(), out);
            }
            TokenTree::Literal(_) => {}
        }
    }
}

/// The module file that `name`, declared in `declaring` under the inline
/// modules `inline`, loads: `name.rs` or `name/mod.rs` in the directory of a
/// root or `mod.rs` file, else in the directory named after the declaring
/// file, under the inline modules as further directories.
fn module_file(files: &BTreeMap<String, String>, declaring: &str, inline: &[String], name: &str) -> Result<String, String> {
    let (directory, stem) = declaring.rsplit_once('/').unwrap_or(("", declaring));
    let stem = stem.trim_end_matches(".rs");
    let crate_root = declaring.split('/').take(2).collect::<Vec<_>>().join("/");
    let is_root = (directory == crate_root && (stem == "lib" || stem == "main")) || stem == "mod";
    let mut base = if is_root { directory.to_string() } else { format!("{directory}/{stem}") };
    for module in inline {
        base.push('/');
        base.push_str(module);
    }
    let candidates = [format!("{base}/{name}.rs"), format!("{base}/{name}/mod.rs")];
    match (files.contains_key(&candidates[0]), files.contains_key(&candidates[1])) {
        (true, false) => Ok(candidates[0].clone()),
        (false, true) => Ok(candidates[1].clone()),
        (true, true) => Err(format!("{declaring}: the module {name} matches both {} and {}", candidates[0], candidates[1])),
        (false, false) => Err(format!("{declaring}: the module {name} has no file ({} or {})", candidates[0], candidates[1])),
    }
}

/// The scan over `sources`: every finding, as text. Empty when the boundary
/// holds.
pub(crate) fn scan(sources: &Sources) -> Vec<String> {
    scan_reading(sources).0
}

/// [`scan`], with the production files it read.
pub(crate) fn scan_reading(sources: &Sources) -> (Vec<String>, BTreeSet<String>) {
    let mut findings = Vec::new();
    // Manifests: the library at src/lib.rs, no binary target, no src/bin.
    for (key, _) in CRATES {
        let manifest = sources.manifests.get(key).cloned().unwrap_or_default();
        if manifest.contains("[[bin]]") {
            findings.push(format!("{key}/Cargo.toml declares a [[bin]] target"));
        }
        if let Some(lib) = manifest.split("[lib]").nth(1) {
            let section = lib.split("\n[").next().unwrap_or("");
            if let Some(line) = section.lines().find(|line| line.trim_start().starts_with("path")) {
                if !line.contains("\"src/lib.rs\"") {
                    findings.push(format!("{key}/Cargo.toml's [lib] names another path"));
                }
            }
        }
        if sources.bin_dirs.contains(key) {
            findings.push(format!("{key} has a src/bin directory"));
        }
    }
    let mut occurrences = Vec::new();
    let mut entries = Vec::new();
    let mut constructions = Vec::new();
    let mut macros = Vec::new();
    let mut type_names = TypeNames::default();
    let mut plain_literals = Vec::new();
    let mut queue: Vec<String> = CRATES.iter().map(|(key, _)| format!("{key}/src/lib.rs")).collect();
    queue.push("src-tauri/src/main.rs".to_string());
    let mut read: BTreeSet<String> = BTreeSet::new();
    while let Some(file) = queue.pop() {
        if !read.insert(file.clone()) {
            continue;
        }
        let Some(text) = sources.files.get(&file) else {
            findings.push(format!("{file}: missing"));
            continue;
        };
        let parsed = match syn::parse_file(text) {
            Ok(parsed) => parsed,
            Err(error) => {
                findings.push(format!("{file}: does not parse ({error})"));
                continue;
            }
        };
        let mut modules = Vec::new();
        let mut scan = FileScan {
            file: file.clone(),
            text,
            findings: &mut findings,
            occurrences: &mut occurrences,
            modules: &mut modules,
            owners: vec![Owner::Module],
            inline: Vec::new(),
            entries: &mut entries,
            constructions: &mut constructions,
            macros: &mut macros,
            impl_self: Vec::new(),
            type_names: &mut type_names,
            plain_literals: &mut plain_literals,
            in_type_alias: false,
            in_macro: false,
        };
        if scan.is_core() {
            scan.token_rules();
        }
        for attr in &parsed.attrs {
            scan.visit_attribute(attr);
        }
        for item in &parsed.items {
            scan.visit_item(item);
        }
        for (declaring, inline, name, line) in modules {
            match module_file(&sources.files, &declaring, &inline, &name) {
                Ok(next) => queue.push(next),
                Err(error) => findings.push(format!("{error} (line {line})")),
            }
        }
    }

    // A plain struct literal in the defining module names a struct the crate
    // defines, and no alias shares the name.
    for (file, line, name, how) in &plain_literals {
        if !type_names.structs.contains(name) || type_names.aliases.contains(name) {
            findings.push(format!("{file}:{line}: {how} of {name} in the defining module of {DERIVED}, which is not a struct the crate defines under a name no alias shares"));
        }
    }
    // The rule over every occurrence of a scanned name.
    let mut calls: BTreeMap<(&str, String, String), usize> = BTreeMap::new();
    let mut definitions: BTreeMap<String, usize> = BTreeMap::new();
    for occurrence in &occurrences {
        let Occurrence { name, file, line, kind, owner } = occurrence;
        let at = format!("{file}:{line}");
        if name == DERIVED {
            match kind {
                Kind::Definition if file == DERIVED_FILE => *definitions.entry(name.clone()).or_default() += 1,
                Kind::TypeUse => {}
                Kind::Literal if file == DERIVED_FILE && *owner == Owner::Fn("check_derived".to_string()) => {}
                Kind::Literal => findings.push(format!("{at}: a {DERIVED} literal outside check_derived")),
                other => findings.push(format!("{at}: {DERIVED} {}", describe(other))),
            }
            continue;
        }
        let Some((_, home, callers)) = ROWS.iter().find(|(function, _, _)| function == name) else { continue };
        match kind {
            Kind::Definition if file == home => *definitions.entry(name.clone()).or_default() += 1,
            Kind::Definition => findings.push(format!("{at}: {name} is defined outside {home}")),
            Kind::Call => match owner {
                Owner::Fn(caller) if callers.iter().any(|(caller_file, allowed)| caller_file == file && allowed == caller) => {
                    *calls.entry((name.as_str(), file.clone(), caller.clone())).or_default() += 1;
                }
                other => findings.push(format!("{at}: {name} is called from {other:?}, which is not an allowed caller")),
            },
            other => findings.push(format!("{at}: {name} {}", describe(other))),
        }
    }
    for (function, home, callers) in ROWS {
        if definitions.get(function).copied().unwrap_or(0) != 1 {
            findings.push(format!("{home}: {function} is not defined there exactly once"));
        }
        for (file, caller) in callers {
            let count = calls.get(&(function, file.to_string(), caller.to_string())).copied().unwrap_or(0);
            if count != 1 {
                findings.push(format!("{file}: {caller} calls {function} {count} times, not exactly once"));
            }
        }
    }
    if definitions.get(DERIVED).copied().unwrap_or(0) != 1 {
        findings.push(format!("{DERIVED_FILE}: {DERIVED} is not defined there exactly once"));
    }
    if !occurrences.iter().any(|occurrence| occurrence.name == DERIVED && occurrence.kind == Kind::Literal) {
        findings.push(format!("{DERIVED_FILE}: check_derived constructs no {DERIVED}"));
    }
    // Each pinned macro is defined exactly once.
    for (file, name, _) in MACRO_DEFINITIONS {
        let count = macros.iter().filter(|(seen_file, seen)| seen_file == file && seen == name).count();
        if count > 1 {
            findings.push(format!("{file}: the pinned macro {name} is defined {count} times"));
        }
    }
    // The entry argument.
    for (callee, owner, entry, text, line) in &entries {
        let caller = match owner {
            Owner::Fn(caller) => caller.as_str(),
            _ => "",
        };
        let expected = ENTRY_RULES.iter().find(|(rule_callee, rule_caller, _)| rule_callee == callee && *rule_caller == caller).map(|(_, _, entry)| *entry);
        if expected.is_none() || expected != *entry {
            findings.push(format!("line {line}: {caller} passes {text} to {callee} (expected {expected:?})"));
        }
    }
    // UserRelease is constructed only in assemble_derived.
    for (owner, file, line) in &constructions {
        if !(file == "report_package/src/user_release.rs" && *owner == Owner::Fn("assemble_derived".to_string())) {
            findings.push(format!("{file}:{line}: ProducerEntry::UserRelease is constructed outside assemble_derived ({owner:?})"));
        }
    }
    (findings, read)
}

fn describe(kind: &Kind) -> String {
    match kind {
        Kind::Other(what) => what.to_string(),
        Kind::Definition => "defined again".to_string(),
        Kind::Call => "called".to_string(),
        Kind::TypeUse => "used as a type".to_string(),
        Kind::Literal => "constructed".to_string(),
    }
}

/// The real sources of the four crates, read from the project tree.
pub(crate) fn project_sources(project: &Path) -> Sources {
    fn read_dir(dir: &Path, prefix: &str, out: &mut BTreeMap<String, String>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                read_dir(&path, &format!("{prefix}/{name}"), out);
            } else if name.ends_with(".rs") {
                out.insert(format!("{prefix}/{name}"), std::fs::read_to_string(&path).unwrap());
            }
        }
    }
    let mut sources = Sources::default();
    for (key, root) in CRATES {
        let crate_dir = project.join(root);
        read_dir(&crate_dir.join("src"), &format!("{key}/src"), &mut sources.files);
        sources.manifests.insert(key.to_string(), std::fs::read_to_string(crate_dir.join("Cargo.toml")).unwrap());
        if crate_dir.join("src/bin").is_dir() {
            sources.bin_dirs.insert(key.to_string());
        }
    }
    sources
}
