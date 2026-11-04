//! RT-A4 §10.4 (DEL-05-02 Design `absolute-sum-srss-combinations.md`, "Recorded results
//! from before RT-A4 (historical read)"): the closed registry of the semantic tables a
//! result recorded before RT-A4 binds, and the one scoped way a reader reads under them.
//!
//! RT-A4 restated the preview-physics combination policy and limitation 6, so the
//! preview-physics tables' digests moved; a preview-physics statement carries its
//! table's limitation array (`formulation_basis.limitations`) and the readers require it
//! to equal the table's, so a statement published before RT-A4 no longer reads under the
//! current tables. Each variant here is the table's bytes as the main line held them
//! (`fixtures/results/recorded_pre_rt_a4/`, verbatim), its SHA-256 checked when it is
//! first read; a variant names its identity and, for the preview successor, the
//! preview-physics-2 variant its base inherits.
//!
//! Selection is exact and scoped. A variant is selected only by a recorded binding
//! `{id, sha256}` (exactly those two members) equal to a registry member whose identity
//! is the statement's own, and only inside [`read_recorded`], which the recorded-run
//! read entries alone call (`reader_verdict::*_recorded`, the runner's and the wasm
//! engine's `recorded_binding` input). Every other entry reads the current tables only:
//! no current admission path (producer output checks, record building, current
//! dispatch, standing or export) ever selects a variant, and a statement carrying a
//! variant's limitations is refused there as before. Selection grants no standing.
//!
//! The selection is this thread's, held only for the synchronous call of a recorded read
//! and restored when that call ends, however it ends: a returned value or error, a refusal
//! (nothing is selected), or an unwinding panic (the `Restore` guard's drop). A current
//! binding nested in a recorded read clears it for its read; a recorded read nested in
//! another restores the outer one. No caller holds it across an async yield: every caller
//! runs the read to completion on its own thread (the wasm engine's exports, the runner's
//! `read` command, and the desktop's native custody commands, which make each recorded read synchronously inside `entry_scope` on their worker thread).
//!
//! Under `panic = abort` a panic does not unwind, so the guard does not run. Natively that
//! ends the process, and the selection with it. The wasm32 target aborts by default, and a
//! trap leaves the instance usable with this (single-threaded) selection still set, so every
//! outermost entry runs inside [`entry_scope`], which clears whatever it finds: the wasm
//! engine's every export (`operation_applier`'s `wasm_api`) and the runner's `read` command.
use serde_json::Value;
use std::cell::Cell;
use std::sync::OnceLock;

/// The recorded binding names no registry variant (and is not the current table's).
pub const SOURCE_RECORDED_BINDING_UNKNOWN: &str = "SOURCE_RECORDED_BINDING_UNKNOWN";
/// The recorded binding's identity is not the statement's.
pub const SOURCE_RECORDED_BINDING_IDENTITY: &str = "SOURCE_RECORDED_BINDING_IDENTITY";

/// One historical table variant.
pub struct RecordedVariant {
    pub id: &'static str,
    pub sha256: &'static str,
    profile: &'static str,
    bytes: &'static [u8],
    /// The registry index of the preview-physics-2 variant this successor's base inherits.
    base: Option<usize>,
    /// Whether a statement bound to this variant is read under the variant's tables. The
    /// exact successor's statements read under the current reader (their rows and
    /// receipt bindings are unchanged; only the vocabulary and inherited pressure-2 digest
    /// moved), so its variant changes no reader table: it is a recorded binding only.
    reads_under_variant: bool,
}

const PREVIEW: &str = crate::semantic_contract::PREVIEW_PHYSICS_ID;
const TIMOSHENKO: &str = crate::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID;
const RETAINED: &str = crate::retained_precision::CONTRACT_ID;
const EXACT_RETAINED: &str = crate::retained_precision::EXACT_CONTRACT_ID;

