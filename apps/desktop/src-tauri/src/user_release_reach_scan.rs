//! `user_release_reach_scan` (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.8, §6): who can reach the renderer's
//! user-released class rule. The scan parses the production source of five
//! crates (`apps/desktop/src-tauri`, `core/reporting/report_package`,
//! `core/reporting/pdf_emitter`, `core/reporting/report_renderer` and, since
//! code review r9 at #1682, `core/runner/headless`) with `syn` and attributes
//! every scanned name to its innermost lexical scope owner.
//!
//! - **Allowed dependents** (code review r9 at #1682). Every allowed
//!   dependent of a reporting crate that is not one itself
//!   ([`DEPENDENT_CRATES`]: the desktop shell and the headless runner) is
//!   scanned under the shell's rules, from the library and binary roots
//!   Cargo reports, its targets, build-dependencies and normal and
//!   development dependencies pinned ([`TARGETS`], [`BUILD_DEPENDENCIES`],
//!   [`DEPENDENCY_PINS`]). No runner function may name a scanned function,
//!   the runner's own list additions are named with reasons
//!   ([`RUNNER_MACRO_INVOCATIONS`], [`RUNNER_SERDE_FLAGS`],
//!   [`RUNNER_SERDE_FUNCTIONS`], its two pinned [`MACRO_DEFINITIONS`]), and
//!   each runner root carries `#![forbid(unsafe_code)]`
//!   ([`UNSAFE_FORBIDDING_DEPENDENTS`]). Every `.rs` file in a scanned
//!   crate's directory is available to the module walk, and a production
//!   `#[path]`, still a finding, is followed and its file read as
//!   production; a `#[path]` that leaves the declaring crate's directory is
//!   a finding unless listed ([`PATH_CROSSINGS`]). No package in the
//!   allowed dependents' path-dependency closure, which holds every
//!   reporting crate's, builds a proc-macro ([`proc_macro_findings`],
//!   [`PROC_MACRO_PACKAGES`], empty).
//! - **Symbol-level FFI** (code review r9 follow-up at #1682). `src-tauri`'s
//!   roots carry `#![deny(unsafe_code)]`, and only the pinned `mod` items
//!   carry `#[allow(unsafe_code)]` ([`SHELL_UNSAFE_MODULES`]); every other
//!   lint attribute naming `unsafe_code` is a finding. In every scanned
//!   crate a foreign block is a finding unless pinned ([`FOREIGN_BLOCKS`]),
//!   and every linkage attribute ([`LINKAGE_ATTRIBUTES`]), dynamic-lookup
//!   name ([`DYNAMIC_LOOKUP`]) and transmute ([`TRANSMUTES`]) is a finding.
//!
//! - **Targets, from Cargo** (code review r5 at #1682). `cargo metadata
//!   --offline --locked --no-deps` reports each crate's actual targets; they
//!   must be exactly the allow-listed set ([`TARGETS`]): each library at its
//!   expected root, `src-tauri`'s app binary and build script, and today's
//!   integration tests by name and file. Any other binary, example, bench,
//!   build script, proc-macro or test target, auto-discovered or declared,
//!   and any other library root, is a finding; the build-dependencies are
//!   pinned ([`BUILD_DEPENDENCIES`]). Since code review r7, `src-tauri`'s
//!   build script is pinned by content ([`BUILD_SCRIPTS`]), so a
//!   `cargo:rustc-cfg=test` or any other change is a finding, and its normal
//!   and development dependencies are pinned ([`SHELL_DEPENDENCIES`]).
//! - **Production source.** From each library and binary target's actual
//!   root file (`src/lib.rs`, and `src-tauri`'s `src/main.rs`, when the
//!   targets are as allow-listed) it follows every `mod` item, outlined and
//!   inline, and resolves the files itself. An item is test-only when one of its outer
//!   attributes is exactly `#[cfg(test)]`; every other item, whatever its
//!   `cfg`, is production. In the three reporting crates test-only items and
//!   test modules are read under the construction, literal and naming rules,
//!   the one allow-listed proof-writing test excepted ([`TEST_PROOF_ITEMS`];
//!   code review r6 at #1682). In `src-tauri` they are read too (code review
//!   r7 at #1682): a build that sets `cfg(test)` would compile them into the
//!   app, so every occurrence of a scanned name or the renderer's pair there
//!   is a finding ([`SHELL_TEST_CALLERS`], empty), as are a glob or crate
//!   binding of the renderer or PDF crate, an import of the pair, and a macro
//!   definition not pinned ([`SHELL_TEST_MACROS`]); a test module's
//!   `#[path]` is followed ([`path_module_key`]) and the file read as the
//!   shell's test code. A `#[path]` on a production `mod`, a `cfg_attr`
//!   applying `path` at any depth and under any predicate (refused, not
//!   followed: the scan does not evaluate predicates), any `include!`, an
//!   `include_str!` that is not one string literal or names a `.rs` file,
//!   any `include_bytes!`, a missing or ambiguous module file, and a file
//!   that does not parse are findings.
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
//! - **`unsafe` and macro names** (code review r4 at #1682). Each reporting
//!   crate's library root carries `#![forbid(unsafe_code)]` exactly among
//!   its own file attributes, so the compiler refuses `unsafe` there, in
//!   macro arguments and included files too; its absence or any other form,
//!   and a `forbid` elsewhere (a nested module's inner one included), are
//!   findings, and
//!   the token rule also finds every `unsafe` keyword there. So that no
//!   listed macro path resolves to another macro: in the reporting crates
//!   every `use … as …` rename and every non-private `use` is on a closed
//!   list of today's ([`USE_RENAMES`], [`PUBLIC_USES`]); in every scanned crate a
//!   `mod`, `use` binding, `extern crate` or `macro_rules!` named like a
//!   macro path root or a listed macro ([`reserved_name`]) is a finding, as is
//!   a `mod`, `use`, `extern`, `macro` or `macro_rules!` in tokens the scan
//!   does not parse; and `include!`, `include_bytes!` and `concat_idents!`
//!   are refused by any path, invoked or imported, with `include_str!`
//!   admitted only bare.
//!
//! - **Build configuration** (code review r6 at #1682, [`build_configuration`]).
//!   The desktop shell's resolved graph (`cargo metadata --offline --locked
//!   --filter-platform host-tuple`) holds each reporting package exactly
//!   once, from the directory the scan read, and only crates.io and
//!   repository paths; the headless runner declares `report_package` at that
//!   directory; no `.cargo/config(.toml)` is in the repository tree
//!   ([`CARGO_CONFIGS`], empty); and no checked manifest names `patch` or
//!   `replace`. Since code review r7, no in-repository package of the
//!   consumers' graphs but `src-tauri` runs a build script and depends on a
//!   reporting crate ([`in_repo_build_scripts`]). Since code review r8, the
//!   packages that depend on each reporting package are exactly the allowed
//!   ones ([`REPORTING_DEPENDENTS`]), by name and canonical manifest, as
//!   Cargo reports the dependencies of every package in the project and of
//!   every path dependency reachable from them ([`in_repo_packages`]), and
//!   in the shell's resolved graph; no manifest text is matched for this.
//!
//! What it does not establish: code that builds a document without calling
//! the scanned functions, and a listed external macro whose version would
//! emit a call without naming it (§2.8, "What the scan does not
//! establish"). Standard claim fence applies (claims taxonomy per DEC-081).

use proc_macro2::{Span, TokenStream, TokenTree};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};

/// The scanned crates: (key, source root). The three reporting crates, and
/// the allowed dependents of a reporting crate that are not reporting crates
/// themselves ([`DEPENDENT_CRATES`]): the desktop shell and, since code
/// review r9 at #1682, the headless runner.
pub(crate) const CRATES: [(&str, &str); 5] = [
    ("src-tauri", "apps/desktop/src-tauri"),
    ("report_package", "core/reporting/report_package"),
    ("pdf_emitter", "core/reporting/pdf_emitter"),
    ("report_renderer", "core/reporting/report_renderer"),
    ("headless", "core/runner/headless"),
];

/// The reporting crates' keys in [`CRATES`].
const REPORTING_KEYS: [&str; 3] = ["report_package", "pdf_emitter", "report_renderer"];

/// The crate keys of the allowed dependents that are not reporting crates
/// ([`REPORTING_DEPENDENTS`], [`DEPENDENT_MANIFESTS`]), each scanned under the
/// shell's rules (code review r9 at #1682; §2.8, "Allowed dependents"): its
/// Cargo targets, its module closure from each library and binary root, the
/// name, construction, literal, macro, attribute, namespace and test-item
/// rules, and its target, build-script and dependency pins.
pub(crate) const DEPENDENT_CRATES: [&str; 2] = ["src-tauri", "headless"];

/// The headless runner's normal, development and build dependencies,
/// exactly, as Cargo reports them, in [`SHELL_DEPENDENCIES`]' form (code
/// review r9 at #1682): any change, a new path dependency (a local
/// proc-macro included, which can emit a call the scan never reads) or a
/// rename, is a finding until reviewed. Its build-dependencies are also
/// pinned (none, [`BUILD_DEPENDENCIES`]).
pub(crate) const RUNNER_DEPENDENCIES: [&str; 10] = [
    "normal open_pipe_stress_canonical_json",
    "normal open_pipe_stress_mechanics_benchmarks",
    "normal open_pipe_stress_nonlinear_benchmarks",
    "normal open_pipe_stress_product_physics",
    "normal open_pipe_stress_report_package",
    "normal open_pipe_stress_result_export",
    "normal open_pipe_stress_stress_benchmarks",
    "normal serde",
    "normal serde_json",
    "normal sha2",
];

/// The pinned normal and development dependency sets, by crate key: the
/// allowed dependents' ([`DEPENDENT_CRATES`]).
const DEPENDENCY_PINS: [(&str, &[&str]); 2] = [("src-tauri", &SHELL_DEPENDENCIES), ("headless", &RUNNER_DEPENDENCIES)];

/// The macro invocations the headless runner's production code may write
/// besides [`MACRO_INVOCATIONS`], by path as written, each with its reason
/// (code review r9 at #1682): exactly today's. Admitted only in the runner;
/// every other macro there, an external crate's included, is a finding.
const RUNNER_MACRO_INVOCATIONS: [(&str, &str); 3] = [
    ("base", "reader_binding's pinned local macro_rules! (MACRO_DEFINITIONS), which writes one Reader literal"),
    ("transport", "reader_binding's pinned local macro_rules! (MACRO_DEFINITIONS), which writes one Reader literal"),
    ("concat", "the standard macro, inside the pinned transport!, joining two literals into one string literal"),
];

/// The `serde` flags the headless runner may write besides [`SERDE_FLAGS`]
/// (code review r9 at #1682): `skip_serializing` names no function.
const RUNNER_SERDE_FLAGS: [&str; 1] = ["skip_serializing"];

/// The `serde` keys naming a function the headless runner may write, with the
/// one value each admits (code review r9 at #1682): the derive expands the
/// value into a call of that path, so the value is pinned, as
/// `skip_serializing_if`'s are ([`SKIP_SERIALIZING_IF`]).
const RUNNER_SERDE_FUNCTIONS: [(&str, &str); 1] = [("deserialize_with", "deserialize_present_json")];

/// The allowed dependents that must carry `#![forbid(unsafe_code)]` at every
/// library and binary root Cargo reports, as the reporting crates do at
/// their library root (code review r9 at #1682): the headless runner, which
/// holds no `unsafe` code. A call through a declared foreign symbol needs
/// `unsafe`, so the compiler refuses it there. The shell holds `unsafe` code
/// (`host_memory.rs`, `worker_alloc.rs`) and is not on this list.
const UNSAFE_FORBIDDING_DEPENDENTS: [&str; 1] = ["headless"];

/// The crates whose `src/bin` directory is admitted (code review r9 at
/// #1682): the headless runner declares its two binaries there. Cargo
/// discovers any other file there as a binary target, which the target pin
/// ([`TARGETS`]) refuses.
const BIN_DIRS: [&str; 1] = ["headless"];

/// The `#[path]` attributes admitted that load a file outside the declaring
/// crate's directory (code review r9 at #1682), as (declaring file, loaded
/// file, reason), each on a test-only module and read as the declaring
/// crate's test code. Any other `#[path]` that leaves its crate's directory,
/// in a reporting crate or an allowed dependent, is a finding.
const PATH_CROSSINGS: [(&str, &str, &str); 2] = [
    (
        "src-tauri/src/report_package_save.rs",
        "report_package/tests/common/package_basis.rs",
        "the save tests build their request from report_package's shared test basis, read as the shell's test code",
    ),
    (
        "src-tauri/src/report_render_exact_tests.rs",
        "report_package/src/forwarding_graph_scan.rs",
        "W-11's forwarding-graph scan, a test-only module, run over the shell's sources, read as the shell's test code",
    ),
];

/// The in-repository proc-macro packages admitted in the path-dependency
/// closure of the reporting crates and their allowed dependents (code review
/// r9 at #1682): none. A proc-macro's expansion is code the scan never reads,
/// so any package with a `proc-macro` target there is a finding.
pub(crate) const PROC_MACRO_PACKAGES: [&str; 0] = [];

/// The `src-tauri` modules admitted to hold `unsafe` code (code review r9
/// follow-up at #1682; §2.8, "Symbol-level FFI"), as (declaring file, `mod`
/// name, reason): each `mod` item carries exactly the outer attribute
/// `#[allow(unsafe_code)]`, under the roots' `#![deny(unsafe_code)]`. Any
/// other `allow(unsafe_code)` or `expect(unsafe_code)`, in any form (direct,
/// through `cfg_attr`, inner, or on any other item), is a finding, and so is
/// a listed `mod` without its allow. The list is a deliberate gate: a future
/// module needing `unsafe` is added to it explicitly, in its own reviewed PR.
/// `allow(warnings)` and the lint groups do
/// not lift a `deny(unsafe_code)` (no rustc group holds `unsafe_code`;
/// checked on Rust 1.97.1), so they need no rule.
pub(crate) const SHELL_UNSAFE_MODULES: [(&str, &str, &str); 12] = [
    ("src-tauri/src/embedded_agent/mod.rs", "keychain_native", "generic provider credential SecItem calls and SDK-public constants, no-UI exact-item queries"),
    // HELP_HUMAN's ruling on #1709's autosave (code review r9 follow-up at
    // #1682): pinned as the eleventh; its test module `tests`
    // (`autosave_tests.rs`) is declared inside it and inherits the allow.
    ("src-tauri/src/lib.rs", "autosave", "autosave's directory-descriptor system calls (openat, fstatat, unlinkat) and its tests' mkfifo"),
    ("src-tauri/src/lib.rs", "handoff_package", "the handoff package's file-descriptor and atomic-publish system calls"),
    ("src-tauri/src/lib.rs", "handoff_package_publish", "the handoff publish's directory-descriptor system calls"),
    ("src-tauri/src/lib.rs", "host_memory", "the host memory sysctls and mach_host_self"),
    ("src-tauri/src/lib.rs", "ingress", "#1676's ingress envelope: the page-size and resident-memory probes"),
    ("src-tauri/src/lib.rs", "stored_read", "SQLite's memory-status call"),
    ("src-tauri/src/lib.rs", "stored_model_document_export", "SQLite's memory-status call in its tests"),
    ("src-tauri/src/lib.rs", "solve_jobs", "the worker liveness probe (kill with signal 0)"),
    ("src-tauri/src/lib.rs", "solve_worker", "the solve worker's process, pipe and watchdog system calls"),
    ("src-tauri/src/lib.rs", "worker_alloc", "the worker's counting global allocator"),
    ("src-tauri/src/analysis_runs.rs", "tests", "analysis_runs_tests.rs: SQLite's memory-status call and an allocation probe"),
];

