"""RV113 (RV-R): a mutant schema in the reviewer's own copy (WT/rv113/mut) of the head.

Each mutant is a source edit guarded by `mx("<id>")`, true only when RV113_MUT equals the
id; with RV113_MUT unset the copy behaves as the head (checked by a control run). One
build serves every mutant. Usage: python make_mutants.py <RE dir> <manifest out>
"""
import json
import sys

re_dir, manifest_out = sys.argv[1], sys.argv[2]
rs_path = f"{re_dir}/src/retained_precision.rs"
sb_path = f"{re_dir}/src/source_blocks.rs"
rs = open(rs_path).read()
sb = open(sb_path).read()
manifest = []


def sub(text, old, new, mid, desc, family, path):
    assert text.count(old) == 1, (mid, text.count(old), old[:80])
    manifest.append({"id": mid, "family": family, "file": path, "description": desc})
    return text.replace(old, new)


HELPER = '\nfn mx(id: &str) -> bool {\n    std::env::var("RV113_MUT").map_or(false, |v| v == id)\n}\n'
anchor = "const MAX_BITS: u64 = 0x7fef_ffff_ffff_ffff;\n"
assert rs.count(anchor) == 1
rs = rs.replace(anchor, anchor + HELPER)

# --- R-D38 (4b): the branch and the predicate ---------------------------------------
old = '''            if st["native"] == "failed" && a["run_ref"].is_null() {
                pf(d38_capture_before_run(c, a, ai))?;
            } else {'''
new = '''            let branch = if mx("M01") {
                a["run_ref"].is_null()
            } else if mx("M02") {
                st["native"] == "failed" || a["run_ref"].is_null()
            } else if mx("M03") {
                false
            } else {
                st["native"] == "failed" && a["run_ref"].is_null()
            };
            if branch {
                pf(mx("M04") || d38_capture_before_run(c, a, ai))?;
            } else {'''
rs = sub(rs, old, new, "M01", "(4b) branch without `native == failed` (any entered native stage with no run_ref)", "D38", "RS")
manifest += [
    {"id": "M02", "family": "D38", "file": "RS", "description": "(4b) branch `&&` -> `||` (a native-failed attempt with a Run is also routed to (4b))"},
    {"id": "M03", "family": "D38", "file": "RS", "description": "the relaxed check restored (I90's D38-1, replicated)"},
    {"id": "M04", "family": "D38", "file": "RS", "description": "(4b)'s predicate replaced by true (no (4b) conjunct)"},
]
old = '''fn d38_capture_before_run(c: &Value, a: &Value, ai: usize) -> bool {
    let st = &a["stages"];
    a["result"]["kind"] == "unavailable"'''
new = '''fn d38_capture_before_run(c: &Value, a: &Value, ai: usize) -> bool {
    let st = &a["stages"];
    if mx("M05") {
        return a["source_ref"] == c["source_ref"];
    }
    if mx("M06") {
        return !c["source_ref"].is_null();
    }
    a["result"]["kind"] == "unavailable"'''
rs = sub(rs, old, new, "M05", "(4b)'s predicate reduced to the source equality alone (I90's 'minimal form')", "D38", "RS")
manifest.append({"id": "M06", "family": "D38", "file": "RS", "description": "(4b)'s predicate reduced to `case.source_ref` non-null (equality weakened)"})

# --- G8: F-1 text B per case ---------------------------------------------------------
old = '''        // P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        fail(
            modes.len() == 1'''
new = '''        let selected_only = b["cases"][i]["status"] == "selected";
        // P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        fail(
            (mx("M14") && !selected_only) || modes.len() == 1'''
rs = sub(rs, old, new, "M14", "P1 for selected cases only (P2-P4 still for every case)", "G8", "RS")
old = '''                    == Some(if mode == "sparse_interactive" {
                        1.0
                    } else {
                        2.0
                    }),
        )?;
        let parity = rows'''
new = '''                    == Some(if mode == "sparse_interactive" {
                        1.0
                    } else {
                        2.0
                    })
                || (mx("M14") && !selected_only),
        )?;
        let parity_rows = if mx("M16") { rows_for(source, &b["cases"][0]) } else { rows.clone() };
        let parity = parity_rows'''
rs = sub(rs, old, new, "M16", "P2-P4 count the parity rows of case 0 (rows mis-indexed in the loop)", "G8", "RS")
old = '''        fail(
            parity <= 1
                && (parity == 0 || mode == "dense_scrutiny")
                && (parity == 0 || o["w2"]["kind"] != "published"),
        )?;'''
