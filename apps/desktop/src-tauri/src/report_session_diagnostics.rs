//! The session's Diagnostics entries in an exact-route or pressure-free
//! Timoshenko report, bound by the host (review of f3d02469c, finding 2).
//!
//! A bound report's `report_sections.diagnostics` opens with the projection
//! of the delivered result's diagnostics (`check_report_diagnostics`). The
//! entries after them are the session's, and no caller-supplied member (a
//! class, a reference type, a remediation text, an origin tag) can establish
//! that an entry is the session's: the host derives every entry the session
//! may append from what it holds, and admits an appended entry only when it
//! equals one of them on its code, class, severity, source, affected object,
//! message and remediation (the provenance is the session record, unbound).
//! An appended entry with a result diagnostic's code and affected object is
//! refused first, whatever its prose. The kinds, mirroring the desktop
//! builder (`apps/desktop/src/features/report/renderableReportInput.ts`
//! `buildRenderableReportInput`), each derived from the bound intake model:
//! - `COMPONENT_PROVENANCE_MISSING` for each component without provenance
//!   (`componentProvenanceDiagnostics`);
//! - `SPRING_HANGER_PROVENANCE_MISSING` for each spring-hanger or
//!   constant-effort support without `provenance` or `hanger.source_reference`
//!   (`springHangerDiagnostics`, `springHangerSupport.ts`);
//! - `MATERIAL_PROVENANCE_MISSING` and `SECTION_PROVENANCE_MISSING` for each
//!   material and section without provenance (`subjectProvenance`);
//! - a library store finding (`libraryFindingDiagnostics`):
//!   `MODEL-LIBRARY-REF-UNRESOLVED` or `MODEL-LIBRARY-REF-VERSION-CHANGED` on
//!   an entity that carries a `library_ref` in the bound model, at most one
//!   per reference site, among the store check's findings that reference's
//!   shape allows (`lib.rs` `library_ref_findings`; [`for_each_site`]).
//!   Whether the store holds the library is the one fact the host does not
//!   re-check: it reads no store here.
//!
//! Each site yields at most one entry, so a repeated entry is admitted only
//! as many times as the model's derivation yields it (counts, not
//! deduplication: two reference sites can yield identical findings), and a
//! site that may yield one of several findings is used once ([`place`]).
//!
//! Bounded by construction (review round 3 of c290410c6): no derived entry is
//! kept. Each site's entries are written into buffers the next site reuses and
//! looked up among the report's own appended entries, so the check holds the
//! report's entries' keys, a few counters per distinct key, and buffers no
//! longer than the longest derived text; its work is a constant per site plus
//! the report's entries times the logarithm of their number.
//!
//! Applied by the existing render command's admission
//! (`report_render_exact::bind_render_admission`, on both bound identities)
//! and by the local-private package save's admission
//! (`report_package_save::admit_both_routes`, on exact and pressure-free
//! Timoshenko documents).
//! Standard claim fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_report_package::bindings::expected_report_diagnostics;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

/// The library store check's diagnostic source (desktop
/// `features/library/libraryRefFindings.ts` `LIBRARY_REF_CHECK_SOURCE`).
pub(crate) const LIBRARY_REF_CHECK_SOURCE: &str = "apps/desktop/src-tauri/src/lib.rs check_model_library_refs";
/// The name of the one refusal that is a limit of this check, not a fault of
/// the report ([`place`]).
pub(crate) const SESSION_CHECK_LIMIT: &str = "SESSION-DIAGNOSTICS-CHECK-LIMIT";
const PROVENANCE_WARNING: &str = "PROVENANCE_WARNING";
const WARNING: &str = "warning";
const UNRESOLVED: &str = "MODEL-LIBRARY-REF-UNRESOLVED";
const VERSION_CHANGED: &str = "MODEL-LIBRARY-REF-VERSION-CHANGED";
const LIBRARY_REMEDIATION: &str = "Compare the library reference with the local library store before relying on this report.";