/// The foreign blocks admitted, as (file, ABI, item text with whitespace
/// collapsed) (code review r9 follow-up at #1682): the block carries no
/// attribute and no `unsafe`, and its items are exactly the pinned text, so
/// an added item or an attribute on one changes it. Any other foreign block,
/// any ABI, in any scanned crate, is a finding: a declared symbol can name a
/// mangled Rust function (`assemble_derived`'s included) under another
/// identifier.
pub(crate) const FOREIGN_BLOCKS: [(&str, &str, &str); 2] = [
    ("src-tauri/src/host_memory.rs", "C", "fn mach_host_self() -> libc::mach_port_t;"),
    ("src-tauri/src/embedded_agent/keychain_native.rs", "C", "static kSecUseAuthenticationUIFail: CFStringRef; static kSecAttrGeneric: CFStringRef; static kSecMatchLimitOne: CFStringRef;"),
];

/// The linkage attributes, each a finding wherever it is written, through
/// `cfg_attr` and in the `unsafe(...)` form included (code review r9
/// follow-up at #1682). The allow-list is empty; none exists today.
const LINKAGE_ATTRIBUTES: [&str; 6] = ["link_name", "link", "no_mangle", "export_name", "used", "link_section"];

/// The names of dynamic symbol lookup and of reinterpreting transmutes, each
/// a finding as any identifier or path segment, in code, imports and macro
/// tokens (code review r9 follow-up at #1682). The allow-lists are empty and
/// none occurs today. Every `transmute` and `transmute_copy` is refused, not
/// only those with a function-pointer or inferred target: a type alias can
/// hide a function-pointer target from a parse.
const DYNAMIC_LOOKUP: [&str; 5] = ["dlsym", "dlopen", "dlvsym", "NSAddressOfSymbol", "libloading"];
const TRANSMUTES: [&str; 2] = ["transmute", "transmute_copy"];

/// Whether `attr` is `#![deny(unsafe_code)]` exactly: the `src-tauri` roots'
/// own (code review r9 follow-up at #1682), as [`is_root_forbid`].
fn is_root_deny(attr: &syn::Attribute) -> bool {
    matches!(attr.style, syn::AttrStyle::Inner(_))
        && matches!(&attr.meta, syn::Meta::List(list)
            if list.path.leading_colon.is_none()
                && list.path.is_ident("deny")
                && matches!(list.delimiter, syn::MacroDelimiter::Paren(_))
                && list.tokens.to_string() == "unsafe_code")
}

/// Whether `attr` is the outer `#[allow(unsafe_code)]` exactly, as the pinned
/// modules carry it ([`SHELL_UNSAFE_MODULES`]).
fn is_module_allow(attr: &syn::Attribute) -> bool {
    matches!(attr.style, syn::AttrStyle::Outer)
        && matches!(&attr.meta, syn::Meta::List(list)
            if list.path.leading_colon.is_none()
                && list.path.is_ident("allow")
                && matches!(list.delimiter, syn::MacroDelimiter::Paren(_))
                && list.tokens.to_string() == "unsafe_code")
}

/// Whether `tokens` hold the identifier `unsafe_code`, at any depth.
fn names_unsafe_code(tokens: &TokenStream) -> bool {
    tokens.clone().into_iter().any(|tree| match tree {
        TokenTree::Ident(ident) => ident_text(&ident) == "unsafe_code",
        TokenTree::Group(group) => names_unsafe_code(&group.stream()),
        _ => false,
    })
}

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

/// The test-only items that construct the proof, by file and item path
/// ([`FileScan::test_key`]), each with its reason (code review r6 at #1682;
/// §2.8, "Test-only items"). Each writes exactly one `DerivedUserRelease`
/// literal and one `ProducerEntry::UserRelease` construction; any other
/// test-only item that writes either is a finding, and so is a listed item
/// that no longer writes exactly one of each.
pub(crate) const TEST_PROOF_ITEMS: [(&str, &str, &str); 1] = [(
    "report_package/src/user_release.rs",
    "tests::check_request_refuses_markers_and_the_entry_binds_its_form",
    // Reason: the unit test of `bindings::check_request_in` under the
    // user-release entry (§2.8 refusal 2 and item 8, the handoff rule) needs
    // that entry, and so a proof, for requests the test edits so that the
    // check refuses them; `check_derived` would refuse to derive a proof for
    // them, so the test writes one. It is compiled only into the crate's own
    // unit-test binary: a dependency build with `cfg(test)` fails to compile
    // (the crate root's `CARGO_PRIMARY_PACKAGE` guard).
    "check_request_in's own unit test needs the user-release entry for requests it edits to be refused, which check_derived would not derive",
)];

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
    ("assemble_wire_request_in", "assemble_derived", Entry::Released),
];

/// The allowed production macro definitions: (file, name, SHA-256 of its
/// text). `app_invoke_handler` was re-pinned after #1665 added two ordinary
/// handler entries, `check_preview_model` and `cancel_model_checks`, and
/// after #1717 added `acknowledge_close_request` and `resolve_close_request`,
/// and after #1718 (D5 slice 2, RT-U3) added the six `autosave_commands::`
/// entries `write_project_autosave`, `list_project_autosaves`,
/// `read_project_autosave`, `discard_project_autosave`, `get_autosave_settings`
/// and `set_autosave_settings` (each reviewed: no `mod`, no `$parameter ::`,
/// no scanned name added). The
/// D-6 review: save_report_package's command signature and handler tokens are
/// unchanged; the existing macro digest was reproduced from macro_rules through
/// the closing brace before changing route-010 admission. No new command,
/// module, parameter path or scanned name is added. The
/// headless runner's two (code review r9 at #1682) each write one `Reader`
/// literal of its own reader table (reviewed: no `mod`, no `$parameter ::`,
/// no scanned name).
const MACRO_DEFINITIONS: [(&str, &str, &str); 3] = [
    ("src-tauri/src/lib.rs", "app_invoke_handler", "d7e1ccc99b1e58b0189b065b71ef923a888c925149baca07648e677358235f77"),
    ("headless/src/reader_binding.rs", "base", "e4ca54306ec20645a40f7658e2d9f63508fd5ab234c9d29b83f95c7ac0bd5aea"),
    ("headless/src/reader_binding.rs", "transport", "b000ef8eab3fc5711dbb04be30705f8902c19adfee232aec9114210f6d48fd79"),
];

/// The macro definitions admitted in `src-tauri`'s test-only items (code
/// review r7 at #1682; §2.8, "Test-only items"): (file, name, SHA-256 of its
/// text, reason), pinned as [`MACRO_DEFINITIONS`] is. A test-only macro
/// could assemble a renderer path from fragments the token screen cannot
/// join (`open_pipe_stress_report_renderer::$f`), so any other definition in
/// a `src-tauri` test-only item, and any change to one of these, is a
/// finding until reviewed and re-pinned.
const SHELL_TEST_MACROS: [(&str, &str, &str, &str); 2] = [
    (
        "src-tauri/src/command_ingress.rs",
        "probe_scalars",
        "5b80746536a75677810c3199902df7ff3611cb6df38fbde2e00488b53aeb1946",
        // Reason: the ingress tests' serde probe declares one `Deserializer`
        // method per listed name (`fn $method`, `stringify!($method)`); it
        // names no path.
        "the ingress tests' serde probe declares a Deserializer method per name and names no path",
    ),
    (
        "src-tauri/src/command_ingress.rs",
        "probe_unsupported",
        "dc8a3e5894e98b0eb3ecf4dafaad96aade36b4cfc676631a3af7a0b642e16f1a",
        // Reason: as `probe_scalars`, for the methods the probe refuses.
        "the ingress tests' serde probe declares a Deserializer method per name and names no path",
    ),
];

/// The test-only items in `src-tauri` that may name a scanned function or
/// the renderer's pair (code review r7 at #1682; §2.8, "Test-only items"):
/// (file, item path, name, reason). None does today: the shell's tests
/// render through `render_calculation_report_value` and the save command,
/// and never name the renderer's pair, the PDF crate's functions, the
/// producer's internals or the bridge function. Any such occurrence in a
/// `src-tauri` test-only item (a call, a definition, a value, an import, a
/// macro's tokens) is a finding.
pub(crate) const SHELL_TEST_CALLERS: [(&str, &str, &str, &str); 0] = [];

/// The `include_str!` arguments naming a `.rs` file admitted in `src-tauri`'s
/// test-only items (code review r7 at #1682): (file, literal, reason).
/// `include_str!` yields a string literal and brings in no code; a `.rs`
/// argument is still refused elsewhere, as in production.
const SHELL_TEST_SOURCE_TEXTS: [(&str, &str, &str); 1] = [(
    "src-tauri/src/solve_attestation.rs",
    "solve_attestation.rs",
    // Reason: `the_table_is_never_persisted_or_serialized` reads its own
    // module's text, as a string, to assert what the production part names.
    "the attestation test reads its own module's text as a string to assert what the production part names",
)];

/// The build scripts admitted, pinned by content (code review r7 at #1682;
/// §2.8, "Build scripts"): (file, byte length, SHA-256). A build script's
/// `cargo:rustc-cfg=test` sets `cfg(test)` for its own package, so it could
/// compile `src-tauri`'s test-only items into the app; any change to the
/// script, and any build script of a scanned crate not on this list
/// ([`TARGETS`] admits only `src-tauri`'s), is a finding until reviewed and
/// re-pinned.
pub(crate) const BUILD_SCRIPTS: [(&str, usize, &str); 1] =
    [("src-tauri/build.rs", 2078, "b803655933381e84626d8277ee6816d14a2db58634a52e70b7404419f14318f8")];

/// `src-tauri`'s normal and development dependencies, exactly, as Cargo
/// reports them (code review r7 at #1682): `<kind> <package>`, with ` as
/// <name>` for a rename and ` (<platform>)` for a platform-specific one. The
/// reach scan's parser crates are development dependencies only: as normal
/// dependencies they would let a build that sets `cfg(test)` compile the
/// scan's test modules, so moving one, like any other change here, is a
/// finding until reviewed.
pub(crate) const SHELL_DEPENDENCIES: [&str; 37] = [
    "dev proc-macro2",
    "dev syn",
    "normal bytes (cfg(target_os = \"macos\"))",
    "normal core-foundation (cfg(target_os = \"macos\"))",
    "normal getrandom",
    "normal http (cfg(target_os = \"macos\"))",
    "normal http-body-util (cfg(target_os = \"macos\"))",
    "normal httparse (cfg(target_os = \"macos\"))",
    "normal hyper (cfg(target_os = \"macos\"))",
    "normal hyper-util (cfg(target_os = \"macos\"))",
    "normal libc (cfg(unix))",
    "normal open_pipe_stress_audit_manifest",
    "normal open_pipe_stress_canonical_json",
    "normal open_pipe_stress_library_import_document",
    "normal open_pipe_stress_operation_applier",
    "normal open_pipe_stress_product_physics",
    "normal open_pipe_stress_report_package",
    "normal open_pipe_stress_report_renderer",
    "normal open_pipe_stress_result_export",
    "normal open_pipe_stress_rule_check_runner",
    "normal open_pipe_stress_rule_pack_document",
    "normal open_pipe_stress_self_weight_wasm",
    "normal open_pipe_stress_units",
    "normal rusqlite",
    "normal rustls (cfg(target_os = \"macos\"))",
    "normal rustls-platform-verifier (cfg(target_os = \"macos\"))",
    "normal security-framework (cfg(target_os = \"macos\"))",
    "normal security-framework-sys (cfg(target_os = \"macos\"))",
    "normal serde",
    "normal serde_json",
    "normal sha2",
    "normal socket2 (cfg(target_os = \"macos\"))",
    "normal tauri",
    "normal tauri-plugin-dialog",
    "normal tokio (cfg(target_os = \"macos\"))",
    "normal tokio-rustls (cfg(target_os = \"macos\"))",
    "normal url (cfg(target_os = \"macos\"))",
];

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

/// The library roots of the three reporting crates, as Cargo must report
/// them ([`TARGETS`]). The root the scan reads is the lib target's actual
/// `src_path`, and its `syn::File` attributes must carry the inner attribute
/// `#![forbid(unsafe_code)]` exactly (code review r4 at #1682): the compiler
/// then refuses `unsafe` code in the crate, inside macro arguments and
/// included files too, and no inner `allow` can lift it. Its absence or any
/// other form is a finding, as is a `forbid` anywhere else, a nested
/// module's inner `#![forbid]` included (code review r5 at #1682).
const REPORTING_ROOTS: [(&str, &str); 3] =
    [("report_package", "report_package/src/lib.rs"), ("report_renderer", "report_renderer/src/lib.rs"), ("pdf_emitter", "pdf_emitter/src/lib.rs")];

/// The standard crate roots a macro path can start from, besides the listed
/// external crates ([`LISTED_ROOTS`]): no local item may take one of these
/// names, nor any root of a listed invocation ([`reserved_name`]).
const BUILTIN_ROOTS: [&str; 3] = ["std", "core", "alloc"];

/// The renames admitted in a `use` item of the three reporting crates, as
/// (path, name): exactly today's, each a crate alias (code review r4 at
/// #1682). Whether a renamed path is a macro is not decidable from source,
/// so every other `use … as …`, `as _` included, is a finding there.
const USE_RENAMES: [(&str, &str); 7] = [
    ("open_pipe_stress_audit_manifest", "audit"),
    ("open_pipe_stress_pdf_emitter", "pdf"),
    ("open_pipe_stress_report_renderer", "renderer"),
    ("open_pipe_stress_protected_content_linter", "linter"),
    ("open_pipe_stress_report_generator", "report"),
    ("open_pipe_stress_report_sections", "sections"),
    ("crate", "package"),
];

/// The non-private `use` paths admitted in the three reporting crates, as
/// (file, path): exactly today's re-exports, none a macro (code review r4 at
/// #1682). Every other `pub use`, `pub(crate) use` or other visibility there
/// is a finding.
const PUBLIC_USES: [(&str, &str); 5] = [
    ("report_package/src/lib.rs", "open_pipe_stress_report_renderer::serde_error_kind"),
    ("report_package/src/lib.rs", "reader::read_report_package_container"),
    ("report_package/src/lib.rs", "reader::PackageReadRefusal"),
    ("report_package/src/lib.rs", "reader::ReadReportPackage"),
    ("report_package/src/lib.rs", "reader::SUPPORTED_PACKAGE_VERSION"),
];

/// Macros that bring in source or build identifiers the scan cannot read,
/// refused by any path, as invocations and as imports (code review r4 at
/// #1682); `include_str!` is admitted only bare and with one string literal
/// that names no `.rs` file ([`FileScan::check_invocation_path`]).
const SOURCE_MACROS: [&str; 4] = ["include", "include_str", "include_bytes", "concat_idents"];

/// Whether `name` is reserved for macro paths (code review r4 at #1682): a
/// standard crate root, a listed external crate, or the first segment of a
/// listed invocation (a bare listed macro's own name included). A local
/// `mod`, `use` binding, `extern crate` or `macro_rules!` of that name could
/// make a listed path resolve to another macro (`mod tauri { pub use
/// std::include as generate_context; }`), so each is a finding. The headless
/// runner's listed invocations ([`RUNNER_MACRO_INVOCATIONS`]) are reserved
/// in every scanned crate too (code review r9 at #1682), but for the pinned
/// definitions themselves.
fn reserved_name(name: &str) -> bool {
    BUILTIN_ROOTS.contains(&name)
        || LISTED_ROOTS.contains(&name)
        || MACRO_INVOCATIONS.iter().chain(RUNNER_MACRO_INVOCATIONS.iter().map(|(path, _)| path)).any(|path| path.split("::").next() == Some(name))
}