new = '''        let w2 = if mx("M10") { &b["ordinary_attempts"][0]["w2"] } else { &o["w2"] };
        let p4 = if mx("M11") { w2["kind"] == "not_triggered" } else { w2["kind"] != "published" };
        let ok = if mx("M18") && selected_only {
            parity == usize::from(mode == "dense_scrutiny")
        } else {
            parity <= (if mx("M13") { 2 } else { 1 })
                && (parity == 0 || mode == "dense_scrutiny")
                && (parity == 0 || p4 || (mx("M12") && !selected_only))
        };
        fail((mx("M15") && !selected_only) || ok)?;'''
rs = sub(rs, old, new, "M10", "P4 reads case 0's ordinary attempt instead of the case's own", "G8", "RS")
manifest += [
    {"id": "M11", "family": "G8", "file": "RS", "description": "P4 stricter: a parity row needs W2 not_triggered (refuses it beside a failed W2)"},
    {"id": "M12", "family": "G8", "file": "RS", "description": "P4 for selected cases only"},
    {"id": "M13", "family": "G8", "file": "RS", "description": "P2 widened to at most two parity rows"},
    {"id": "M15", "family": "G8", "file": "RS", "description": "P2-P4 for selected cases only (P1 still for every case)"},
    {"id": "M18", "family": "G8", "file": "RS", "description": "the old 'exactly one parity row iff dense' kept for selected cases; P2-P4 only for the others"},
]

# --- G5: the not_required rule and the report-outcome equality ------------------------
old = '''        if c["status"] == "not_required" {
            fail(c["product_attempt_ref"].is_null() && quality["solve_quality"] == "checks_passed")?;
        }'''
new = '''        if c["status"] == "not_required" {
            fail(
                (mx("M20") || c["product_attempt_ref"].is_null())
                    && (mx("M21") || quality["solve_quality"] == "checks_passed")
                    && (!mx("M24") || o["w2"]["kind"] != "published"),
            )?;
        }'''
rs = sub(rs, old, new, "M20", "not_required: `product_attempt_ref` null dropped", "G5", "RS")
manifest += [
    {"id": "M21", "family": "G5", "file": "RS", "description": "not_required: verdict checks_passed dropped"},
    {"id": "M24", "family": "G5", "file": "RS", "description": "not_required: a W2-published case refused (a partial restoration)"},
]
old = '''        if o["initial"]["kind"] == "report" && o["w2"]["kind"] == "not_triggered" {
            fail(o["initial"]["outcome"] == quality["solve_quality"])?;
        }'''
new = '''        if !mx("M23") && o["initial"]["kind"] == "report" && (mx("M22") || o["w2"]["kind"] == "not_triggered") {
            fail(o["initial"]["outcome"] == quality["solve_quality"])?;
        }'''
rs = sub(rs, old, new, "M22", "the report-outcome equality's W2 guard dropped (claimed equivalent under D6c)", "G5", "RS")
manifest.append({"id": "M23", "family": "G5", "file": "RS", "description": "the report-outcome equality dropped"})
old = '''        if matches!(text(&c["status"]), "selected" | "not_required") {
            fail(o["initial"]["kind"] != "not_attempted")?;
        }'''
new = '''        if matches!(text(&c["status"]), "selected" | "not_required") && !(mx("M25") && c["status"] == "not_required") {
            fail(o["initial"]["kind"] != "not_attempted")?;
        }'''
rs = sub(rs, old, new, "M25", "the existing `initial != not_attempted` check no longer applied to not_required (now its only guard)", "G5", "RS")

# --- N-5 (source_blocks.rs) -----------------------------------------------------------
sb_anchor = "fn integer(v: &Value) -> Result<usize, String> {\n"
assert sb.count(sb_anchor) == 1
sb = sb.replace(sb_anchor, 'fn mx(id: &str) -> bool {\n    std::env::var("RV113_MUT").map_or(false, |v| v == id)\n}\n' + sb_anchor)
old = '''        .filter(|n| *n <= 9_007_199_254_740_991)'''
new = '''        .filter(|n| mx("M30") || (if mx("M31") { *n < 9_007_199_254_740_991 } else { *n <= 9_007_199_254_740_991 }))'''
sb = sub(sb, old, new, "M30", "integer's 2^53-1 bound removed (RV95's S1, replicated)", "N-5", "source_blocks.rs")
manifest.append({"id": "M31", "family": "N-5", "file": "source_blocks.rs", "description": "integer's bound off by one (`<`)"})
# M32 (a call site bypassed) is a source-text change the runtime schema cannot express;
# N-5's call-site census asserts the exact argument set, so its removal fails that assertion.

open(rs_path, "w").write(rs)
open(sb_path, "w").write(sb)
json.dump(manifest, open(manifest_out, "w"), indent=1)
print(len(manifest), "mutants:", ", ".join(m["id"] for m in manifest))