/// JavaScript's `String.prototype.trim` (the builder's): WhiteSpace and
/// LineTerminator, so U+FEFF is removed and U+0085 kept, unlike Rust's.
fn js_trim(text: &str) -> &str {
    let space = |c: char| {
        matches!(c, '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
    };
    text.trim_matches(space)
}

/// The builder's `text`: a string non-empty after trimming, trimmed.
fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).map(js_trim).filter(|text| !text.is_empty())
}

/// The builder's `hasProvenance`: `provenance_record.source_name`, or a
/// non-empty `provenance` note (a string after trimming, or an object with a
/// member).
fn has_provenance(entity: &Value) -> bool {
    if text(entity.get("provenance_record").filter(|record| record.is_object()).and_then(|record| record.get("source_name"))).is_some() {
        return true;
    }
    match entity.get("provenance") {
        Some(Value::String(note)) => !js_trim(note).is_empty(),
        Some(Value::Object(note)) => !note.is_empty(),
        _ => false,
    }
}

/// `springHangerSupport.ts` `isSpringHangerSupport` (Rust trimming, as there).
fn is_spring_hanger(support: &Value) -> bool {
    let explicit = support.pointer("/hanger/hanger_type").and_then(Value::as_str).map(str::trim).unwrap_or("");
    let kind = if explicit.is_empty() { support.get("family").and_then(Value::as_str).map(str::trim) } else { Some(explicit) };
    matches!(kind, Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support"))
}

/// The JavaScript `a || b` of the builder on optional strings.
fn or_id<'a>(label: Option<&'a Value>, id: &'a str) -> &'a str {
    label.and_then(Value::as_str).filter(|label| !label.is_empty()).unwrap_or(id)
}

fn items<'a>(model: &'a Value, member: &str) -> impl Iterator<Item = &'a Value> {
    model.get(member).and_then(Value::as_array).into_iter().flatten()
}

fn id(entity: &Value) -> &str {
    entity.get("id").and_then(Value::as_str).unwrap_or("")
}

/// An entry's seven bound fields, borrowed: code, class, severity, source and
/// affected object (`{ref_type, ref_id}`, exactly those two members), message
/// and remediation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Fields<'a> {
    code: &'a str,
    class: &'a str,
    severity: &'a str,
    source: (&'a str, &'a str),
    affected: (&'a str, &'a str),
    message: &'a str,
    remediation: &'a str,
}

/// A `{ref_type, ref_id}` object with string members and no other member.
fn reference_of(value: Option<&Value>) -> Option<(&str, &str)> {
    let object = value?.as_object()?;
    if object.len() != 2 {
        return None;
    }
    Some((object.get("ref_type")?.as_str()?, object.get("ref_id")?.as_str()?))
}

impl<'a> Fields<'a> {
    /// `entry`'s bound fields, or `None` when one of them is not of a session
    /// entry's form, so that no derived entry can equal it.
    fn of(entry: &'a Value) -> Option<Self> {
        let string = |field: &str| entry.get(field).and_then(Value::as_str);
        Some(Fields {
            code: string("code")?,
            class: string("class")?,
            severity: string("severity")?,
            source: reference_of(entry.get("source"))?,
            affected: reference_of(entry.get("affected_object"))?,
            message: string("message")?,
            remediation: string("remediation")?,
        })
    }

    /// A session entry: every kind is a provenance warning.
    fn session(code: &'a str, source: (&'a str, &'a str), affected: (&'a str, &'a str), message: &'a str, remediation: &'a str) -> Self {
        Fields { code, class: PROVENANCE_WARNING, severity: WARNING, source, affected, message, remediation }
    }
}