/// The closed registry. Main at 6c3cb46d3 held the first four; the fifth is RT-A3's
/// final constant-effort candidate's exact successor table (#101 at 01e05a486), which
/// main holds once that lands before RT-A4; the sixth is the RT-A4 main line's exact
/// successor table (main at bbf384dec), whose inherited pressure-2 digest RT-A3
/// increment 4 moves.
pub static VARIANTS: [RecordedVariant; 6] = [
    RecordedVariant {
        id: PREVIEW,
        sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2",
        profile: crate::semantic_contract::PREVIEW_PHYSICS_PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_2.1ccc9561.json"),
        base: None,
        reads_under_variant: true,
    },
    RecordedVariant {
        id: TIMOSHENKO,
        sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b",
        profile: crate::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_timoshenko_1.a7b83c37.json"),
        base: None,
        reads_under_variant: true,
    },
    RecordedVariant {
        id: RETAINED,
        sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46",
        profile: crate::retained_precision::PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_retained_1.b001070e.json"),
        base: Some(0),
        reads_under_variant: true,
    },
    RecordedVariant {
        id: EXACT_RETAINED,
        sha256: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd",
        profile: crate::retained_precision::EXACT_PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.cf29fd5f.json"),
        base: None,
        reads_under_variant: false,
    },
    RecordedVariant {
        id: EXACT_RETAINED,
        sha256: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b",
        profile: crate::retained_precision::EXACT_PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.045f6f0d.json"),
        base: None,
        reads_under_variant: false,
    },    RecordedVariant {
        id: EXACT_RETAINED,
        sha256: "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034",
        profile: crate::retained_precision::EXACT_PROFILE,
        bytes: include_bytes!("../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.040ef9a6.json"),
        base: None,
        reads_under_variant: false,
    },
];

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

/// A variant's bytes checked: their SHA-256 is the registry's, and the table names the
/// variant's identity and profile (and a successor's inherited digest its base's).
pub fn verify(variant: &RecordedVariant, bytes: &[u8]) -> Result<Value, String> {
    if sha256_hex(bytes) != variant.sha256 {
        return Err("SOURCE_RECORDED_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes).map_err(|_| "SOURCE_RECORDED_TABLE_HASH".to_string())?;
    let inherited_ok = variant.base.is_none_or(|b| table["inherited_semantic_contract_sha256"] == VARIANTS[b].sha256);
    if table["semantic_contract_id"] != variant.id || table["formulation_profile_id"] != variant.profile || !inherited_ok {
        return Err("SOURCE_RECORDED_TABLE_IDENTITY".into());
    }
    Ok(table)
}

impl RecordedVariant {
    fn index(&self) -> usize {
        VARIANTS.iter().position(|v| std::ptr::eq(v, self)).expect("a registry member")
    }
    /// The variant's table, its bytes checked on first read.
    pub fn table(&'static self) -> &'static Value {
        static TABLES: [OnceLock<Value>; 6] = [const { OnceLock::new() }; 6];
        TABLES[self.index()].get_or_init(|| verify(self, self.bytes).expect("a registry table's pinned bytes"))
    }
    /// The base variant a successor's base inherits (preview-physics-2's), if any.
    pub fn base(&self) -> Option<&'static RecordedVariant> {
        self.base.map(|b| &VARIANTS[b])
    }
    pub fn reads_under_variant(&self) -> bool {
        self.reads_under_variant
    }
}

/// The registry variant a recorded binding names: an object of exactly `id` and
/// `sha256`, equal to a member's. Anything else names none.
pub fn variant_of(binding: &Value) -> Option<&'static RecordedVariant> {
    let object = binding.as_object().filter(|o| o.len() == 2)?;
    let (id, sha) = (object.get("id")?.as_str()?, object.get("sha256")?.as_str()?);
    VARIANTS.iter().find(|v| v.id == id && v.sha256 == sha)
}

thread_local! {
    static SELECTED: Cell<Option<&'static RecordedVariant>> = const { Cell::new(None) };
}
/// The variant selected for this read, if any (only inside [`read_recorded`]).
pub(crate) fn selected() -> Option<&'static RecordedVariant> {
    SELECTED.with(Cell::get)
}
/// The table a reader reads for `id` inside a recorded read: the selected variant's own
/// table, or, for preview-physics-2, the base a selected successor inherits. `None`
/// outside a recorded read, for a binding-only variant, or for any other identity: the
/// reader then reads its current table.
pub(crate) fn selected_table(id: &str) -> Option<&'static Value> {
    let variant = selected().filter(|v| v.reads_under_variant)?;
    if variant.id == id {
        return Some(variant.table());
    }
    variant.base().filter(|b| b.id == id).map(RecordedVariant::table)
}
/// The binding the selected variant gives `id` (a reader's `binding_of`), if any.
pub(crate) fn selected_binding(id: &str) -> Option<&'static str> {
    selected().filter(|v| v.id == id).map(|v| v.sha256)
}