/// Whether `attr` is `#![forbid(unsafe_code)]` exactly: inner, the bare path
/// `forbid`, parentheses, and the one lint `unsafe_code`. It is admitted only
/// among the `syn::File` attributes of a reporting crate's actual library
/// root ([`scan_reading`]); anywhere else, a nested `mod`'s inner attributes
/// included, it is not on the attribute list.
fn is_root_forbid(attr: &syn::Attribute) -> bool {
    matches!(attr.style, syn::AttrStyle::Inner(_))
        && matches!(&attr.meta, syn::Meta::List(list)
            if list.path.leading_colon.is_none()
                && list.path.is_ident("forbid")
                && matches!(list.delimiter, syn::MacroDelimiter::Paren(_))
                && list.tokens.to_string() == "unsafe_code")
}

/// The attribute paths admitted (§2.8 macro rule 3).
const ATTRIBUTES: [&str; 11] = ["cfg", "cfg_attr", "allow", "inline", "cold", "global_allocator", "derive", "doc", "default", "serde", "tauri::command"];
/// The attributes a test-only item may carry besides [`ATTRIBUTES`] (code
/// review r6 at #1682): the standard test harness's own.
const TEST_ATTRIBUTES: [&str; 3] = ["test", "should_panic", "ignore"];
/// The attributes a `src-tauri` test-only item may carry besides those (code
/// review r7 at #1682): `#[track_caller]`, on today's test helpers, which
/// only moves a panic's reported location.
const SHELL_TEST_ATTRIBUTES: [&str; 1] = ["track_caller"];
/// The derives a test-only item may write besides [`DERIVES`]: the same two
/// `serde` derives by their crate path (code review r6 at #1682).
const TEST_DERIVES: [&str; 2] = ["serde::Serialize", "serde::Deserialize"];
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
    /// `&ProducerEntry::UserRelease(derived)`. Not named `UserRelease`: the
    /// scan reads its own source as `src-tauri` test code (code review r7 at
    /// #1682), where that name is a construction.
    Released,
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
    /// In a test-only item ([`is_test_only`]): the item's path in its file
    /// ([`FileScan::test_key`]).
    test: Option<String>,
}

/// The sources the scan reads: crate files by `<crate key>/...` path, each
/// crate's Cargo targets and build-dependency names as Cargo itself reports
/// them ([`cargo_crate`]), whether it has a `src/bin` directory, and the
/// canonical directory each crate was read from (code review r6 at #1682:
/// the resolved reporting packages must be these, [`build_configuration`]).
/// Since code review r7, also each build script's text by its target file
/// ([`BUILD_SCRIPTS`]) and each crate's normal and development dependencies
/// ([`SHELL_DEPENDENCIES`]).
#[derive(Clone, Default)]
pub(crate) struct Sources {
    pub(crate) files: BTreeMap<String, String>,
    pub(crate) targets: BTreeMap<String, Vec<CrateTarget>>,
    pub(crate) build_dependencies: BTreeMap<String, BTreeSet<String>>,
    pub(crate) dependencies: BTreeMap<String, BTreeSet<String>>,
    pub(crate) build_scripts: BTreeMap<String, String>,
    pub(crate) bin_dirs: BTreeSet<String>,
    pub(crate) crate_dirs: BTreeMap<String, PathBuf>,
}

/// One Cargo target of a scanned crate, from `cargo metadata`: its kinds
/// joined by `,` (`lib`, `bin`, `test`, `bench`, `example`, `custom-build`,
/// `proc-macro`, or a lib's crate types), its name, and its source file as
/// `<crate key>/<path in the crate>` (the absolute path when it lies outside
/// the crate directory).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CrateTarget {
    pub(crate) kind: String,
    pub(crate) name: String,
    pub(crate) src_path: String,
}

/// The target kinds whose source is a crate root the scan reads: a library
/// (any crate type) or a binary. Integration tests, benches, examples and
/// build scripts are not read; only the allow-listed ones may exist
/// ([`TARGETS`]).
fn is_scanned_target(kind: &str) -> bool {
    kind.split(',').all(|kind| !matches!(kind, "test" | "bench" | "example" | "custom-build"))
}

/// Each scanned crate's Cargo targets, exactly, as (kinds, name, source
/// file) (code review r5 at #1682). Cargo, not the manifest's spelling,
/// decides what a crate builds: `cargo metadata` reports the targets Cargo
/// would build, auto-discovered ones (`src/main.rs`, `src/bin/`, `examples/`,
/// `benches/`, `tests/`, `build.rs`) and an `autobins` or other `auto*` key
/// included, and the library's actual root file. Each reporting crate builds
/// its library from `src/lib.rs` and today's integration tests, nothing
/// else; `src-tauri` also builds the app binary from `src/main.rs` (read as a
/// crate root) and runs its `build.rs` (`tauri_build::build()`, whose only
/// build-dependency is `tauri-build`, [`BUILD_DEPENDENCIES`]). The headless
/// runner (code review r9 at #1682) builds its library, its two declared
/// binaries (each read as a crate root) and today's integration tests, and
/// runs no build script. Any other
/// target (a binary, an example, a bench, a build script, a proc-macro or
/// another integration test) and any other root file is a finding.
pub(crate) const TARGETS: [(&str, &[(&str, &str, &str)]); 5] = [
    (
        "src-tauri",
        &[
            ("staticlib,cdylib,rlib", "openpipestress_desktop_lib", "src-tauri/src/lib.rs"),
            ("bin", "openpipestress-desktop", "src-tauri/src/main.rs"),
            ("custom-build", "build-script-build", "src-tauri/build.rs"),
        ],
    ),
    (
        "report_package",
        &[
            ("lib", "open_pipe_stress_report_package", "report_package/src/lib.rs"),
            ("test", "container", "report_package/tests/container.rs"),
            ("test", "current_result_package", "report_package/tests/current_result_package.rs"),
            ("test", "exact_sections_parity", "report_package/tests/exact_sections_parity.rs"),
            ("test", "pf_timoshenko_package", "report_package/tests/pf_timoshenko_package.rs"),
            ("test", "quantity_display_parity", "report_package/tests/quantity_display_parity.rs"),
            ("test", "reader", "report_package/tests/reader.rs"),
            ("test", "reader_allocation", "report_package/tests/reader_allocation.rs"),
            ("test", "report_diagnostics_parity", "report_package/tests/report_diagnostics_parity.rs"),
            ("test", "report_rows_parity", "report_package/tests/report_rows_parity.rs"),
            ("test", "schema_eval", "report_package/tests/schema_eval.rs"),
            ("test", "user_release_derivation", "report_package/tests/user_release_derivation.rs"),
            ("test", "user_release_package", "report_package/tests/user_release_package.rs"),
        ],
    ),
    (
        "pdf_emitter",
        &[("lib", "open_pipe_stress_pdf_emitter", "pdf_emitter/src/lib.rs"), ("test", "emit", "pdf_emitter/tests/emit.rs")],
    ),
    (
        "report_renderer",
        &[
            ("lib", "open_pipe_stress_report_renderer", "report_renderer/src/lib.rs"),
            ("test", "object_form", "report_renderer/tests/object_form.rs"),
            ("test", "render", "report_renderer/tests/render.rs"),
        ],
    ),
    (
        "headless",
        &[
            ("lib", "open_pipe_stress_headless_runner", "headless/src/lib.rs"),
            ("bin", "headless_preview_runner", "headless/src/bin/headless_preview_runner.rs"),
            ("bin", "openpipestress-runner", "headless/src/bin/openpipestress-runner.rs"),
            ("test", "del1005_package_witness", "headless/tests/del1005_package_witness.rs"),
            ("test", "exact_admission_seam", "headless/tests/exact_admission_seam.rs"),
            ("test", "exact_route_refusals", "headless/tests/exact_route_refusals.rs"),
            ("test", "load_reference_cli", "headless/tests/load_reference_cli.rs"),
            ("test", "object_form_cli", "headless/tests/object_form_cli.rs"),
            ("test", "objective_connector", "headless/tests/objective_connector.rs"),
            ("test", "physics_source_connected", "headless/tests/physics_source_connected.rs"),
            ("test", "pressure_export", "headless/tests/pressure_export.rs"),
            ("test", "preview_physics_admission", "headless/tests/preview_physics_admission.rs"),
            ("test", "reader_cli", "headless/tests/reader_cli.rs"),
            ("test", "refused_package_ids", "headless/tests/refused_package_ids.rs"),
            ("test", "retained_precision_admission", "headless/tests/retained_precision_admission.rs"),
            ("test", "s11g_zero_work_receipt", "headless/tests/s11g_zero_work_receipt.rs"),
            ("test", "tied_joint_references", "headless/tests/tied_joint_references.rs"),
            ("test", "untied_joint_pressure_references", "headless/tests/untied_joint_pressure_references.rs"),
            ("test", "user_release_runner_refused", "headless/tests/user_release_runner_refused.rs"),
        ],
    ),
];

/// Each scanned crate's build-dependencies, by name, exactly: only
/// `src-tauri`'s build script has any, and it cannot link a reporting crate;
/// the headless runner has none (code review r9 at #1682).
pub(crate) const BUILD_DEPENDENCIES: [(&str, &[&str]); 5] =
    [("src-tauri", &["tauri-build"]), ("report_package", &[]), ("pdf_emitter", &[]), ("report_renderer", &[]), ("headless", &[])];

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
    /// Outlined modules to resolve: (declaring file, inline module path, name,
    /// line, whether the module is test-only, its `#[path]` literal when the
    /// scan follows one: a `src-tauri` test module's, code review r7 at #1682).
    modules: &'a mut Vec<(String, Vec<String>, String, usize, bool, Option<String>)>,
    owners: Vec<Owner>,
    inline: Vec<String>,
    /// Entry arguments seen at calls of the producer internals: (callee, caller owner, entry, line).
    entries: &'a mut Vec<(String, Owner, Option<Entry>, String, usize)>,
    /// `UserRelease` constructions: (owner, file, line, test-only item).
    constructions: &'a mut Vec<(Owner, String, usize, Option<String>)>,
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
    /// Inside a test-only item ([`is_test_only`]), or a file only a test-only
    /// module loads (code review r6 at #1682): the construction, literal and
    /// naming rules apply; the rules that only govern production code's
    /// shape do not ([`Self::production_finding`]).
    test_only: bool,
    /// The pinned `allow(unsafe_code)` modules declared, and those seen with
    /// their allow: (declaring file, name, whether it carries the allow).
    unsafe_modules: &'a mut BTreeSet<(String, String, bool)>,
    /// The crate this file is compiled into: its own, or `src-tauri` for a
    /// file a `src-tauri` test module loads by `#[path]` from another crate
    /// (code review r7 at #1682). The crate rules follow it.
    host: String,
}