/// The model's `library_ref` sites in the builder's walk
/// (`libraryImportService.ts` `libraryRefSites`): `emit(entity, reference)`
/// with the owning entity's id, or the site's path when no enclosing object
/// has one. `path` is one buffer, extended and truncated in place.
fn library_ref_sites<'m>(value: &'m Value, owner: Option<&'m str>, path: &mut String, emit: &mut dyn FnMut(&str, &'m Map<String, Value>)) {
    match value {
        Value::Array(entries) => {
            for (index, entry) in entries.iter().enumerate() {
                let length = path.len();
                let _ = write!(path, "[{index}]");
                library_ref_sites(entry, owner, path, emit);
                path.truncate(length);
            }
        }
        Value::Object(object) => {
            let named = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).or(owner);
            for (key, child) in object {
                let length = path.len();
                if length > 0 {
                    path.push('.');
                }
                path.push_str(key);
                if key == "library_ref" {
                    if let Value::Object(reference) = child {
                        match named {
                            Some(entity) => emit(entity, reference),
                            None => emit(path, reference),
                        }
                    }
                }
                library_ref_sites(child, named, path, emit);
                path.truncate(length);
            }
        }
        _ => {}
    }
}

/// The library kinds the store holds (`lib.rs` `library_records_key`): a
/// reference of any other kind can only be unresolved.
const STORED_LIBRARY_KINDS: [&str; 4] = ["material", "section", "component", "hanger"];

/// Visits every site at which the session may append a Diagnostics entry for
/// `model`, with the entries it may yield there: at most one of them, once
/// (review of c2cc050ea, R3 and R4). A provenance kind's site yields exactly
/// its one entry. A library reference's site yields at most one store finding,
/// among those the store check (`lib.rs` `library_ref_findings`) can produce
/// for that reference's shape:
/// - the library not held (`MODEL-LIBRARY-REF-UNRESOLVED`), for any reference;
/// - the record not held (`MODEL-LIBRARY-REF-UNRESOLVED`), only when the store
///   can hold the library: a stored kind ([`STORED_LIBRARY_KINDS`], trimmed)
///   and a library id not blank after trimming;
/// - the stored record changed (`MODEL-LIBRARY-REF-VERSION-CHANGED`), only
///   when, in addition, the reference records a `record_id` not blank after
///   trimming (`library_identity`) and a `library_version`: the store check
///   returns the record-not-held finding before it compares any version.
///
/// The entries passed to `visit` borrow the model and three message buffers
/// that the next site overwrites (review round 3 of c290410c6: no copy of a
/// shared string, such as the project id, is kept per alternative).
fn for_each_site(model: &Value, visit: &mut dyn FnMut(&[Fields<'_>])) {
    let mut messages: [String; 3] = Default::default();
    let mut remediation = String::new();
    for component in items(model, "components").filter(|component| !has_provenance(component)) {
        let id = id(component);
        let message = &mut messages[0];
        message.clear();
        let _ = write!(
            message,
            "Component {} has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component.",
            or_id(component.get("label"), id)
        );
        visit(&[Fields::session(
            "COMPONENT_PROVENANCE_MISSING",
            ("component", id),
            ("component", id),
            message,
            "Enter component source/provenance before relying on this report.",
        )]);
    }
    for support in items(model, "supports").filter(|support| is_spring_hanger(support)) {
        let blank = |value: Option<&Value>| value.and_then(Value::as_str).is_none_or(|text| js_trim(text).is_empty());
        if blank(support.get("provenance")) || blank(support.pointer("/hanger/source_reference")) {
            let id = id(support);
            let message = &mut messages[0];
            message.clear();
            let _ = write!(message, "Spring hanger {} is missing support provenance or hanger.source_reference.", or_id(support.get("label"), id));
            visit(&[Fields::session(
                "SPRING_HANGER_PROVENANCE_MISSING",
                ("support", id),
                ("support", id),
                message,
                "Enter spring-hanger source/provenance before relying on this report.",
            )]);
        }
    }
    for (kind, member, label, code, title) in
        [("material", "materials", "label", "MATERIAL_PROVENANCE_MISSING", "Material"), ("section", "sections", "name", "SECTION_PROVENANCE_MISSING", "Section")]
    {
        remediation.clear();
        let _ = write!(remediation, "Enter {kind} source/provenance before relying on this report.");
        for subject in items(model, member).filter(|subject| !has_provenance(subject)) {
            let id = id(subject);
            let message = &mut messages[0];
            message.clear();
            let _ = write!(
                message,
                "{title} {} has neither a provenance_record nor a provenance note; report provenance is incomplete for this {kind}.",
                text(subject.get(label)).unwrap_or(id)
            );
            visit(&[Fields::session(code, (kind, id), (kind, id), message, &remediation)]);
        }
    }
    let project = model.pointer("/project/id").and_then(Value::as_str).unwrap_or("").trim();
    let mut path = String::new();
    library_ref_sites(model, None, &mut path, &mut |entity, site| {
        let member = |key: &str| site.get(key).and_then(Value::as_str);
        let (kind, library) = (member("library_kind").unwrap_or(""), member("library_id").unwrap_or(""));
        let [not_held, record_not_held, changed] = &mut messages;
        not_held.clear();
        let _ = write!(
            not_held,
            "{entity} names {kind} library `{library}`, which the local store does not hold for project `{project}`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."
        );
        let mut yielded = 1;
        if STORED_LIBRARY_KINDS.contains(&kind.trim()) && !library.trim().is_empty() {
            record_not_held.clear();
            let _ = write!(
                record_not_held,
                "{entity} names record `{}` of {kind} library `{library}`, which the stored library does not hold. The model's values stand.",
                member("record_id").unwrap_or("(none recorded)")
            );
            yielded = 2;
            let record = member("record_id").filter(|record| !record.trim().is_empty());
            if let (Some(record), Some(_)) = (record, member("library_version")) {
                changed.clear();
                let _ = write!(
                    changed,
                    "The stored record `{record}` of {kind} library `{library}` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record."
                );
                yielded = 3;
            }
        }
        let alternatives = [library_finding(UNRESOLVED, entity, not_held), library_finding(UNRESOLVED, entity, record_not_held), library_finding(VERSION_CHANGED, entity, changed)];
        visit(&alternatives[..yielded]);
    });
}

/// A library store finding on `entity` (`libraryFindingDiagnostics`).
fn library_finding<'a>(code: &'a str, entity: &'a str, message: &'a str) -> Fields<'a> {
    Fields::session(code, ("diagnostic_source", LIBRARY_REF_CHECK_SOURCE), ("model_entity", entity), message, LIBRARY_REMEDIATION)
}