struct Restore(Option<&'static RecordedVariant>);
impl Drop for Restore {
    fn drop(&mut self) {
        SELECTED.with(|s| s.set(self.0));
    }
}

/// Run `read` as an outermost entry (a wasm export, the runner's `read` command): with no
/// variant selected, and none selected after, whatever this thread held on entry. Nothing is
/// legitimately selected at an outermost entry; a selection found there was left by a recorded
/// read that trapped under `panic = abort` (the wasm32 target's strategy), whose `Restore`
/// guard never ran, and it must not reach a current read. Only outermost entries call this: a
/// call nested in a recorded read would clear that read's selection.
pub fn entry_scope<R>(read: impl FnOnce() -> R) -> R {
    let _clear = Restore(None);
    SELECTED.with(|s| s.set(None));
    read()
}

/// Run `read` as the recorded-run read of `source` under its recorded `binding`.
///
/// A binding equal to the statement's current table binding (`current`, the reader's
/// `{id, sha256}` for the statement's identity) reads as the current read does. A binding
/// naming a registry variant whose identity is the statement's
/// (`producer.semantic_contract_id`) selects that variant for `read` and nothing after it.
/// Any other binding is refused by name (`SOURCE_RECORDED_BINDING_UNKNOWN`, or
/// `SOURCE_RECORDED_BINDING_IDENTITY` for a variant of another identity), and `read`
/// does not run.
///
/// `read` runs synchronously inside this call, and the thread's prior selection is restored
/// when it ends: on return (an `Err` it returns included) and on unwind. Callers never hold
/// the selection across an async yield (module documentation).
pub fn read_recorded<R>(binding: &Value, source: &Value, current: Option<(&str, &str)>, read: impl FnOnce() -> R) -> Result<R, String> {
    let identity = source["producer"]["semantic_contract_id"].as_str();
    let is_current = current.is_some_and(|(id, sha)| {
        binding.as_object().is_some_and(|o| o.len() == 2) && binding["id"] == id && binding["sha256"] == sha
    });
    if is_current {
        // The current binding reads the current table even inside another recorded read
        // (review r5): the enclosing selection is cleared for `read` and restored after.
        let _restore = Restore(selected());
        SELECTED.with(|s| s.set(None));
        return Ok(read());
    }
    let variant = variant_of(binding).ok_or_else(|| SOURCE_RECORDED_BINDING_UNKNOWN.to_string())?;
    if identity != Some(variant.id) {
        return Err(SOURCE_RECORDED_BINDING_IDENTITY.into());
    }
    let _restore = Restore(selected());
    SELECTED.with(|s| s.set(Some(variant)));
    Ok(read())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn every_variant_is_its_pinned_bytes_and_none_is_current() {
        for variant in &VARIANTS {
            let table = variant.table();
            assert_eq!(table["semantic_contract_id"], variant.id);
            let mut tampered = variant.bytes.to_vec();
            tampered.push(b' ');
            assert_eq!(verify(variant, &tampered).unwrap_err(), "SOURCE_RECORDED_TABLE_HASH");
            let current = crate::reader_verdict::current_binding(variant.id).expect("a current identity");
            assert_ne!(current, variant.sha256, "{}: a variant is never the current table", variant.id);
        }
        // Rows are unchanged from each current table: only limitations, policy text,
        // vocabulary and inherited digests moved.
        assert_eq!(VARIANTS[0].table()["rows"], crate::semantic_contract::preview_physics_contract()["rows"]);
        assert_eq!(VARIANTS[1].table()["rows"], crate::semantic_contract::preview_physics_timoshenko_contract()["rows"]);
        assert_eq!(VARIANTS[2].table()["rows"], crate::semantic_contract::preview_physics_retained_contract()["rows"]);
        assert_eq!(VARIANTS[3].table()["rows"], crate::semantic_contract::physics_retained_contract()["rows"]);
        assert_eq!(VARIANTS[4].table()["rows"], crate::semantic_contract::physics_retained_contract()["rows"]);
        assert_eq!(VARIANTS[5].table()["rows"], crate::semantic_contract::physics_retained_contract()["rows"]);
        assert!(VARIANTS[0].table()["supported_profile_limitations"] != crate::semantic_contract::preview_physics_contract()["supported_profile_limitations"]);
    }

    #[test]
    fn a_current_binding_nested_in_a_recorded_read_reads_the_current_table() {
        let source = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let old = json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256});
        let sha = crate::reader_verdict::current_binding(PREVIEW).expect("a current identity");
        let current = json!({"id": PREVIEW, "sha256": sha});
        let (outer, inner, after) = read_recorded(&old, &source, Some((PREVIEW, sha)), || {
            let outer = selected().map(|v| v.sha256);
            let inner = read_recorded(&current, &source, Some((PREVIEW, sha)), || selected().map(|v| v.sha256)).unwrap();
            (outer, inner, selected().map(|v| v.sha256))
        })
        .unwrap();
        assert_eq!((outer, inner, after), (Some(VARIANTS[0].sha256), None, Some(VARIANTS[0].sha256)));
        assert!(selected().is_none());
    }

    fn sha(selection: Option<&'static RecordedVariant>) -> Option<&'static str> {
        selection.map(|v| v.sha256)
    }

    #[test]
    fn a_read_that_returns_an_error_or_is_refused_restores_the_prior_selection() {
        let source = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let good = json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256});
        let inside = read_recorded(&good, &source, None, || -> Result<(), String> {
            assert_eq!(sha(selected()), Some(VARIANTS[0].sha256));
            Err("READ_FAILED".into())
        });
        assert_eq!(inside, Ok(Err("READ_FAILED".to_string())));
        assert!(selected().is_none());
        // Inside an outer read: an inner error and an inner refusal leave the outer selection.
        let timoshenko = json!({"producer": {"semantic_contract_id": TIMOSHENKO}});
        let outer = json!({"id": TIMOSHENKO, "sha256": VARIANTS[1].sha256});
        read_recorded(&outer, &timoshenko, None, || {
            let _ = read_recorded(&good, &source, None, || -> Result<(), ()> { Err(()) });
            assert_eq!(sha(selected()), Some(VARIANTS[1].sha256));
            let refused = read_recorded(&json!({"id": PREVIEW, "sha256": "0".repeat(64)}), &source, None, || ());
            assert_eq!(refused.unwrap_err(), SOURCE_RECORDED_BINDING_UNKNOWN);
            assert_eq!(sha(selected()), Some(VARIANTS[1].sha256));
        })
        .unwrap();
        assert!(selected().is_none());
    }

    #[test]
    fn a_read_that_panics_restores_the_prior_selection_on_unwind() {
        let source = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let good = json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256});
        let unwound = std::panic::catch_unwind(|| read_recorded(&good, &source, None, || panic!("a reader panicked")));
        assert!(unwound.is_err());
        assert!(selected().is_none());
        let timoshenko = json!({"producer": {"semantic_contract_id": TIMOSHENKO}});
        let outer = json!({"id": TIMOSHENKO, "sha256": VARIANTS[1].sha256});
        read_recorded(&outer, &timoshenko, None, || {
            let unwound = std::panic::catch_unwind(|| read_recorded(&good, &source, None, || panic!("a nested reader panicked")));
            assert!(unwound.is_err());
            assert_eq!(sha(selected()), Some(VARIANTS[1].sha256));
        })
        .unwrap();
        assert!(selected().is_none());
    }

    #[test]
    fn a_recorded_read_nested_in_another_restores_the_outer_selection() {
        let preview = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let timoshenko = json!({"producer": {"semantic_contract_id": TIMOSHENKO}});
        let (outer, inner) = (json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256}), json!({"id": TIMOSHENKO, "sha256": VARIANTS[1].sha256}));
        let seen = read_recorded(&outer, &preview, None, || {
            let before = sha(selected());
            let during = read_recorded(&inner, &timoshenko, None, || sha(selected())).unwrap();
            (before, during, sha(selected()))
        })
        .unwrap();
        assert_eq!(seen, (Some(VARIANTS[0].sha256), Some(VARIANTS[1].sha256), Some(VARIANTS[0].sha256)));
        assert!(selected().is_none());
    }

    /// A selection left set without its guard (as a recorded read that traps under
    /// `panic = abort` leaves it) does not reach a current read made through an outermost
    /// entry, and is cleared by it. Without `entry_scope` the leaked variant would be read.
    #[test]
    fn a_leaked_selection_does_not_reach_a_current_read_through_an_outermost_entry() {
        let current: Value = serde_json::from_str(include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")).unwrap();
        let read = || crate::reader_verdict::validate("preview-physics", &current, None, None);
        assert_eq!(read()["ok"], true);
        SELECTED.with(|s| s.set(Some(&VARIANTS[0])));
        // The gap the guard closes: the leaked variant reaches an unguarded current read.
        let leaked = read();
        assert_eq!(leaked["ok"], false, "{leaked}");
        assert_eq!(leaked["error"]["code"], "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
        let guarded = entry_scope(|| (sha(selected()), read()));
        assert_eq!(guarded.0, None);
        assert_eq!(guarded.1["ok"], true, "{}", guarded.1);
        assert!(selected().is_none(), "an outermost entry leaves no selection behind");
        assert_eq!(read()["ok"], true);
        // A recorded read made through an outermost entry still selects its own variant.
        let source = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let good = json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256});
        SELECTED.with(|s| s.set(Some(&VARIANTS[1])));
        assert_eq!(entry_scope(|| read_recorded(&good, &source, None, || sha(selected()))), Ok(Some(VARIANTS[0].sha256)));
        assert!(selected().is_none());
    }

    #[test]
    fn a_binding_selects_only_an_exact_member_of_the_statements_identity() {
        let source = json!({"producer": {"semantic_contract_id": PREVIEW}});
        let good = json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256});
        assert!(variant_of(&good).is_some());
        for bad in [
            json!({"id": PREVIEW, "sha256": "0".repeat(64)}),
            json!({"id": PREVIEW, "sha256": VARIANTS[1].sha256}),
            json!({"id": PREVIEW, "sha256": VARIANTS[0].sha256, "signature_id": null}),
            json!({"id": PREVIEW}),
            json!(VARIANTS[0].sha256),
            Value::Null,
        ] {
            assert!(variant_of(&bad).is_none(), "{bad}");
            assert_eq!(read_recorded(&bad, &source, None, || ()).unwrap_err(), SOURCE_RECORDED_BINDING_UNKNOWN);
        }
        let timoshenko = json!({"id": TIMOSHENKO, "sha256": VARIANTS[1].sha256});
        assert_eq!(read_recorded(&timoshenko, &source, None, || ()).unwrap_err(), SOURCE_RECORDED_BINDING_IDENTITY);
        // Inside, the variant is selected; after (and outside), nothing is.
        assert!(selected().is_none());
        let inside = read_recorded(&good, &source, None, || selected_table(PREVIEW).map(|t| t["supported_profile_limitations"].clone())).unwrap();
        assert_eq!(inside.as_ref(), Some(&VARIANTS[0].table()["supported_profile_limitations"]));
        assert!(selected().is_none() && selected_table(PREVIEW).is_none());
        // A successor's base is its inherited preview-physics-2 variant.
        let successor = json!({"producer": {"semantic_contract_id": RETAINED}});
        let binding = json!({"id": RETAINED, "sha256": VARIANTS[2].sha256});
        let base = read_recorded(&binding, &successor, None, || selected_table(PREVIEW).map(|t| t as *const Value)).unwrap();
        assert_eq!(base, Some(VARIANTS[0].table() as *const Value));
        // A binding-only variant selects no reader table.
        let exact = json!({"producer": {"semantic_contract_id": EXACT_RETAINED}});
        let binding = json!({"id": EXACT_RETAINED, "sha256": VARIANTS[3].sha256});
        assert_eq!(read_recorded(&binding, &exact, None, || selected_table(EXACT_RETAINED).is_none()), Ok(true));
    }
}