impl FileScan<'_> {
    fn crate_key(&self) -> &str {
        &self.host
    }

    /// Whether this is an allowed dependent's test-only code (code review r7
    /// at #1682 for `src-tauri`, r9 for the headless runner; §2.8,
    /// "Test-only items"): unlike the reporting crates', whose root guard
    /// stops a dependency build with `cfg(test)`, an allowed dependent is
    /// the primary package of a product build, so its test-only items are
    /// held to the caller-graph and renderer-call rules.
    fn shell_test(&self) -> bool {
        self.test_only && self.is_dependent()
    }

    /// Whether this file is compiled into an allowed dependent that is not a
    /// reporting crate ([`DEPENDENT_CRATES`]).
    fn is_dependent(&self) -> bool {
        DEPENDENT_CRATES.contains(&self.crate_key())
    }

    /// Whether this file is one of the three reporting crates, which hold no
    /// `unsafe` code: `unsafe` could build a `DerivedUserRelease` without
    /// naming it (`std::mem::zeroed`, `transmute`), so it is refused there.
    fn is_core(&self) -> bool {
        REPORTING_KEYS.contains(&self.crate_key())
    }

    /// Whether `unsafe` code is refused in this file's crate: a reporting
    /// crate's, or an allowed dependent's on [`UNSAFE_FORBIDDING_DEPENDENTS`]
    /// (code review r9 at #1682).
    fn forbids_unsafe(&self) -> bool {
        self.is_core() || UNSAFE_FORBIDDING_DEPENDENTS.contains(&self.crate_key())
    }

    /// Whether this file is compiled into the headless runner (code review r9
    /// at #1682), whose own additions to the production lists apply.
    fn is_runner(&self) -> bool {
        self.crate_key() == "headless"
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
    /// every `unsafe` keyword (code review r4), and every brace group in
    /// generic arguments ([`lexical_findings`]), in
    /// any position, macro tokens, attributes and inline test modules
    /// included. A file that does not lex is a finding.
    /// In an allowed dependent on [`UNSAFE_FORBIDDING_DEPENDENTS`] only the
    /// `unsafe` keyword rule applies (code review r9 at #1682).
    fn token_rules(&mut self) {
        let Ok(tokens) = self.text.parse::<TokenStream>() else {
            self.finding(0, "a file that does not lex");
            return;
        };
        let mut found = Vec::new();
        let label = if self.is_core() { "a reporting crate" } else { "the headless runner" };
        lexical_findings(tokens, self.is_core(), label, &mut found);
        for (line, what) in found {
            self.finding(line, what);
        }
    }

    fn unsafe_found(&mut self, line: usize, what: &str) {
        if self.is_core() {
            self.finding(line, format!("{what} in a reporting crate"));
        } else if self.forbids_unsafe() {
            self.finding(line, format!("{what} in the headless runner"));
        }
    }

    fn owner(&self) -> Owner {
        self.owners.last().cloned().unwrap_or(Owner::Module)
    }

    fn finding(&mut self, line: usize, what: impl std::fmt::Display) {
        self.findings.push(format!("{}:{line}: {what}", self.file));
    }

    /// A finding of a rule that governs only production code's shape: the
    /// invocation, attribute, import and macro-definition allow-lists, and
    /// the call and entry-argument rules. A test-only item is held to the
    /// construction, literal and naming rules instead (code review r6 at
    /// #1682; §2.8, "Test-only items"), so these are not findings there.
    fn production_finding(&mut self, line: usize, what: impl std::fmt::Display) {
        if !self.test_only {
            self.finding(line, what);
        }
    }

    /// The test-only item being read, as its path in the file: the inline
    /// modules, then the innermost scope owner's name (`tests::forge`), or
    /// `None` in production code.
    fn test_key(&self) -> Option<String> {
        if !self.test_only {
            return None;
        }
        let owner = match self.owner() {
            Owner::Fn(name) | Owner::Method(name) => name,
            other => format!("{other:?}"),
        };
        Some(self.inline.iter().cloned().chain(std::iter::once(owner)).collect::<Vec<_>>().join("::"))
    }

    /// A construction of `ProducerEntry::UserRelease` here, by `owner`.
    fn construction(&mut self, owner: Owner, line: usize) {
        let test = self.test_key();
        self.constructions.push((owner, self.file.clone(), line, test));
    }

    /// Visits `body` as a test-only item (code review r6 at #1682): no longer
    /// skipped, it is read under the construction, literal and naming rules.
    fn as_test_only(&mut self, body: impl FnOnce(&mut Self)) {
        let outer = std::mem::replace(&mut self.test_only, true);
        body(self);
        self.test_only = outer;
    }

    /// Whether `name` is a scanned name matched bare in this crate.
    fn scanned_bare(&self, name: &str) -> bool {
        let scanned = ROWS.iter().any(|(function, _, _)| *function == name) || name == DERIVED;
        scanned && !(self.crate_key() == "src-tauri" && RENDERER_PAIR.contains(&name))
    }

    fn record(&mut self, name: &str, line: usize, kind: Kind) {
        let owner = self.owner();
        // Code review r6 at #1682: in a reporting crate's test-only item only
        // the occurrences of `DerivedUserRelease` and the definitions of a
        // scanned function are read; its calls are a test's own (§2.8,
        // "Test-only items"). In `src-tauri` every occurrence is read (code
        // review r7 at #1682).
        if self.test_only && self.is_core() && name != DERIVED && kind != Kind::Definition {
            return;
        }
        let test = self.test_key();
        self.occurrences.push(Occurrence { name: name.to_string(), file: self.file.clone(), line, kind, owner, test });
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
                    self.lookup_name(&name, line);
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
                        self.construction(Owner::Macro, line);
                    }
                    // An item that binds a name, in tokens the scan does not
                    // parse (code review r4 at #1682): a `mod`, `use`,
                    // `extern crate`, `macro` (2.0) or `macro_rules!` there
                    // could shadow a listed macro or its root unseen, so each
                    // is a finding. (The first four are keywords, so as
                    // identifiers they would be raw.)
                    let raw = ident.to_string() != name;
                    if !raw && matches!(name.as_str(), "mod" | "use" | "extern" | "macro") {
                        self.finding(line, format!("the item keyword {name} in tokens the scan does not parse"));
                    }
                    if name == "macro_rules" && matches!(trees.get(index + 1), Some(TokenTree::Punct(bang)) if bang.as_char() == '!') {
                        self.finding(line, "a macro_rules! definition in tokens the scan does not parse");
                    }
                    // A nested invocation: `<path> ! <group>`.
                    if check_invocations {
                        if let (Some(TokenTree::Punct(bang)), Some(TokenTree::Group(arguments))) = (trees.get(index + 1), trees.get(index + 2)) {
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
                                // A `macro_rules!` is found above, whatever follows it.
                                if path != "macro_rules" {
                                    self.check_invocation_path(&path, &arguments.stream(), line);
                                }
                            }
                        }
                    }
                }
                // Code review r10 at #1682: an attribute in tokens the scan
                // does not parse (`#[…]` or `#![…]`, at any depth, since
                // every group is screened in turn) is checked by every
                // attribute rule, as a direct one is: the unsafe-lint,
                // linkage, attribute-list, derive and `serde` rules.
                TokenTree::Punct(pound) if pound.as_char() == '#' => {
                    let at = match trees.get(index + 1) {
                        Some(TokenTree::Punct(bang)) if bang.as_char() == '!' => index + 2,
                        _ => index + 1,
                    };
                    if let Some(TokenTree::Group(group)) = trees.get(at) {
                        if group.delimiter() == proc_macro2::Delimiter::Bracket {
                            let line = line_of(pound.span());
                            match syn::parse2::<syn::Meta>(group.stream()) {
                                Ok(meta) => self.check_meta(&meta, line, false),
                                Err(_) => self.finding(line, "an attribute in tokens the scan does not parse, which does not parse as one"),
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Dynamic symbol lookup and transmutes, by name (code review r9
    /// follow-up at #1682; [`DYNAMIC_LOOKUP`], [`TRANSMUTES`]).
    fn lookup_name(&mut self, name: &str, line: usize) {
        if DYNAMIC_LOOKUP.contains(&name) {
            self.finding(line, format!("{name}, dynamic symbol lookup"));
        }
        if TRANSMUTES.contains(&name) {
            self.finding(line, format!("{name}, a reinterpreting transmute"));
        }
    }

    /// One attribute. `forbid` is not on the attribute list: the crate-root
    /// `#![forbid(unsafe_code)]` of a reporting crate ([`is_root_forbid`]) is
    /// admitted before this, only among its root file's own attributes.
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
        // Code review r9 follow-up at #1682: linkage attributes, also in the
        // `unsafe(...)` form, and any lint attribute that could lift the
        // roots' `deny(unsafe_code)` outside the pinned modules.
        let via = if through { " (through cfg_attr)" } else { "" };
        if LINKAGE_ATTRIBUTES.contains(&path.as_str()) {
            self.finding(line, format!("the linkage attribute #[{path}]{via}"));
            return;
        }
        if path == "unsafe" {
            let inner = match meta {
                syn::Meta::List(list) => list.parse_args::<syn::Meta>().ok().map(|inner| path_text(inner.path())),
                _ => None,
            };
            match inner {
                Some(inner) if LINKAGE_ATTRIBUTES.contains(&inner.as_str()) => self.finding(line, format!("the linkage attribute #[unsafe({inner})]{via}")),
                _ => self.finding(line, format!("an unsafe(...) attribute{via}")),
            }
            return;
        }
        if matches!(path.as_str(), "allow" | "expect" | "warn") {
            if let syn::Meta::List(list) = meta {
                if names_unsafe_code(&list.tokens) {
                    self.finding(line, format!("#[{path}(unsafe_code)]{via} outside the pinned allow(unsafe_code) modules"));
                    return;
                }
            }
        }
        if through && path == "path" {
            self.finding(line, "a cfg_attr that applies #[path]");
            return;
        }
        if !ATTRIBUTES.contains(&path.as_str()) && !(self.test_only && TEST_ATTRIBUTES.contains(&path.as_str())) && !(self.shell_test() && SHELL_TEST_ATTRIBUTES.contains(&path.as_str())) {
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
                            if !DERIVES.contains(&name.as_str()) && !(self.test_only && TEST_DERIVES.contains(&name.as_str())) {
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
                // The headless runner's own (code review r9 at #1682).
                syn::Meta::Path(_) if self.is_runner() && RUNNER_SERDE_FLAGS.contains(&key.as_str()) => {}
                syn::Meta::NameValue(value) if self.is_runner() && RUNNER_SERDE_FUNCTIONS.iter().any(|(listed, _)| *listed == key) => {
                    let literal = match &value.value {
                        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(text), .. }) => Some(text.value()),
                        _ => None,
                    };
                    if !literal.as_deref().is_some_and(|text| RUNNER_SERDE_FUNCTIONS.contains(&(key.as_str(), text))) {
                        self.finding(line, format!("a {key} naming a function other than the one admitted"));
                    }
                }
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
        self.check_invocation_path(&path, &mac.tokens, line);
        let subtree = in_defining_module(&self.file);
        let listed = SUBTREE_MACROS.contains(&path.as_str());
        if subtree && !listed {
            self.production_finding(line, format!("the macro {path}! in the defining module of {DERIVED} is not on its parsed-macro list"));
        }
        // In a test-only item of the defining subtree (code review r6 at
        // #1682) an unlisted macro's tokens (`assert_eq!`, `assert!`) are
        // parsed as comma-separated expressions and visited as code; tokens
        // that do not parse so are screened, and a brace group in them there
        // is a finding.
        let parsed = if (subtree && listed) || path == "matches" {
            parse_listed_macro(&path, &mac.tokens)
        } else if subtree && self.test_only {
            parse_listed_macro("format", &mac.tokens)
        } else {
            None
        };
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
                } else if subtree && (listed || self.test_only) {
                    self.finding(line, format!("the tokens of {path}! do not parse by its grammar, in the defining module of {DERIVED}"));
                }
                self.with_owner(Owner::Macro, |scan| scan.screen_tokens(&mac.tokens, true));
            }
        }
    }

    /// An invocation's path, as written and normalized, parsed or in tokens
    /// alike: on the invocation list, and the source macros
    /// ([`SOURCE_MACROS`]) by their final segment, whatever path comes before
    /// it (code review r4 at #1682). `include!`, `include_bytes!` and
    /// `concat_idents!` are refused outright; `include_str!` only bare and
    /// with one string literal that names no `.rs` file (production reads
    /// JSON and schema text with it). `env!` and `option_env!` expand only to
    /// a string literal, so they bring in no code and need no rule beyond the
    /// list.
    fn check_invocation_path(&mut self, path: &str, tokens: &TokenStream, line: usize) {
        let runner_listed = self.is_runner() && RUNNER_MACRO_INVOCATIONS.iter().any(|(listed, _)| *listed == path);
        if !MACRO_INVOCATIONS.contains(&path) && !runner_listed {
            self.production_finding(line, format!("the macro {path}! is not on the invocation list"));
        }
        let last = path.rsplit("::").next().unwrap_or(path);
        match last {
            "include" => self.finding(line, format!("{path}! brings in source the scan does not read")),
            "include_bytes" | "concat_idents" => self.finding(line, format!("{path}! is refused by any path in production")),
            "include_str" if path != "include_str" => self.finding(line, format!("{path}! names include_str by a path other than its bare name")),
            "include_str" => {
                let literal = syn::parse2::<syn::LitStr>(tokens.clone()).ok().map(|text| text.value());
                // A `src-tauri` test's own module text, as a string (code
                // review r7 at #1682, [`SHELL_TEST_SOURCE_TEXTS`]).
                let listed = self.shell_test() && literal.as_ref().is_some_and(|text| SHELL_TEST_SOURCE_TEXTS.iter().any(|(file, listed, _)| *file == self.file && listed == text));
                if !listed && !literal.is_some_and(|text| !text.ends_with(".rs")) {
                    self.finding(line, format!("{path}! takes one string literal that names no .rs file"));
                }
            }
            _ => {}
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
        // `src-tauri`'s test-only macros are pinned too (code review r7 at
        // #1682, [`SHELL_TEST_MACROS`]).
        let pinned = MACRO_DEFINITIONS.iter().find(|(file, macro_name, _)| *file == self.file && *macro_name == name).map(|(_, _, sha)| *sha).or_else(|| {
            if self.shell_test() {
                SHELL_TEST_MACROS.iter().find(|(file, macro_name, _, _)| *file == self.file && *macro_name == name).map(|(_, _, sha, _)| *sha)
            } else {
                None
            }
        });
        if pinned.is_none() && reserved_name(&name) {
            self.finding(line, format!("the macro definition {name} takes the reserved macro-path name {name}"));
        }
        match (pinned, text) {
            (Some(sha), Some(text)) if sha256_hex(text.as_bytes()) == sha => {
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
            (None, _) if self.shell_test() => self.finding(line, format!("the macro definition {name} in a {} test-only item is not on the test macro allow-list", self.host)),
            (None, _) => self.production_finding(line, format!("the macro definition {name} is not on the allow-list")),
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
            for segment in &path {
                self.lookup_name(segment, line);
            }
            // Code review r7 at #1682: in `src-tauri`'s test-only items an
            // import naming the renderer or PDF crate (any segment) may bind
            // only one of its other items by name: a glob, the crate itself
            // (`as` any name, or `self`), or the renderer's pair is a finding,
            // so the pair is named there only through a path the scan reads.
            let shell_crate_import = self.shell_test() && path.iter().any(|segment| segment == RENDERER_CRATE || segment == PDF_CRATE);
            if glob {
                if shell_crate_import {
                    self.finding(line, format!("a glob import of {joined} in a {crate_key} test-only item"));
                }
                self.production_finding(line, format!("a glob import of {joined}"));
                continue;
            }
            if shell_crate_import {
                let last = path.last().map(String::as_str).unwrap_or_default();
                let binds_crate = path.len() == 1 || last == "self" || last == RENDERER_CRATE || last == PDF_CRATE;
                if binds_crate || path.iter().any(|segment| RENDERER_PAIR.contains(&segment.as_str())) {
                    self.finding(line, format!("the import {joined} in a {crate_key} test-only item binds the renderer or PDF crate or the renderer's pair"));
                }
            }
            let last = path.last().cloned().unwrap_or_default();
            if let (Some(rename), "report_package") = (&rename, crate_key.as_str()) {
                // A renamed import could name any type under a struct's name.
                self.type_names.aliases.insert(rename.clone());
            }
            let name = rename.clone().unwrap_or_else(|| if last == "self" { path.iter().rev().nth(1).cloned().unwrap_or_default() } else { last.clone() });
            let listed_name = reserved_name(&name) || DERIVES.contains(&name.as_str());
            // Code review r4 at #1682: the source macros by any path.
            let imported = if last == "self" { path.iter().rev().nth(1).cloned().unwrap_or_default() } else { last.clone() };
            if SOURCE_MACROS.contains(&imported.as_str()) {
                self.finding(line, format!("the import {joined} binds the source macro {imported}"));
            }
            if REPORTING_KEYS.contains(&crate_key.as_str()) {
                // Code review r4 at #1682: whether a path names a macro is not
                // decidable from source, so in the three reporting crates
                // every rename and every non-private import is on a closed
                // list of today's.
                if let Some(rename) = &rename {
                    if !USE_RENAMES.contains(&(joined.as_str(), rename.as_str())) {
                        self.production_finding(line, format!("the import {joined} as {rename}, a rename not on the rename allow-list"));
                    }
                }
                if public && !PUBLIC_USES.contains(&(self.file.as_str(), joined.as_str())) {
                    self.production_finding(line, format!("a non-private use of {joined}, which is not on the re-export allow-list"));
                }
            }
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
                        self.production_finding(line, format!("the import {joined} binds the renderer or PDF crate as {name}"));
                    }
                    let exception = joined == format!("{RENDERER_CRATE}::serde_error_kind") && rename.is_none();
                    if public && (renderer || pdf) && !exception {
                        self.production_finding(line, format!("a {} use of {joined} re-exports the renderer or PDF crate", "non-private"));
                    }
                }
                // The allowed dependents (code review r9 at #1682 for the
                // headless runner): no import of the renderer or PDF crate or
                // of a scanned function.
                "src-tauri" | "headless" => {
                    if names_crate(RENDERER_CRATE) || names_crate(PDF_CRATE) || path.iter().any(|segment| ROWS.iter().any(|(function, _, _)| function == segment)) {
                        self.production_finding(line, format!("{crate_key} imports {joined}"));
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
                    (Some(Entry::Released), "&ProducerEntry::UserRelease(derived)".to_string())
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
        // Test-only items are read in every scanned crate: the reporting crates'
        // since code review r6, `src-tauri`'s since code review r7 at #1682.
        if is_test_only(attrs) && !self.test_only {
            self.as_test_only(|scan| scan.visit_item(item));
            return;
        }
        match item {
            syn::Item::Verbatim(tokens) => {
                let line = tokens.clone().into_iter().next().map_or(0, |tree| line_of(tree.span()));
                self.finding(line, "an item syn gives as Verbatim (a macro 2.0 item)");
            }
            syn::Item::ExternCrate(extern_crate) => {
                self.check_attributes(&extern_crate.attrs);
                let line = line_of(extern_crate.crate_token.span);
                self.finding(line, format!("extern crate {}", extern_crate.ident));
                let bound = extern_crate.rename.as_ref().map_or_else(|| ident_text(&extern_crate.ident), |(_, rename)| ident_text(rename));
                if reserved_name(&bound) {
                    self.finding(line, format!("extern crate binds the reserved macro-path name {bound}"));
                }
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
        if reserved_name(&name) {
            self.finding(line, format!("the module {name} takes the reserved macro-path name {name}"));
        }
        let mut followed = None;
        let paths = module.attrs.iter().filter(|attr| path_text(attr.path()) == "path").count();
        // Code review r9 follow-up at #1682: a pinned module's own
        // `#[allow(unsafe_code)]` ([`SHELL_UNSAFE_MODULES`]).
        // Code review r10 at #1682: only the file-level `mod` item, outside
        // every inline module and every function, block, closure, `const`
        // or `static` (the owner stack holds only the module), matches a
        // pin; a block-local or nested `mod` of the same name is not it.
        let pinned = self.inline.is_empty() && self.owners.len() == 1 && SHELL_UNSAFE_MODULES.iter().any(|(file, pinned, _)| *file == self.file && *pinned == name);
        if pinned {
            self.unsafe_modules.insert((self.file.clone(), name.clone(), false));
        }
        for attr in &module.attrs {
            if pinned && is_module_allow(attr) {
                self.unsafe_modules.insert((self.file.clone(), name.clone(), true));
                continue;
            }
            if path_text(attr.path()) == "path" {
                // Code review r7 at #1682: an allowed dependent's test
                // module's one `#[path = "<literal>"]`, on an outlined module
                // outside any inline module, is followed ([`path_module_key`])
                // and the file read as that crate's test code; any other
                // `#[path]` is a finding, as in production. Since code review
                // r9, a production module's is followed too, wherever it
                // points, and read as production, besides being a finding.
                let literal = match &attr.meta {
                    syn::Meta::NameValue(syn::MetaNameValue { value: syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(text), .. }), .. }) => Some(text.value()),
                    _ => None,
                };
                let followable = paths == 1 && module.content.is_none() && self.inline.is_empty();
                match literal {
                    Some(literal) if self.shell_test() && followable => followed = Some(literal),
                    _ if self.shell_test() => self.finding(line, format!("a #[path] on the {} test module {name} that is not one string literal on an outlined module outside inline modules", self.host)),
                    Some(literal) if !self.test_only && followable => {
                        self.finding(line, format!("a #[path] on the production module {name}"));
                        followed = Some(literal);
                    }
                    _ => self.finding(line, format!("a #[path] on the production module {name}")),
                }
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
            None => self.modules.push((self.file.clone(), self.inline.clone(), name, line, self.test_only, followed)),
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
        if is_test_only(attrs) && !self.test_only {
            self.as_test_only(|scan| visit::visit_impl_item(scan, item));
        } else {
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
        if is_test_only(&function.attrs) && !self.test_only {
            self.as_test_only(|scan| scan.visit_trait_item_fn(function));
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
                // A test-only one included (code review r6 at #1682).
                if let syn::ImplItem::Type(assoc) = member {
                    let assigned = if matches!(&assoc.ty, syn::Type::Path(path) if path.qself.is_none() && path.path.is_ident("Self")) { "Self" } else { "a type" };
                    self.finding(line_of(assoc.ident.span()), format!("the associated type {} assigned {assigned} in an impl for {DERIVED}", ident_text(&assoc.ident)));
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
        let line = line_of(item.abi.extern_token.span);
        self.unsafe_found(line, "an extern block");
        // Code review r9 follow-up at #1682: only a pinned block
        // ([`FOREIGN_BLOCKS`]), with no attribute and no `unsafe`, its items
        // exactly the pinned text.
        let abi = item.abi.name.as_ref().map(|name| name.value()).unwrap_or_default();
        let (open, close) = (item.brace_token.span.open().end(), item.brace_token.span.close().start());
        let items = byte_offset(self.text, open.line, open.column)
            .zip(byte_offset(self.text, close.line, close.column))
            .and_then(|(from, to)| self.text.get(from..to))
            .map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "));
        let pinned = item.attrs.is_empty()
            && item.unsafety.is_none()
            && !self.in_macro
            && items.as_deref().is_some_and(|items| FOREIGN_BLOCKS.iter().any(|(file, listed_abi, listed)| *file == self.file && *listed_abi == abi && *listed == items));
        if !pinned {
            self.finding(line, format!("the foreign block extern {abi:?} {{ {} }}, which is not on the foreign-block pin list", items.unwrap_or_default()));
        }
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
                if !self.test_only && ENTRY_RULES.iter().any(|(callee, _, _)| *callee == name) {
                    let (entry, text) = Self::entry_of(&call.args);
                    let owner = self.owner();
                    self.entries.push((name.clone(), owner, entry, text, line));
                }
                consumed = true;
            } else if path.path.segments.last().is_some_and(|segment| ident_text(&segment.ident) == "UserRelease") {
                self.construction(self.owner(), line);
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
            self.construction(self.owner(), line);
        }
        visit::visit_expr_path(self, path);
    }

    fn visit_expr_struct(&mut self, literal: &'ast syn::ExprStruct) {
        let line = literal.path.segments.last().map_or(0, |segment| line_of(segment.ident.span()));
        let last = literal.path.segments.last().map(|segment| ident_text(&segment.ident)).unwrap_or_default();
        // `ProducerEntry::UserRelease { 0: derived }` constructs one, as its
        // call form does (Codex round 2 at #1682, P2).
        if last == "UserRelease" {
            self.construction(self.owner(), line);
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
        self.lookup_name(&name, line_of(ident.span()));
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
///
/// With `reporting` false (an allowed dependent on
/// [`UNSAFE_FORBIDDING_DEPENDENTS`]; code review r9 at #1682) only the
/// `unsafe` keyword is found, the finding naming `label`.
fn lexical_findings(tokens: TokenStream, reporting: bool, label: &str, out: &mut Vec<(usize, String)>) {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    let mut open = 0usize;
    for (index, tree) in trees.iter().enumerate() {
        let previous = index.checked_sub(1).map(|at| &trees[at]);
        let previous_char = |chars: &[char]| matches!(previous, Some(TokenTree::Punct(punct)) if chars.contains(&punct.as_char()));
        match tree {
            TokenTree::Ident(ident) => {
                let text = ident.to_string();
                // Code review r4 at #1682: `unsafe` in any position, macro
                // tokens included, besides the crate root's
                // `#![forbid(unsafe_code)]`, which the compiler enforces.
                if text == "unsafe" {
                    out.push((line_of(ident.span()), format!("the unsafe keyword in {label}")));
                }
                if !reporting {
                    continue;
                }
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
                if reporting && group.delimiter() == proc_macro2::Delimiter::Brace {
                    if previous_char(&['<']) || (open > 0 && previous_char(&[',', '='])) {
                        out.push((line_of(group.span()), "a brace or block expression in generic arguments in a reporting crate".to_string()));
                    }
                }
                if group.delimiter() == proc_macro2::Delimiter::Brace {
                    open = 0;
                }
                lexical_findings(group.stream(), reporting, label, out);
            }
            TokenTree::Literal(_) => {}
        }
    }
}

/// The module file that `name`, declared in `declaring` under the inline
/// modules `inline`, loads: `name.rs` or `name/mod.rs` in the directory of a
/// crate root (one of `roots`, the target files Cargo reports) or a `mod.rs`
/// file, else in the directory named after the declaring file, under the
/// inline modules as further directories.
fn module_file(files: &BTreeMap<String, String>, roots: &BTreeSet<String>, declaring: &str, inline: &[String], name: &str) -> Result<String, String> {
    let (directory, stem) = declaring.rsplit_once('/').unwrap_or(("", declaring));
    let stem = stem.trim_end_matches(".rs");
    let is_root = roots.contains(declaring) || stem == "mod";
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

/// The file a module's `#[path = "<literal>"]` loads (code review r7 at
/// #1682 for `src-tauri`'s test modules; r9 for every followed one), as a
/// `<crate key>/...` path: on an outlined module outside inline modules,
/// Rust resolves the literal from the declaring file's directory. The
/// literal is joined in the project tree ([`CRATES`]), `.` and `..`
/// resolved, and the result must lie in one of the scanned crates; an
/// absolute literal, a backslash, or a file outside them is an error (a
/// finding). One that leaves the declaring crate's directory is a finding
/// besides, unless listed ([`PATH_CROSSINGS`]).
pub(crate) fn path_module_key(declaring: &str, literal: &str) -> Result<String, String> {
    let refuse = |why: &str| Err(format!("{declaring}: the #[path] {literal:?} {why}"));
    if literal.is_empty() || literal.starts_with('/') || literal.contains('\\') || literal.contains(':') {
        return refuse("is not a relative path");
    }
    let key = crate_of(declaring);
    let Some((_, root)) = CRATES.iter().find(|(crate_key, _)| *crate_key == key) else { return refuse("is declared outside the scanned crates") };
    let relative = &declaring[key.len()..];
    let joined = format!("{root}{relative}");
    let mut segments: Vec<String> = joined.split('/').map(str::to_string).collect();
    segments.pop();
    for segment in literal.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.pop().is_none() {
                    return refuse("leaves the project tree");
                }
            }
            other => segments.push(other.to_string()),
        }
    }
    let resolved = segments.join("/");
    for (crate_key, root) in CRATES {
        if let Some(rest) = resolved.strip_prefix(&format!("{root}/")) {
            return Ok(format!("{crate_key}/{rest}"));
        }
    }
    refuse(&format!("loads {resolved}, outside the scanned crates"))
}

/// The scan over `sources`: every finding, as text. Empty when the boundary
/// holds.
pub(crate) fn scan(sources: &Sources) -> Vec<String> {
    scan_reading(sources).0
}

/// [`scan`], with the production files it read.
pub(crate) fn scan_reading(sources: &Sources) -> (Vec<String>, BTreeSet<String>) {
    let (findings, production, _) = scan_reading_all(sources);
    (findings, production)
}

/// [`scan`], with the production files it read and the files it read only
/// as test-only modules (code review r6 at #1682).
pub(crate) fn scan_reading_all(sources: &Sources) -> (Vec<String>, BTreeSet<String>, BTreeSet<String>) {
    let mut findings = Vec::new();
    // Targets, as Cargo reports them (code review r5 at #1682): exactly the
    // allow-listed set, the library of each reporting crate at its expected
    // root, and the build-dependencies exactly. The crate roots read are the
    // actual library and binary target files, whatever the expected ones.
    let mut roots: BTreeSet<String> = BTreeSet::new();
    let mut forbid_roots: BTreeSet<String> = BTreeSet::new();
    let mut deny_roots: BTreeSet<String> = BTreeSet::new();
    let mut unsafe_modules: BTreeSet<(String, String, bool)> = BTreeSet::new();
    for (key, expected) in TARGETS {
        let actual: BTreeSet<CrateTarget> = sources.targets.get(key).into_iter().flatten().cloned().collect();
        let expected: BTreeSet<CrateTarget> =
            expected.iter().map(|(kind, name, src_path)| CrateTarget { kind: kind.to_string(), name: name.to_string(), src_path: src_path.to_string() }).collect();
        if actual.is_empty() {
            findings.push(format!("{key}: Cargo reports no target"));
        }
        for target in actual.difference(&expected) {
            let CrateTarget { kind, name, src_path } = target;
            findings.push(format!("{key}: Cargo builds the {kind} target {name} from {src_path}, which is not on the target allow-list"));
        }
        for target in expected.difference(&actual) {
            let CrateTarget { kind, name, src_path } = target;
            findings.push(format!("{key}: Cargo does not build the allow-listed {kind} target {name} from {src_path}"));
        }
        for target in actual.iter().filter(|target| is_scanned_target(&target.kind)) {
            roots.insert(target.src_path.clone());
        }
        if let Some((_, expected_root)) = REPORTING_ROOTS.iter().find(|(reporting, _)| *reporting == key) {
            let libraries: Vec<&CrateTarget> = actual.iter().filter(|target| target.kind == "lib").collect();
            match libraries.as_slice() {
                [library] => {
                    if library.src_path != *expected_root {
                        findings.push(format!("{key}: Cargo's library root is {}, not {expected_root}", library.src_path));
                    }
                    forbid_roots.insert(library.src_path.clone());
                }
                _ => findings.push(format!("{key}: Cargo reports {} lib targets, not exactly one", libraries.len())),
            }
        }
        // Code review r9 at #1682: every library and binary root of an
        // allowed dependent on [`UNSAFE_FORBIDDING_DEPENDENTS`] carries
        // `#![forbid(unsafe_code)]`, as a reporting crate's library root does.
        if UNSAFE_FORBIDDING_DEPENDENTS.contains(&key) {
            for target in actual.iter().filter(|target| is_scanned_target(&target.kind)) {
                forbid_roots.insert(target.src_path.clone());
            }
        }
        // Code review r9 follow-up at #1682: every `src-tauri` library and
        // binary root carries `#![deny(unsafe_code)]`.
        if key == "src-tauri" {
            for target in actual.iter().filter(|target| is_scanned_target(&target.kind)) {
                deny_roots.insert(target.src_path.clone());
            }
        }
        let build: BTreeSet<String> = sources.build_dependencies.get(key).cloned().unwrap_or_default();
        let expected_build: BTreeSet<String> =
            BUILD_DEPENDENCIES.iter().find(|(crate_key, _)| *crate_key == key).map(|(_, names)| names.iter().map(|name| name.to_string()).collect()).unwrap_or_default();
        if build != expected_build {
            findings.push(format!("{key}: the build-dependencies are {build:?}, not {expected_build:?}"));
        }
        if sources.bin_dirs.contains(key) && !BIN_DIRS.contains(&key) {
            findings.push(format!("{key} has a src/bin directory"));
        }
        // Code review r7 at #1682: every build script Cargo reports is pinned
        // by content ([`BUILD_SCRIPTS`]).
        for target in actual.iter().filter(|target| target.kind.split(',').any(|kind| kind == "custom-build")) {
            let pin = BUILD_SCRIPTS.iter().find(|(file, _, _)| *file == target.src_path);
            match (pin, sources.build_scripts.get(&target.src_path)) {
                (None, _) => findings.push(format!("{key}: the build script {} is not on the build-script pin list", target.src_path)),
                (Some(_), None) => findings.push(format!("{key}: the build script {} was not read", target.src_path)),
                (Some((file, length, sha)), Some(text)) => {
                    if text.len() != *length || sha256_hex(text.as_bytes()) != *sha {
                        findings.push(format!(
                            "{file}: the build script differs from its pin ({} B, SHA-256 {}; pinned {length} B, {sha}): review it, a cargo:rustc-cfg line included, and replace its pin",
                            text.len(),
                            sha256_hex(text.as_bytes())
                        ));
                    }
                }
            }
        }
        // Code review r7 at #1682: `src-tauri`'s normal and development
        // dependencies are exactly [`SHELL_DEPENDENCIES`]; since code review
        // r9, the headless runner's are exactly [`RUNNER_DEPENDENCIES`]
        // ([`DEPENDENCY_PINS`]).
        if let Some((_, pinned)) = DEPENDENCY_PINS.iter().find(|(pinned_key, _)| *pinned_key == key) {
            let actual: BTreeSet<String> = sources.dependencies.get(key).cloned().unwrap_or_default();
            let expected: BTreeSet<String> = pinned.iter().map(|dependency| dependency.to_string()).collect();
            for dependency in actual.difference(&expected) {
                findings.push(format!("{key}: Cargo reports the dependency {dependency}, which is not on the dependency pin list"));
            }
            for dependency in expected.difference(&actual) {
                findings.push(format!("{key}: Cargo does not report the pinned dependency {dependency}"));
            }
        }
    }
    let mut occurrences = Vec::new();
    let mut entries = Vec::new();
    let mut constructions = Vec::new();
    let mut macros = Vec::new();
    let mut type_names = TypeNames::default();
    let mut plain_literals = Vec::new();
    // Each file is read once as production and once as a test-only module,
    // as its declarations load it, for each crate that compiles it (a file a
    // `src-tauri` test module loads by `#[path]` from another crate is read
    // as `src-tauri`'s too; code review r7 at #1682).
    let mut queue: Vec<(String, bool, String)> = roots.iter().rev().map(|root| (root.clone(), false, crate_of(root).to_string())).collect();
    let mut read: BTreeSet<String> = BTreeSet::new();
    let mut test_read: BTreeSet<String> = BTreeSet::new();
    let mut hosted: BTreeSet<(String, bool, String)> = BTreeSet::new();
    // Files loaded by `#[path]`, and the outlined modules each file declares.
    let mut path_loaded: BTreeSet<String> = BTreeSet::new();
    let mut outlined: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();
    while let Some((file, test_only, host)) = queue.pop() {
        if !hosted.insert((file.clone(), test_only, host.clone())) {
            continue;
        }
        (if test_only { &mut test_read } else { &mut read }).insert(file.clone());
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
            test_only,
            unsafe_modules: &mut unsafe_modules,
            host: host.clone(),
        };
        if scan.forbids_unsafe() {
            scan.token_rules();
        }
        // Code review r4 at #1682: each reporting crate's actual library root
        // carries `#![forbid(unsafe_code)]` exactly. It is admitted only
        // among that file's own attributes (`syn::File.attrs`): a nested
        // module's inner `#![forbid]` is checked as any other attribute
        // (code review r5 at #1682).
        let forbid_root = forbid_roots.contains(&file);
        if forbid_root && !parsed.attrs.iter().any(is_root_forbid) {
            scan.finding(0, "the crate root does not carry #![forbid(unsafe_code)] exactly");
        }
        let deny_root = deny_roots.contains(&file) && !test_only;
        if deny_root && !parsed.attrs.iter().any(is_root_deny) {
            scan.finding(0, "the crate root does not carry #![deny(unsafe_code)] exactly");
        }
        for attr in &parsed.attrs {
            if !(forbid_root && is_root_forbid(attr)) && !(deny_root && is_root_deny(attr)) {
                scan.visit_attribute(attr);
            }
        }
        for item in &parsed.items {
            scan.visit_item(item);
        }
        for (declaring, inline, name, line, test_module, followed) in modules {
            outlined.entry(declaring.clone()).or_default().push((name.clone(), line));
            let next = match followed {
                Some(literal) => path_module_key(&declaring, &literal).map(|next| {
                    path_loaded.insert(next.clone());
                    // Code review r9 at #1682: a `#[path]` that leaves the
                    // declaring crate's directory is a finding, unless listed
                    // ([`PATH_CROSSINGS`]); the file is still read.
                    if crate_of(&next) != crate_of(&declaring) && !PATH_CROSSINGS.iter().any(|(from, to, _)| *from == declaring && *to == next) {
                        findings.push(format!("{declaring}:{line}: the #[path] {literal:?} of the module {name} loads {next}, outside its crate's directory, and is not on the path-crossing allow-list"));
                    }
                    next
                }),
                None => module_file(&sources.files, &roots, &declaring, &inline, &name),
            };
            match next {
                Ok(next) => queue.push((next, test_module, host.clone())),
                Err(error) => findings.push(format!("{error} (line {line})")),
            }
        }
    }
    // Code review r9 follow-up at #1682: each pinned module that is
    // declared carries its `#[allow(unsafe_code)]`.
    for (file, name, _) in SHELL_UNSAFE_MODULES {
        let key = |allowed: bool| (file.to_string(), name.to_string(), allowed);
        if unsafe_modules.contains(&key(false)) && !unsafe_modules.contains(&key(true)) {
            findings.push(format!("{file}: the pinned module {name} does not carry #[allow(unsafe_code)] on its mod item"));
        }
    }
    // A file loaded by `#[path]` declares no outlined module: Rust resolves
    // such a module's file from the path-loaded file's own directory, which
    // the scan does not model (code review r7 at #1682).
    for file in &path_loaded {
        for (name, line) in outlined.get(file).into_iter().flatten() {
            findings.push(format!("{file}:{line}: the outlined module {name} in a file loaded by #[path]"));
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
    // Test-only items (code review r6 at #1682): a `DerivedUserRelease`
    // literal only in an item on [`TEST_PROOF_ITEMS`], exactly once; any other
    // name of it but a type, and any definition of it or of a scanned
    // function, is a finding.
    let mut test_literals: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut shell_test_calls: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for occurrence in &occurrences {
        let Occurrence { name, file, line, kind, owner, test } = occurrence;
        let at = format!("{file}:{line}");
        if let Some(item) = test {
            // `src-tauri`'s test-only items (code review r7 at #1682): a
            // scanned name only as listed ([`SHELL_TEST_CALLERS`]), exactly
            // once each; with the list empty, every occurrence is a finding.
            if name != DERIVED && SHELL_TEST_CALLERS.iter().any(|(listed_file, listed, listed_name, _)| listed_file == file && listed == item && listed_name == name) && *kind == Kind::Call {
                *shell_test_calls.entry((file.clone(), item.clone(), name.clone())).or_default() += 1;
                continue;
            }
            match kind {
                Kind::TypeUse if name == DERIVED => {}
                Kind::Literal if TEST_PROOF_ITEMS.iter().any(|(listed_file, listed, _)| listed_file == file && listed == item) => {
                    *test_literals.entry((file.clone(), item.clone())).or_default() += 1;
                }
                Kind::Literal => findings.push(format!("{at}: a {DERIVED} literal in the test-only item {item}, which is not on the test allow-list")),
                Kind::Definition => findings.push(format!("{at}: {name} is defined again in a test-only item")),
                other => findings.push(format!("{at}: {name} {} in the test-only item {item}", describe(other))),
            }
            continue;
        }
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
    for (file, item, name, _) in SHELL_TEST_CALLERS {
        let count = shell_test_calls.get(&(file.to_string(), item.to_string(), name.to_string())).copied().unwrap_or(0);
        if count != 1 {
            findings.push(format!("{file}: the allow-listed src-tauri test item {item} calls {name} {count} times, not exactly once"));
        }
    }
    if definitions.get(DERIVED).copied().unwrap_or(0) != 1 {
        findings.push(format!("{DERIVED_FILE}: {DERIVED} is not defined there exactly once"));
    }
    if !occurrences.iter().any(|occurrence| occurrence.name == DERIVED && occurrence.kind == Kind::Literal && occurrence.test.is_none()) {
        findings.push(format!("{DERIVED_FILE}: check_derived constructs no {DERIVED}"));
    }
    // Each pinned macro is defined exactly once.
    for (file, name) in MACRO_DEFINITIONS.iter().map(|(file, name, _)| (file, name)).chain(SHELL_TEST_MACROS.iter().map(|(file, name, _, _)| (file, name))) {
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
    // UserRelease is constructed only in assemble_derived, and in a test-only
    // item only on [`TEST_PROOF_ITEMS`], exactly once (code review r6 at #1682).
    let mut test_constructions: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (owner, file, line, test) in &constructions {
        if let Some(item) = test {
            if TEST_PROOF_ITEMS.iter().any(|(listed_file, listed, _)| listed_file == file && listed == item) {
                *test_constructions.entry((file.clone(), item.clone())).or_default() += 1;
            } else {
                findings.push(format!("{file}:{line}: ProducerEntry::UserRelease is constructed in the test-only item {item}, which is not on the test allow-list"));
            }
        } else if !(file == "report_package/src/user_release.rs" && *owner == Owner::Fn("assemble_derived".to_string())) {
            findings.push(format!("{file}:{line}: ProducerEntry::UserRelease is constructed outside assemble_derived ({owner:?})"));
        }
    }
    for (file, item, _) in TEST_PROOF_ITEMS {
        let key = (file.to_string(), item.to_string());
        let literals = test_literals.get(&key).copied().unwrap_or(0);
        let entries = test_constructions.get(&key).copied().unwrap_or(0);
        if literals != 1 || entries != 1 {
            findings.push(format!("{file}: the allow-listed test-only item {item} writes {literals} {DERIVED} literals and {entries} ProducerEntry::UserRelease constructions, not exactly one of each"));
        }
    }
    (findings, read, test_read)
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

/// The real sources of the scanned crates, read from the project tree, with
/// each crate's targets from Cargo ([`crate_sources`]). Read once per test
/// process for each project path.
pub(crate) fn project_sources(project: &Path) -> Sources {
    static READ: std::sync::Mutex<BTreeMap<std::path::PathBuf, Sources>> = std::sync::Mutex::new(BTreeMap::new());
    let mut read = READ.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    read.entry(project.to_path_buf())
        .or_insert_with(|| {
            let mut sources = Sources::default();
            for (key, root) in CRATES {
                crate_sources(key, &project.join(root), &mut sources);
            }
            sources
        })
        .clone()
}

/// One crate's part of [`Sources`], read from `crate_dir` under the key
/// `key`: every `.rs` file in the crate's directory, at any depth but under a
/// `target` or `.git` directory (code review r9 at #1682: a `#[path]` can load
/// a file outside `src/`, `../r9.rs` included, and the scan follows it),
/// whether it has `src/bin`, and its targets and build-dependencies from
/// [`cargo_crate`]. Only files a crate root or a module declaration loads are
/// read as code. Panics when Cargo cannot report them, so the scan fails
/// rather than assuming a layout.
pub(crate) fn crate_sources(key: &str, crate_dir: &Path, sources: &mut Sources) {
    fn read_dir(dir: &Path, prefix: &str, out: &mut BTreeMap<String, String>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                if name != "target" && name != ".git" {
                    read_dir(&path, &format!("{prefix}/{name}"), out);
                }
            } else if name.ends_with(".rs") {
                out.insert(format!("{prefix}/{name}"), std::fs::read_to_string(&path).unwrap());
            }
        }
    }
    read_dir(crate_dir, key, &mut sources.files);
    sources.crate_dirs.insert(key.to_string(), std::fs::canonicalize(crate_dir).unwrap_or_else(|error| panic!("{}: {error}", crate_dir.display())));
    if crate_dir.join("src/bin").is_dir() {
        sources.bin_dirs.insert(key.to_string());
    }
    let (targets, build_dependencies, dependencies) = cargo_crate(key, crate_dir).unwrap_or_else(|error| panic!("{key}: {error}"));
    // Each build script's text, for its pin (code review r7 at #1682).
    for target in targets.iter().filter(|target| target.kind.split(',').any(|kind| kind == "custom-build")) {
        if let Some(relative) = target.src_path.strip_prefix(&format!("{key}/")) {
            if let Ok(text) = std::fs::read_to_string(crate_dir.join(relative)) {
                sources.build_scripts.insert(target.src_path.clone(), text);
            }
        }
    }
    sources.dependencies.insert(key.to_string(), dependencies);
    for target in targets.iter().filter(|target| is_scanned_target(&target.kind)) {
        if let Some(relative) = target.src_path.strip_prefix(&format!("{key}/")) {
            if !sources.files.contains_key(&target.src_path) {
                if let Ok(text) = std::fs::read_to_string(crate_dir.join(relative)) {
                    sources.files.insert(target.src_path.clone(), text);
                }
            }
        }
    }
    sources.targets.insert(key.to_string(), targets);
    sources.build_dependencies.insert(key.to_string(), build_dependencies);
}

/// The targets, the build-dependency names, and the normal and development
/// dependencies ([`SHELL_DEPENDENCIES`]'s form; code review r7 at #1682)
/// Cargo reports for the crate in
/// `crate_dir` (code review r5 at #1682), by `cargo metadata --format-version
/// 1 --offline --locked --no-deps --manifest-path <crate>/Cargo.toml`, run
/// with the `cargo` that runs this test (`CARGO`). Each target's source path
/// is given relative to the crate directory under `key`, or absolute when it
/// lies outside it.
pub(crate) fn cargo_crate(key: &str, crate_dir: &Path) -> Result<(Vec<CrateTarget>, BTreeSet<String>, BTreeSet<String>), String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into());
    let manifest = crate_dir.join("Cargo.toml");
    let output = std::process::Command::new(&cargo)
        .args(["metadata", "--format-version", "1", "--offline", "--locked", "--no-deps", "--manifest-path"])
        .arg(&manifest)
        .output()
        .map_err(|error| format!("cargo metadata did not run: {error}"))?;
    if !output.status.success() {
        return Err(format!("cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|error| format!("cargo metadata output: {error}"))?;
    let canonical_manifest = std::fs::canonicalize(&manifest).map_err(|error| format!("{}: {error}", manifest.display()))?;
    let canonical_dir = canonical_manifest.parent().unwrap().to_path_buf();
    let packages = metadata["packages"].as_array().ok_or("cargo metadata reports no packages")?;
    let package = packages
        .iter()
        .find(|package| package["manifest_path"].as_str().and_then(|path| std::fs::canonicalize(path).ok()).as_ref() == Some(&canonical_manifest))
        .ok_or("cargo metadata reports no package for the manifest")?;
    let mut targets = Vec::new();
    for target in package["targets"].as_array().ok_or("a package without targets")? {
        let kind = target["kind"].as_array().ok_or("a target without kinds")?.iter().map(|kind| kind.as_str().unwrap_or("?")).collect::<Vec<_>>().join(",");
        let name = target["name"].as_str().ok_or("a target without a name")?.to_string();
        let src = Path::new(target["src_path"].as_str().ok_or("a target without a source path")?);
        let src = std::fs::canonicalize(src).unwrap_or_else(|_| src.to_path_buf());
        let src_path = match src.strip_prefix(&canonical_dir) {
            Ok(relative) => format!("{key}/{}", relative.to_string_lossy().replace('\\', "/")),
            Err(_) => src.to_string_lossy().to_string(),
        };
        targets.push(CrateTarget { kind, name, src_path });
    }
    targets.sort();
    let build_dependencies = package["dependencies"]
        .as_array()
        .ok_or("a package without dependencies")?
        .iter()
        .filter(|dependency| dependency["kind"].as_str() == Some("build"))
        .map(|dependency| dependency["name"].as_str().unwrap_or("?").to_string())
        .collect();
    let mut dependencies = BTreeSet::new();
    for dependency in package["dependencies"].as_array().ok_or("a package without dependencies")? {
        let kind = match dependency["kind"].as_str() {
            None => "normal",
            Some("dev") => "dev",
            Some("build") => continue,
            Some(_) => "unknown",
        };
        let mut text = format!("{kind} {}", dependency["name"].as_str().unwrap_or("?"));
        if let Some(rename) = dependency["rename"].as_str() {
            text.push_str(&format!(" as {rename}"));
        }
        if let Some(platform) = dependency["target"].as_str() {
            text.push_str(&format!(" ({platform})"));
        }
        dependencies.insert(text);
    }
    Ok((targets, build_dependencies, dependencies))
}

/// The consumers of the reporting crates whose Cargo resolution is checked
/// (code review r6 at #1682), as (key, project-relative manifest, how):
/// the desktop shell, by its full resolved graph, and the headless runner,
/// which depends on `report_package`, by Cargo's reading of its declared
/// dependency. The runner's full graph is not resolved: its lock holds
/// registry versions the desktop's does not, so an offline resolve of it
/// fails where only the desktop's dependencies are fetched (the hosted
/// desktop Rust job).
pub(crate) const CONSUMERS: [(&str, &str, Resolution); 2] =
    [("src-tauri", "apps/desktop/src-tauri/Cargo.toml", Resolution::Graph), ("headless", "core/runner/headless/Cargo.toml", Resolution::Declared)];

/// The canonical repository root. A standalone SWBPIPE project owns its root;
/// recognizable historical monorepo fixtures retain their enclosing root.
pub(crate) fn repository_root(project: &Path) -> Option<PathBuf> {
    let project = std::fs::canonicalize(project).ok()?;
    if project.join("AGENTS.md").is_file() && project.join("software-workflow.json").is_file() {
        return Some(project);
    }
    let root = project.parent()?.parent()?.to_path_buf();
    (root.join("AGENTS.md").is_file() && root.join("projects").is_dir()).then_some(root)
}

/// How a consumer's reporting packages are established ([`CONSUMERS`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Resolution {
    /// `cargo metadata --offline --locked --filter-platform host-tuple` with
    /// dependencies, run in the consumer's directory: the packages Cargo
    /// resolves, `.cargo` `paths` overrides, `[patch]` and `[replace]`
    /// applied ([`resolved_identities`]).
    Graph,
    /// `cargo metadata --offline --locked --no-deps`: the path Cargo reads
    /// for each declared reporting dependency ([`declared_identities`]).
    Declared,
}

/// The reporting packages, by Cargo package name, with their crate key in
/// [`CRATES`]: each must resolve to exactly one package, from the directory
/// the scan read ([`Sources::crate_dirs`]).
pub(crate) const REPORTING_PACKAGES: [(&str, &str); 3] = [(PACKAGE_CRATE, "report_package"), (RENDERER_CRATE, "report_renderer"), (PDF_CRATE, "pdf_emitter")];

/// The desktop shell's and the headless runner's Cargo package names.
const SHELL_PACKAGE: &str = "openpipestress-desktop";
const RUNNER_PACKAGE: &str = "open_pipe_stress_headless_runner";

/// The packages allowed to depend on each reporting package, by Cargo
/// package name, exactly (code review r8 at #1682; §2.8, "Dependents"). Each
/// is identified by its name and its canonical manifest
/// ([`dependent_locations`]): a reporting package's is the `Cargo.toml` the
/// scan read, and the others' are in [`DEPENDENT_MANIFESTS`]. Any other
/// package that depends on a reporting package, under any spelling of the
/// key, rename and dependency kind, is a finding, and so is a listed
/// dependent that no longer depends on it ([`dependent_findings`]).
pub(crate) const REPORTING_DEPENDENTS: [(&str, &[&str]); 3] =
    [(PACKAGE_CRATE, &[SHELL_PACKAGE, RUNNER_PACKAGE]), (RENDERER_CRATE, &[SHELL_PACKAGE, PDF_CRATE, PACKAGE_CRATE]), (PDF_CRATE, &[PACKAGE_CRATE])];

/// The allowed dependents that are not reporting packages, with their
/// project-relative manifests: the two consumers ([`CONSUMERS`]).
pub(crate) const DEPENDENT_MANIFESTS: [(&str, &str); 2] = [(SHELL_PACKAGE, CONSUMERS[0].1), (RUNNER_PACKAGE, CONSUMERS[1].1)];

/// For each reporting package, by Cargo package name, the packages that
/// depend on it: (package name, canonical manifest).
pub(crate) type Dependents = BTreeMap<String, BTreeSet<(String, PathBuf)>>;

/// The Cargo configuration files admitted in the repository tree, by path
/// from the repository root: none exist today (code review r6 at #1682).
/// Cargo reads `.cargo/config` and `.cargo/config.toml` in the directory it
/// runs in and in every ancestor; one in the tree could add a `paths`
/// override, `[patch]`, `rustflags` (`--cfg test`) or a source replacement.
pub(crate) const CARGO_CONFIGS: [&str; 0] = [];

/// The source of every registry package a consumer may resolve.
const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// The build-configuration findings for the real tree (code review r6 at
/// #1682): any Cargo configuration file in the repository tree
/// ([`cargo_configs`]); a `[patch]` or `[replace]` in a consumer's or
/// reporting crate's manifest ([`manifest_overrides`]); each consumer's
/// reporting packages ([`CONSUMERS`]) against the directories `sources` was
/// read from; and, since code review r8, each reporting package's
/// dependents, from Cargo's metadata of every package in the project and of
/// the path dependencies reachable from them, against the exact allow-list
/// ([`REPORTING_DEPENDENTS`], [`in_repo_packages`]); and, since code review
/// r9, no proc-macro in the allowed dependents' path-dependency closure
/// ([`proc_macro_findings`]). Empty when the configuration holds.
pub(crate) fn build_configuration(sources: &Sources, project: &Path) -> Vec<String> {
    let mut findings = Vec::new();
    let Some(repository) = repository_root(project) else {
        return vec![format!("{}: the repository root for the project is not found", project.display())];
    };
    findings.extend(cargo_configs(&repository));
    let expected = match expected_manifests(sources) {
        Ok(expected) => expected,
        Err(error) => return [findings, vec![error]].concat(),
    };
    let mut manifests: Vec<(String, PathBuf)> = CONSUMERS.iter().map(|(key, manifest, _)| (key.to_string(), project.join(manifest))).collect();
    manifests.extend(expected.iter().map(|(name, manifest)| (name.clone(), manifest.clone())));
    for (label, manifest) in manifests {
        match std::fs::read_to_string(&manifest) {
            Ok(text) => findings.extend(manifest_overrides(&label, &text)),
            Err(error) => findings.push(format!("{label}: {}: {error}", manifest.display())),
        }
    }
    // Code review r8 at #1682: the dependents of each reporting package, as
    // Cargo reads every package in the project and every path dependency
    // reachable from them; the allow-list is exact.
    let allowed = allowed_dependents(&dependent_locations(project, &expected)).unwrap_or_else(|error| {
        findings.push(error);
        Dependents::new()
    });
    let (project_manifests, unreadable) = project_manifests(project);
    findings.extend(unreadable);
    let packages = in_repo_packages(&project_manifests, &expected);
    findings.extend(packages.findings);
    findings.extend(dependent_findings(&packages.dependents, &allowed));
    for (key, manifest, resolution) in CONSUMERS {
        let manifest = project.join(manifest);
        let Some(dir) = manifest.parent() else {
            findings.push(format!("{key}: {} has no directory", manifest.display()));
            continue;
        };
        findings.extend(match resolution {
            Resolution::Graph => resolved_identities(key, dir, &expected, &[repository.clone()], &allowed),
            Resolution::Declared => declared_identities(key, dir, &expected),
        });
    }
    // Code review r7 at #1682: the build scripts of the consumers' in-repo
    // path packages ([`in_repo_build_scripts`]).
    let shell = canonical(&project.join(CONSUMERS[0].1).to_string_lossy());
    let consumers: Vec<PathBuf> = CONSUMERS.iter().map(|(_, manifest, _)| project.join(manifest)).collect();
    match in_repo_build_scripts(&consumers, &expected) {
        Ok(scripts) => findings.extend(build_script_findings(&scripts, &shell)),
        Err(error) => findings.push(error),
    }
    // Code review r9 at #1682: no proc-macro in the path-dependency closure
    // of the allowed dependents, which holds every reporting crate's.
    match path_packages(&consumers) {
        Ok(packages) => findings.extend(proc_macro_findings(&packages)),
        Err(error) => findings.push(error),
    }
    findings
}

/// A finding for each in-repository build script whose package can name a
/// scanned function, other than the shell's (`shell`, its canonical
/// manifest), which [`BUILD_SCRIPTS`] pins by content (code review r7 at
/// #1682).
pub(crate) fn build_script_findings(scripts: &[InRepoBuildScript], shell: &Path) -> Vec<String> {
    scripts
        .iter()
        .filter(|script| script.reaches_reporting && script.manifest != shell)
        .map(|script| {
            format!(
                "the in-repository package {} runs the build script {} and depends on a reporting crate: its cargo:rustc-cfg would apply to code that can name the scanned functions",
                script.package,
                script.script.display()
            )
        })
        .collect()
}

/// One in-repository path package of a consumer's graph that runs a build
/// script ([`in_repo_build_scripts`]).
#[derive(Clone, Debug)]
pub(crate) struct InRepoBuildScript {
    pub(crate) package: String,
    pub(crate) manifest: PathBuf,
    pub(crate) script: PathBuf,
    /// Whether the package is a reporting package or depends on one through
    /// path dependencies, of any kind.
    pub(crate) reaches_reporting: bool,
}

/// The in-repository build scripts of the consumers' graphs (code review r7
/// at #1682; §2.8, "Build scripts"). A build script's `cargo:rustc-cfg`
/// sets a `cfg` (`test` included) for its own package's crates only, so one
/// matters here only when its package can name a scanned function: a
/// reporting package (whose build scripts the target pin refuses), or a
/// package that depends on one. `src-tauri`'s is pinned by content
/// ([`BUILD_SCRIPTS`]); any other such package is a finding
/// ([`build_configuration`]). The walk ([`path_packages`]) follows each
/// manifest's path dependencies, of every kind, by `cargo metadata
/// --format-version 1 --offline --locked --no-deps` per manifest, which needs
/// no download (a registry package cannot depend on a repository path).
/// Every package with a build script is returned, whether or not it reaches
/// a reporting one.
pub(crate) fn in_repo_build_scripts(consumers: &[PathBuf], expected: &BTreeMap<String, PathBuf>) -> Result<Vec<InRepoBuildScript>, String> {
    let reporting: BTreeSet<PathBuf> = expected.values().cloned().collect();
    let packages = path_packages(consumers)?;
    // Which packages reach a reporting package through path dependencies.
    let mut reaches: BTreeSet<PathBuf> = reporting.iter().filter(|manifest| packages.contains_key(*manifest)).cloned().collect();
    loop {
        let before = reaches.len();
        for (manifest, package) in &packages {
            if package.dependencies.iter().any(|dependency| reaches.contains(dependency)) {
                reaches.insert(manifest.clone());
            }
        }
        if reaches.len() == before {
            break;
        }
    }
    Ok(packages
        .iter()
        .filter_map(|(manifest, package)| {
            package.script.as_ref().map(|script| InRepoBuildScript { package: package.name.clone(), manifest: manifest.clone(), script: script.clone(), reaches_reporting: reaches.contains(manifest) })
        })
        .collect())
}

/// One in-repository path package, as Cargo reports it ([`path_packages`]).
#[derive(Clone, Debug)]
pub(crate) struct PathPackage {
    pub(crate) name: String,
    /// Its build script, if Cargo reports one.
    pub(crate) script: Option<PathBuf>,
    /// The names of its `proc-macro` targets (code review r9 at #1682).
    pub(crate) proc_macros: Vec<String>,
    /// Its path dependencies' canonical manifests, of every kind.
    pub(crate) dependencies: Vec<PathBuf>,
}

/// The packages at `consumers` and every path dependency reachable from them,
/// of every kind (normal, `dev`, `build`, any platform), by canonical
/// manifest: each read by `cargo metadata --format-version 1 --offline
/// --locked --no-deps --manifest-path`, run in its directory, which needs no
/// download (a registry package cannot depend on a repository path).
/// Metadata Cargo does not give is an error (a finding).
pub(crate) fn path_packages(consumers: &[PathBuf]) -> Result<BTreeMap<PathBuf, PathPackage>, String> {
    let mut packages: BTreeMap<PathBuf, PathPackage> = BTreeMap::new();
    let mut queue: Vec<PathBuf> = consumers.iter().map(|manifest| canonical(&manifest.to_string_lossy())).collect();
    while let Some(manifest) = queue.pop() {
        if packages.contains_key(&manifest) {
            continue;
        }
        let Some(dir) = manifest.parent() else { return Err(format!("{}: no directory", manifest.display())) };
        let output = std::process::Command::new(cargo())
            .args(["metadata", "--format-version", "1", "--offline", "--locked", "--no-deps", "--manifest-path"])
            .arg(&manifest)
            .current_dir(dir)
            .output()
            .map_err(|error| format!("{}: cargo metadata did not run: {error}", manifest.display()))?;
        if !output.status.success() {
            return Err(format!("{}: cargo metadata failed: {}", manifest.display(), String::from_utf8_lossy(&output.stderr).trim()));
        }
        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|error| format!("{}: cargo metadata output: {error}", manifest.display()))?;
        let package = metadata["packages"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|package| package["manifest_path"].as_str().map(canonical).as_ref() == Some(&manifest))
            .ok_or_else(|| format!("{}: cargo metadata reports no package for it", manifest.display()))?;
        let name = package["name"].as_str().unwrap_or("?").to_string();
        let targets: Vec<&serde_json::Value> = package["targets"].as_array().into_iter().flatten().collect();
        let has_kind = |target: &serde_json::Value, wanted: &str| target["kind"].as_array().into_iter().flatten().any(|kind| kind.as_str() == Some(wanted));
        let script = targets.iter().find(|target| has_kind(target, "custom-build")).and_then(|target| target["src_path"].as_str()).map(canonical);
        let proc_macros = targets.iter().filter(|target| has_kind(target, "proc-macro")).map(|target| target["name"].as_str().unwrap_or("?").to_string()).collect();
        let mut dependencies = Vec::new();
        for dependency in package["dependencies"].as_array().into_iter().flatten() {
            if let Some(path) = dependency["path"].as_str() {
                let next = canonical(&Path::new(path).join("Cargo.toml").to_string_lossy());
                queue.push(next.clone());
                dependencies.push(next);
            }
        }
        packages.insert(manifest, PathPackage { name, script, proc_macros, dependencies });
    }
    Ok(packages)
}

/// A finding for each proc-macro package in the path-dependency closure of
/// the reporting crates and their allowed dependents (`packages`, from
/// [`path_packages`] at the allowed dependents, which reach every reporting
/// crate) that is not on [`PROC_MACRO_PACKAGES`] (code review r9 at #1682;
/// §2.8, "Proc-macros"): by target kind, as Cargo reports it, whatever the
/// dependency's key, rename or kind, and however deep. Its expansion is code
/// the scan never reads: the reviewer's round-9 macro emitted the producer's
/// admitting entry into the runner.
pub(crate) fn proc_macro_findings(packages: &BTreeMap<PathBuf, PathPackage>) -> Vec<String> {
    packages
        .iter()
        .filter(|(_, package)| !package.proc_macros.is_empty() && !PROC_MACRO_PACKAGES.contains(&package.name.as_str()))
        .map(|(manifest, package)| {
            format!(
                "the in-repository package {} ({}) builds the proc-macro target {} and is a path dependency of a reporting crate or an allowed dependent: its expansion is code the scan does not read",
                package.name,
                manifest.display(),
                package.proc_macros.join(", ")
            )
        })
        .collect()
}

/// Each reporting package's expected manifest: `Cargo.toml` in the
/// canonical directory the scan read it from.
pub(crate) fn expected_manifests(sources: &Sources) -> Result<BTreeMap<String, PathBuf>, String> {
    REPORTING_PACKAGES
        .iter()
        .map(|(name, key)| {
            let dir = sources.crate_dirs.get(*key).ok_or_else(|| format!("{key}: the scan read no directory for it"))?;
            Ok((name.to_string(), dir.join("Cargo.toml")))
        })
        .collect()
}

/// Every `.cargo/config` and `.cargo/config.toml` under `root` (code review
/// r6 at #1682), at any depth, `.git` aside and symbolic links to
/// directories not followed, that is not on [`CARGO_CONFIGS`]: each a
/// finding. A directory that cannot be read is a finding too.
pub(crate) fn cargo_configs(root: &Path) -> Vec<String> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
        for name in ["config", "config.toml"] {
            let candidate = dir.join(".cargo").join(name);
            if candidate.symlink_metadata().is_ok() {
                let relative = candidate.strip_prefix(root).map_or_else(|_| candidate.display().to_string(), |path| path.to_string_lossy().replace('\\', "/"));
                if !CARGO_CONFIGS.contains(&relative.as_str()) {
                    out.push(format!("the Cargo configuration {relative} is in the repository tree and not on the allow-list"));
                }
            }
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && dir != root => return,
            Err(error) => {
                out.push(format!("{}: the directory cannot be read for Cargo configuration ({error})", dir.display()));
                return;
            }
        };
        let mut children: Vec<PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).collect();
        children.sort();
        for child in children {
            if child.file_name().is_some_and(|name| name == ".git") {
                continue;
            }
            if child.symlink_metadata().is_ok_and(|metadata| metadata.is_dir()) {
                walk(&child, root, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out
}

/// A manifest's `[patch]` and `[replace]` (code review r6 at #1682), found
/// without a TOML parser and failing closed: any `patch` or `replace`, in any
/// case and anywhere in the text (a table header, a dotted or quoted key, an
/// inline table, a comment), and any `\u` or `\U` escape (which could spell
/// one in a quoted key), is a finding. None of the checked manifests holds
/// either today. An override that takes effect also changes the resolved
/// identities ([`resolved_identities`]).
pub(crate) fn manifest_overrides(label: &str, text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut findings = Vec::new();
    for word in ["patch", "replace"] {
        if lower.contains(word) {
            findings.push(format!("{label}: the manifest names {word}, which could be a [{word}] section"));
        }
    }
    if text.contains("\\u") || text.contains("\\U") {
        findings.push(format!("{label}: the manifest holds a unicode escape, which could spell a [patch] or [replace] key"));
    }
    findings
}

/// The Cargo used to read metadata: the one running this test.
fn cargo() -> std::ffi::OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into())
}

/// The canonical form of a path Cargo reports, or the path itself.
fn canonical(path: &str) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path))
}

/// A consumer's resolved reporting packages (code review r6 at #1682), by
/// `cargo metadata --format-version 1 --offline --locked --filter-platform
/// host-tuple` with dependencies, run in the consumer's directory so that
/// Cargo reads the configuration a build there reads: each name in
/// `expected` resolves to exactly one package, whose canonical manifest is
/// the expected one; the resolve's root is the consumer; and every package
/// comes from crates.io or from a path under one of `roots`. A `paths`
/// override, `[patch]`, `[replace]` or source override that moves a package
/// is a finding, and so is metadata that cannot be read. Since code review
/// r8, every package of the graph that depends on a reporting package must
/// be on its allowed list (`allowed`; [`REPORTING_DEPENDENTS`]).
pub(crate) fn resolved_identities(key: &str, consumer_dir: &Path, expected: &BTreeMap<String, PathBuf>, roots: &[PathBuf], allowed: &Dependents) -> Vec<String> {
    let manifest = consumer_dir.join("Cargo.toml");
    let output = std::process::Command::new(cargo())
        .args(["metadata", "--format-version", "1", "--offline", "--locked", "--filter-platform", "host-tuple", "--manifest-path"])
        .arg(&manifest)
        .current_dir(consumer_dir)
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(output) => return vec![format!("{key}: cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr).trim())],
        Err(error) => return vec![format!("{key}: cargo metadata did not run: {error}")],
    };
    let metadata: serde_json::Value = match serde_json::from_slice(&output.stdout) {
        Ok(metadata) => metadata,
        Err(error) => return vec![format!("{key}: cargo metadata output: {error}")],
    };
    let mut findings = Vec::new();
    let Some(packages) = metadata["packages"].as_array() else { return vec![format!("{key}: cargo metadata reports no packages")] };
    let root_id = metadata["resolve"]["root"].as_str().unwrap_or_default();
    let consumer = canonical(&manifest.to_string_lossy());
    let root_manifest = packages.iter().find(|package| package["id"].as_str() == Some(root_id)).and_then(|package| package["manifest_path"].as_str()).map(canonical);
    if root_manifest.as_ref() != Some(&consumer) {
        findings.push(format!("{key}: the resolve's root is {root_manifest:?}, not {}", consumer.display()));
    }
    let resolved: BTreeSet<&str> = metadata["resolve"]["nodes"].as_array().into_iter().flatten().filter_map(|node| node["id"].as_str()).collect();
    for (name, manifest) in expected {
        let found: Vec<&serde_json::Value> = packages.iter().filter(|package| package["name"].as_str() == Some(name)).collect();
        match found.as_slice() {
            [package] => {
                let at = package["manifest_path"].as_str().map(canonical);
                if at.as_ref() != Some(manifest) {
                    findings.push(format!("{key}: {name} resolves to {}, not {}, the crate the scan read", at.map_or("no manifest".to_string(), |at| at.display().to_string()), manifest.display()));
                }
                if !package["source"].is_null() {
                    findings.push(format!("{key}: {name} resolves from the source {}, not a path", package["source"]));
                }
                if !package["id"].as_str().is_some_and(|id| resolved.contains(id)) {
                    findings.push(format!("{key}: {name} is not in the resolved graph"));
                }
            }
            _ => findings.push(format!("{key}: {name} resolves to {} packages, not exactly one", found.len())),
        }
    }
    for package in packages {
        let name = package["name"].as_str().unwrap_or("?");
        match package["source"].as_str() {
            Some(CRATES_IO) => {}
            Some(other) => findings.push(format!("{key}: the package {name} comes from {other}, not crates.io or a path in the repository")),
            None => {
                let at = package["manifest_path"].as_str().map(canonical);
                if !at.as_ref().is_some_and(|at| roots.iter().any(|root| at.starts_with(root))) {
                    findings.push(format!("{key}: the path package {name} at {at:?} is outside the repository"));
                }
            }
        }
    }
    findings.extend(graph_dependent_findings(key, &metadata, expected, allowed));
    findings
}

/// A consumer's declared reporting dependencies (code review r6 at #1682),
/// by `cargo metadata --format-version 1 --offline --locked --no-deps`:
/// Cargo's reading of the manifest, which needs no download. Each declared
/// dependency named in `expected` is a path dependency on the expected
/// crate's directory, and at least one is declared.
pub(crate) fn declared_identities(key: &str, consumer_dir: &Path, expected: &BTreeMap<String, PathBuf>) -> Vec<String> {
    let manifest = consumer_dir.join("Cargo.toml");
    let output = std::process::Command::new(cargo())
        .args(["metadata", "--format-version", "1", "--offline", "--locked", "--no-deps", "--manifest-path"])
        .arg(&manifest)
        .current_dir(consumer_dir)
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(output) => return vec![format!("{key}: cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr).trim())],
        Err(error) => return vec![format!("{key}: cargo metadata did not run: {error}")],
    };
    let metadata: serde_json::Value = match serde_json::from_slice(&output.stdout) {
        Ok(metadata) => metadata,
        Err(error) => return vec![format!("{key}: cargo metadata output: {error}")],
    };
    let consumer = canonical(&manifest.to_string_lossy());
    let Some(package) = metadata["packages"].as_array().into_iter().flatten().find(|package| package["manifest_path"].as_str().map(canonical).as_ref() == Some(&consumer)) else {
        return vec![format!("{key}: cargo metadata reports no package for {}", consumer.display())];
    };
    let mut findings = Vec::new();
    let mut declared = 0;
    for dependency in package["dependencies"].as_array().into_iter().flatten() {
        let name = dependency["name"].as_str().unwrap_or("?");
        let Some(manifest) = expected.get(name) else { continue };
        declared += 1;
        let at = dependency["path"].as_str().map(|path| canonical(path).join("Cargo.toml"));
        if at.as_ref() != Some(manifest) {
            findings.push(format!("{key}: the dependency {name} is declared at {at:?}, not {}, the crate the scan read", manifest.display()));
        }
    }
    if declared == 0 {
        findings.push(format!("{key}: declares no reporting dependency"));
    }
    findings
}

/// Each allowed dependent's canonical manifest, by package name (code review
/// r8 at #1682): the reporting packages' from `expected`, the others' under
/// `project` ([`DEPENDENT_MANIFESTS`]).
pub(crate) fn dependent_locations(project: &Path, expected: &BTreeMap<String, PathBuf>) -> BTreeMap<String, PathBuf> {
    let mut locations = expected.clone();
    for (name, manifest) in DEPENDENT_MANIFESTS {
        locations.insert(name.to_string(), canonical(&project.join(manifest).to_string_lossy()));
    }
    locations
}

/// The allowed dependents ([`REPORTING_DEPENDENTS`]), each at its manifest
/// in `locations` ([`dependent_locations`]).
pub(crate) fn allowed_dependents(locations: &BTreeMap<String, PathBuf>) -> Result<Dependents, String> {
    REPORTING_DEPENDENTS
        .iter()
        .map(|(reporting, dependents)| {
            let set = dependents
                .iter()
                .map(|name| locations.get(*name).map(|manifest| (name.to_string(), manifest.clone())).ok_or_else(|| format!("the allowed dependent {name} of {reporting} has no manifest")))
                .collect::<Result<BTreeSet<_>, String>>()?;
            Ok((reporting.to_string(), set))
        })
        .collect()
}

/// Every file named `Cargo.toml` (in any case) under `project`, at any depth
/// (code review r8 at #1682), `.git` aside and symbolic links to directories
/// not followed, with a finding for each directory or entry that cannot be
/// read. `target` and `node_modules` are not skipped: a package there is
/// read like any other.
pub(crate) fn project_manifests(project: &Path) -> (Vec<PathBuf>, Vec<String>) {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>, findings: &mut Vec<String>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) => {
                findings.push(format!("{}: the directory cannot be read for Cargo manifests ({error})", dir.display()));
                return;
            }
        };
        let mut children = Vec::new();
        for entry in entries {
            match entry {
                Ok(entry) => children.push(entry.path()),
                Err(error) => findings.push(format!("{}: an entry cannot be read for Cargo manifests ({error})", dir.display())),
            }
        }
        children.sort();
        for child in children {
            let name = child.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or_default();
            if name == ".git" {
                continue;
            }
            match child.symlink_metadata() {
                Ok(metadata) if metadata.is_dir() => walk(&child, out, findings),
                Ok(_) if name.eq_ignore_ascii_case("Cargo.toml") => out.push(child),
                Ok(_) => {}
                Err(error) => findings.push(format!("{}: cannot be read for Cargo manifests ({error})", child.display())),
            }
        }
    }
    let mut manifests = Vec::new();
    let mut findings = Vec::new();
    walk(project, &mut manifests, &mut findings);
    (manifests, findings)
}

/// What Cargo reports for the in-repository packages ([`in_repo_packages`]).
#[derive(Debug, Default)]
pub(crate) struct InRepoPackages {
    /// Each package read, by canonical manifest, with its name.
    pub(crate) packages: BTreeMap<PathBuf, String>,
    /// Each reporting package's dependents among them.
    pub(crate) dependents: Dependents,
    /// Each manifest whose metadata Cargo did not give: a finding.
    pub(crate) findings: Vec<String>,
}

/// The packages of `manifests` and of every path dependency reachable from
/// them, and which of them depend on a reporting package (code review r8 at
/// #1682; §2.8, "Dependents"). Each manifest is read by `cargo metadata
/// --format-version 1 --offline --locked --no-deps --manifest-path`, run in
/// its directory, which needs no download; every package it reports (a
/// workspace's members included) is taken, and each of its dependencies'
/// `path`s is followed. A dependency is on a reporting package when Cargo
/// reports its package name (`dependencies[].name`, the normalized package
/// name whatever the key's quoting, dotting or `rename`) as one, or its path
/// as one's directory; every kind (normal, `dev`, `build`) and every
/// platform `target` counts. Metadata that Cargo does not give, for a
/// manifest that is not a package nor a workspace's root, is a finding,
/// never a skip.
pub(crate) fn in_repo_packages(manifests: &[PathBuf], expected: &BTreeMap<String, PathBuf>) -> InRepoPackages {
    let by_manifest: BTreeMap<&PathBuf, &String> = expected.iter().map(|(name, manifest)| (manifest, name)).collect();
    let mut out = InRepoPackages::default();
    let mut read = BTreeSet::new();
    let mut queue: Vec<PathBuf> = manifests.iter().rev().map(|manifest| canonical(&manifest.to_string_lossy())).collect();
    while let Some(manifest) = queue.pop() {
        if out.packages.contains_key(&manifest) || !read.insert(manifest.clone()) {
            continue;
        }
        let Some(dir) = manifest.parent() else {
            out.findings.push(format!("{}: the manifest has no directory", manifest.display()));
            continue;
        };
        let output = std::process::Command::new(cargo())
            .args(["metadata", "--format-version", "1", "--offline", "--locked", "--no-deps", "--manifest-path"])
            .arg(&manifest)
            .current_dir(dir)
            .output();
        let output = match output {
            Ok(output) if output.status.success() => output,
            Ok(output) => {
                out.findings.push(format!("{}: cargo metadata --offline --locked --no-deps failed, so its dependencies are unknown: {}", manifest.display(), String::from_utf8_lossy(&output.stderr).trim()));
                continue;
            }
            Err(error) => {
                out.findings.push(format!("{}: cargo metadata did not run: {error}", manifest.display()));
                continue;
            }
        };
        let metadata: serde_json::Value = match serde_json::from_slice(&output.stdout) {
            Ok(metadata) => metadata,
            Err(error) => {
                out.findings.push(format!("{}: cargo metadata output: {error}", manifest.display()));
                continue;
            }
        };
        let mut reported = false;
        for package in metadata["packages"].as_array().into_iter().flatten() {
            let Some(at) = package["manifest_path"].as_str().map(canonical) else {
                out.findings.push(format!("{}: cargo metadata reports a package without a manifest", manifest.display()));
                continue;
            };
            reported |= at == manifest;
            if out.packages.contains_key(&at) {
                continue;
            }
            let name = package["name"].as_str().unwrap_or("?").to_string();
            out.packages.insert(at.clone(), name.clone());
            let Some(dependencies) = package["dependencies"].as_array() else {
                out.findings.push(format!("{}: cargo metadata reports {name} without dependencies", at.display()));
                continue;
            };
            for dependency in dependencies {
                let mut reporting = BTreeSet::new();
                let dependency_name = dependency["name"].as_str().unwrap_or("?");
                if expected.contains_key(dependency_name) {
                    reporting.insert(dependency_name.to_string());
                }
                if let Some(path) = dependency["path"].as_str() {
                    let next = canonical(path).join("Cargo.toml");
                    if let Some(name) = by_manifest.get(&next) {
                        reporting.insert(name.to_string());
                    }
                    queue.push(next);
                }
                for reporting in reporting {
                    out.dependents.entry(reporting).or_default().insert((name.clone(), at.clone()));
                }
            }
        }
        let workspace_root = metadata["workspace_root"].as_str().map(canonical);
        if !reported && workspace_root.as_deref() != Some(dir) {
            out.findings.push(format!("{}: cargo metadata reports neither a package for it nor it as a workspace's root", manifest.display()));
        }
    }
    out
}

/// A finding for each package that depends on a reporting package and is
/// not on its allowed list, and for each allowed dependent that does not
/// depend on it (code review r8 at #1682): the lists are exact.
pub(crate) fn dependent_findings(found: &Dependents, allowed: &Dependents) -> Vec<String> {
    let empty = BTreeSet::new();
    let mut findings = Vec::new();
    for reporting in found.keys().chain(allowed.keys()).collect::<BTreeSet<_>>() {
        let found = found.get(reporting).unwrap_or(&empty);
        let allowed = allowed.get(reporting).unwrap_or(&empty);
        for (name, manifest) in found.difference(allowed) {
            findings.push(format!("the package {name} ({}) depends on {reporting} and is not on its allowed dependent list", manifest.display()));
        }
        for (name, manifest) in allowed.difference(found) {
            findings.push(format!("the allowed dependent {name} ({}) of {reporting} does not depend on it, as Cargo reports; the list is exact", manifest.display()));
        }
    }
    findings
}

/// The dependents of the reporting packages in a resolved graph (code
/// review r8 at #1682), by `resolve.nodes[].dependencies` and `deps[].pkg`:
/// a finding for each package, from any source, that depends on one and is
/// not on its allowed list.
fn graph_dependent_findings(key: &str, metadata: &serde_json::Value, expected: &BTreeMap<String, PathBuf>, allowed: &Dependents) -> Vec<String> {
    let packages: BTreeMap<&str, &serde_json::Value> = metadata["packages"].as_array().into_iter().flatten().filter_map(|package| package["id"].as_str().map(|id| (id, package))).collect();
    let manifest_of = |package: &serde_json::Value| package["manifest_path"].as_str().map(canonical);
    let reporting: BTreeMap<&str, String> = packages
        .iter()
        .filter_map(|(id, package)| {
            let name = package["name"].as_str().unwrap_or("?");
            if expected.contains_key(name) {
                return Some((*id, name.to_string()));
            }
            let at = manifest_of(package)?;
            expected.iter().find(|(_, manifest)| **manifest == at).map(|(name, _)| (*id, name.clone()))
        })
        .collect();
    let empty = BTreeSet::new();
    let mut findings = BTreeSet::new();
    for node in metadata["resolve"]["nodes"].as_array().into_iter().flatten() {
        let id = node["id"].as_str().unwrap_or("?");
        let mut on: BTreeSet<&str> = node["dependencies"].as_array().into_iter().flatten().filter_map(|dependency| dependency.as_str()).collect();
        on.extend(node["deps"].as_array().into_iter().flatten().filter_map(|dependency| dependency["pkg"].as_str()));
        for dependency in on {
            let Some(reporting) = reporting.get(dependency) else { continue };
            let package = packages.get(id);
            let name = package.and_then(|package| package["name"].as_str()).unwrap_or(id).to_string();
            let at = package.and_then(|package| manifest_of(package)).unwrap_or_default();
            if !allowed.get(reporting).unwrap_or(&empty).contains(&(name.clone(), at.clone())) {
                findings.insert(format!("{key}: in the resolved graph, the package {name} ({}) depends on {reporting} and is not on its allowed dependent list", at.display()));
            }
        }
    }
    findings.into_iter().collect()
}