/// Every entry the session may append for `model`, each site's alternatives
/// in turn, materialized (tests: the module docs' kinds).
#[cfg(test)]
pub(crate) fn session_diagnostics(model: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for_each_site(model, &mut |alternatives| {
        for entry in alternatives {
            out.push(serde_json::json!({
                "code": entry.code, "class": entry.class, "severity": entry.severity,
                "source": {"ref_type": entry.source.0, "ref_id": entry.source.1},
                "affected_object": {"ref_type": entry.affected.0, "ref_id": entry.affected.1},
                "message": entry.message, "remediation": entry.remediation,
            }));
        }
    });
    out
}

#[cfg(test)]
thread_local! {
    static STEPS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// One unit of the check's work: a site entry looked up, an appended entry
/// counted, or a distinct key visited (tests read the total, [`steps`]).
#[inline]
fn step() {
    #[cfg(test)]
    STEPS.with(|steps| steps.set(steps.get() + 1));
}

/// The units of work this thread's checks have taken (tests).
#[cfg(test)]
pub(crate) fn steps() -> u64 {
    STEPS.with(std::cell::Cell::get)
}

/// How a distinct appended entry sits among the sites that yield it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Link {
    /// No site yields it.
    Unseen,
    /// Every site that yields it yields no other appended entry before it.
    Root,
    /// Every site that yields it yields this other appended entry first.
    Parent(usize),
    /// Sites yield it in different positions ([`place`]).
    Conflict,
}

/// Why an appended entry is refused, by its index among the appended entries.
#[derive(Debug, PartialEq, Eq)]
enum Unplaced {
    /// No assignment of the appended entries to distinct sites that yield
    /// them places this entry and the entries before it.
    NotDerived(usize),
    /// [`SESSION_CHECK_LIMIT`]: the sites that yield this entry also yield
    /// other appended entries in different orders, which the check does not
    /// match.
    Limit(usize),
}

fn find(sets: &mut [usize], mut key: usize) -> usize {
    while sets[key] != key {
        sets[key] = sets[sets[key]];
        key = sets[key];
    }
    key
}

/// An assignment of each appended entry to a distinct site that may yield it
/// (each site yields at most one entry, so a repeated entry is admitted only
/// as many times as the model has sites yielding it, and a site that may
/// yield one of several entries is used once), without a general matching.
///
/// Only the appended entries are keyed (by their bound fields, borrowed from
/// the report). Each site is reduced to the appended entries it may yield, in
/// its order (the library-not-held, record-not-held, record-changed order);
/// within a site those are distinct. When every site that yields an appended
/// entry yields the same appended entry just before it (or none), the entries
/// form a forest of depth at most 2 and each site may yield exactly the
/// entries on the path from its last one to a root. Then the assignment
/// exists exactly when, taking the deepest entries first, each entry's count
/// is at most the sites ending at it or below that its descendants left
/// unused: such sites can yield it and every entry above it alike, so only
/// their number matters. The refused entry is the first whose prefix of the
/// appended entries has no assignment (a binary search over prefixes, since a
/// prefix of an assignable list is assignable).
///
/// Every genuine site structure is such a forest: a site's library findings
/// share its entity, a record-changed text fixes the record-not-held text, and
/// the three texts of a site differ. Distinct sites can share a text without
/// such a forest only where reference fields spell one finding's text through
/// different splits (a backtick in a kind, library or record id); an entry
/// there is refused by name as [`Unplaced::Limit`], a limit of this check.
///
/// Work: three lookups per site, then the appended entries and their distinct
/// keys once per prefix tried. Storage: the appended entries' keys and a few
/// counters per distinct key. No recursion.
fn place(appended: &[Value], model: &Value) -> Result<(), Unplaced> {
    let mut index: HashMap<Fields<'_>, usize> = HashMap::with_capacity(appended.len());
    let claims: Vec<Option<usize>> = appended
        .iter()
        .map(|entry| {
            step();
            Fields::of(entry).map(|fields| {
                let next = index.len();
                *index.entry(fields).or_insert(next)
            })
        })
        .collect();
    let distinct = index.len();
    let mut links = vec![Link::Unseen; distinct];
    let mut supply = vec![0usize; distinct];
    let mut sets: Vec<usize> = (0..distinct).collect();
    for_each_site(model, &mut |alternatives| {
        let (mut path, mut length) = ([0usize; 3], 0);
        for alternative in alternatives {
            step();
            if let Some(&key) = index.get(alternative) {
                if !path[..length].contains(&key) {
                    path[length] = key;
                    length += 1;
                }
            }
        }
        for at in 0..length {
            let expected = if at == 0 { Link::Root } else { Link::Parent(path[at - 1]) };
            let link = &mut links[path[at]];
            *link = match *link {
                Link::Unseen => expected,
                same if same == expected => same,
                _ => Link::Conflict,
            };
            if at > 0 {
                let (above, below) = (find(&mut sets, path[at - 1]), find(&mut sets, path[at]));
                sets[below] = above;
            }
        }
        if length > 0 {
            supply[path[length - 1]] += 1;
        }
    });
    drop(index);
    // An entry no site yields, or not of a session entry's form.
    let not_derived = claims.iter().position(|claim| claim.is_none_or(|key| links[key] == Link::Unseen));
    // An entry in a group of entries whose sites order them differently.
    let mut conflicted = vec![false; distinct];
    for key in 0..distinct {
        step();
        if links[key] == Link::Conflict {
            let set = find(&mut sets, key);
            conflicted[set] = true;
        }
    }
    let limited = claims.iter().position(|claim| claim.is_some_and(|key| conflicted[find(&mut sets, key)]));
    match (not_derived, limited) {
        (Some(entry), limit) if limit.is_none_or(|limit| entry < limit) => return Err(Unplaced::NotDerived(entry)),
        (_, Some(entry)) => return Err(Unplaced::Limit(entry)),
        _ => {}
    }
    // Every claimed entry now has a key at depth 0, 1 or 2, deepest first.
    let depth = |key: usize| match links[key] {
        Link::Parent(parent) if matches!(links[parent], Link::Parent(_)) => 2,
        Link::Parent(_) => 1,
        _ => 0,
    };
    let mut order: Vec<usize> = (0..distinct).collect();
    order.sort_by_key(|&key| std::cmp::Reverse(depth(key)));
    let (mut demand, mut pool) = (vec![0usize; distinct], vec![0usize; distinct]);
    let mut assignable = |prefix: usize| {
        demand.iter_mut().for_each(|count| *count = 0);
        for claim in &claims[..prefix] {
            step();
            demand[claim.expect("every claimed entry is keyed")] += 1;
        }
        pool.copy_from_slice(&supply);
        for &key in &order {
            step();
            if pool[key] < demand[key] {
                return false;
            }
            if let Link::Parent(parent) = links[key] {
                pool[parent] += pool[key] - demand[key];
            }
        }
        true
    };
    if assignable(appended.len()) {
        return Ok(());
    }
    // The shortest unassignable prefix: assignable at `low`, not at `high`.
    let (mut low, mut high) = (0, appended.len());
    while high - low > 1 {
        let middle = low + (high - low) / 2;
        if assignable(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }
    Err(Unplaced::NotDerived(high - 1))
}

/// The index, among `appended`, of the first entry with a code and affected
/// object of one of `results` (the result's projected diagnostics), looked up
/// by code and affected object id.
fn first_copy_of_a_result_diagnostic(appended: &[Value], results: &[Value]) -> Option<usize> {
    let affected_id = |entry: &Value| entry.get("affected_object").and_then(|object| object.get("ref_id"));
    let key = |entry: &Value| Some((entry.get("code")?.as_str()?, affected_id(entry)?.as_str()?));
    // The projection writes both as strings; any other result is compared whole.
    let (mut claimed, mut other) = (HashSet::with_capacity(results.len()), Vec::new());
    for result in results {
        match key(result) {
            Some(pair) => {
                claimed.insert(pair);
            }
            None => other.push(result),
        }
    }
    appended.iter().position(|entry| {
        key(entry).is_some_and(|pair| claimed.contains(&pair))
            || other.iter().any(|result| result.get("code") == entry.get("code") && affected_id(result) == affected_id(entry))
    })
}

/// The binding (module docs): every entry of `report_sections.diagnostics`
/// after the result's projected entries is one the host derives from `model`,
/// each site yielding at most one ([`for_each_site`], [`place`]).
/// `Err` names the entry's index only. A report with no entry after the
/// result's derives nothing.
pub(crate) fn check_session_diagnostics(input: &Value, source: &Value, model: &Value) -> Result<(), String> {
    let found = input.pointer("/report_sections/diagnostics").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let Some(result_count) = source.get("diagnostics").and_then(Value::as_array).map(Vec::len) else {
        return expected_report_diagnostics(source).map(drop);
    };
    if found.len() <= result_count {
        return Ok(());
    }
    let results = expected_report_diagnostics(source)?;
    let appended = found.get(results.len()..).unwrap_or(&[]);
    if let Some(index) = first_copy_of_a_result_diagnostic(appended, &results) {
        return Err(format!("report diagnostic {} copies a result diagnostic's code and affected object", results.len() + index));
    }
    let count = results.len();
    drop(results);
    place(appended, model).map_err(|unplaced| match unplaced {
        Unplaced::NotDerived(index) => format!("report diagnostic {} is not a session diagnostic the host derives from the bound model", count + index),
        Unplaced::Limit(index) => format!(
            "report diagnostic {}: {SESSION_CHECK_LIMIT}: the bound model's library references yield this entry's text at sites that order their findings differently, which this check does not match; this is a limit of the check, not a fault of the report",
            count + index
        ),
    })
}
