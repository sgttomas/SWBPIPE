#!/usr/bin/env python3
"""RV112: install RV112's mutant schemata and probe hooks into the reviewer's `mut` copy
(a git archive of b1-a 9812c83ded). One build carries every runtime mutant; the environment
variable RV112_MUT=<id> selects one at run time (unset: the candidate's own behaviour).

Usage: schemata.py <PP src dir> install | list
Each edit is an exact, unique textual replacement of the candidate's text; the tool refuses
to install if any original snippet is missing or not unique. `phase_caps` loses `const` here
(it is only called at run time), so its entries can be switched; that changes no value.
"""
import sys

M = 'crate::retained_memory::rv112_mut'

# (id, kind, description) -- kind: PLAN (PLAN_v2 §2.3's list) or RV112 (reviewer's own)
MUTANTS = [
    ("A01", "PLAN", "census: per-case load rows read case 0 only (length and capacity)"),
    ("A02", "RV112", "census: Loads length from case 0 only (capacity still the maximum)"),
    ("A03", "RV112", "census: LoadsCapacity from case 0 only (length still the maximum)"),
    ("A04", "RV112", "census: Σ l_i is the maximum length, not the sum"),
    ("A05", "RV112", "census: Σ l_i counts case 0 only"),
    ("B01", "PLAN", "D1.4: refuses c ≥ C (admits c < C only)"),
    ("B02", "PLAN", "D1.4: admits c ≤ C + 1"),
    ("B03", "RV112", "D1.4: admits c = 0 (the empty check dropped)"),
    ("B04", "RV112", "D1.5: reads case 0 only"),
    ("B05", "RV112", "D1.5: skips case 0"),
    ("B06", "RV112", "D1.7: reads case 0's loads only"),
    ("C01", "RV112", "cap_rows: TotalLoads capped by l (128), not L"),
    ("C02", "RV112", "cap_rows: LoadCasesCapacity capped by 1 (D1's), not C"),
    ("C03", "RV112", "cap_rows: TotalLoads observes the largest case, not Σ"),
    ("D01", "PLAN", "G-B: running total dropped (observes 0)"),
    ("D02", "PLAN", "G-B: running total is the current case only"),
    ("D03", "RV112", "G-B: CaseLoadsTotal bounded by l, not L"),
    ("D04", "RV112", "G-B: CaseLoads and CaseLoadsTotal observations swapped"),
    ("E01", "PLAN", "G-C: EnvelopeResults ≤ P_final, not C·P_final"),
    ("E02", "RV112", "G-C: EnvelopeResultCapacity ≤ PushCap(P_final)"),
    ("E03", "RV112", "G-C: EnvelopeResultTextBytes ≤ 2·P_final·Text(row)"),
    ("E04", "RV112", "G-C: ContractEvidenceArrayElements without ×C"),
    ("E05", "RV112", "G-C: ContractEvidenceObjects without ×C"),
    ("E06", "RV112", "G-C: ContractEvidenceEntries without ×C"),
    ("E07", "RV112", "G-C: ContractEvidenceStringBytes without ×C"),
    ("E08", "RV112", "G-C: ContractEvidenceKeyBytes without ×C"),
    ("E09", "RV112", "G-C: RetainedErrorTextBytes ≤ (3m + 1)·Text(err), without ×C"),
    ("F01", "PLAN", "T-3 (e): seeds only (I1's predicate: non-empty and every initial)"),
    ("F02", "PLAN", "T-3 (e): requested ignored at G-C (the seed count passed as requested)"),
    ("F03", "PLAN", "T-3 (e): ≥ instead of =="),
    ("F04", "RV112", "T-3 (e): ≤ instead of =="),
    ("F05", "RV112", "T-3 (e): requested ≥ 1 dropped"),
    ("F06", "RV112", "T-3 (e): any seed's initial, not every"),
    ("F07", "RV112", "T-3 (e): compared with C, not the requested count"),
    ("G01", "RV112", "RetainedErrorTextBytes: the capture's own fields only (the unpatched reader)"),
    ("G02", "RV112", "RetainedErrorTextBytes: the parked slots only"),
    ("G03", "RV112", "RetainedErrorTextBytes: parked slots' error only (observable_error dropped)"),
    ("G04", "RV112", "RetainedErrorTextBytes: the first parked slot only"),
]

EDITS = [
    # rv112_mut itself, and the probe module (a child of retained_memory, so it sees private items).
    ("retained_memory.rs",
     "#[cfg(test)]\n#[path = \"retained_memory_law_tests.rs\"]\nmod law_tests;\n",
     "#[cfg(test)]\n#[path = \"retained_memory_law_tests.rs\"]\nmod law_tests;\n"
     "#[cfg(test)]\n#[path = \"zz_rv112_probe.rs\"]\npub(crate) mod zz_rv112_probe;\n"
     "/// RV112: the selected mutant (environment variable RV112_MUT), read once.\n"
     "pub(crate) fn rv112_mut(id: &str) -> bool {\n"
     "    static SELECTED: std::sync::OnceLock<String> = std::sync::OnceLock::new();\n"
     "    SELECTED.get_or_init(|| std::env::var(\"RV112_MUT\").unwrap_or_default()) == id\n"
     "}\n"),
    # census
    ("retained_memory.rs",
     "            let loads = &mut self.facts.primitive_loads;\n"
     "            loads.length = loads.length.max(case.primitive_loads.len());\n"
     "            loads.capacity = loads.capacity.max(case.primitive_loads.capacity());\n"
     "            let total = u32::try_from(case.primitive_loads.len()).ok().and_then(|l| self.facts.total_loads.checked_add(l));\n",
     "            let first = std::ptr::eq(case, &m.load_cases[0]);\n"
     "            let loads = &mut self.facts.primitive_loads;\n"
     f"            if !(({M}(\"A01\") || {M}(\"A02\")) && !first) {{ loads.length = loads.length.max(case.primitive_loads.len()); }}\n"
     f"            if !(({M}(\"A01\") || {M}(\"A03\")) && !first) {{ loads.capacity = loads.capacity.max(case.primitive_loads.capacity()); }}\n"
     f"            let total = if {M}(\"A04\") {{ u32::try_from(loads.length).ok() }} else if {M}(\"A05\") && !first {{ Some(self.facts.total_loads) }} else {{ u32::try_from(case.primitive_loads.len()).ok().and_then(|l| self.facts.total_loads.checked_add(l)) }};\n"),
    # D1.4
    ("retained_memory.rs",
     "    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES {\n",
     f"    if (!{M}(\"B03\") && m.load_cases.is_empty()) || (if {M}(\"B01\") {{ m.load_cases.len() >= caps::LOAD_CASES }} else if {M}(\"B02\") {{ m.load_cases.len() > caps::LOAD_CASES + 1 }} else {{ m.load_cases.len() > caps::LOAD_CASES }}) {{\n"),
    # D1.5
    ("retained_memory.rs",
     "    for case in &m.load_cases {\n        if case.pressure_regions.is_some() {\n",
     f"    for case in m.load_cases.iter().skip(usize::from({M}(\"B05\"))).take(if {M}(\"B04\") {{ 1 }} else {{ usize::MAX }}) {{\n        if case.pressure_regions.is_some() {{\n"),
    # D1.7
    ("retained_memory.rs",
     "    for load in m.load_cases.iter().flat_map(|case| &case.primitive_loads) {\n",
     f"    for load in m.load_cases.iter().take(if {M}(\"B06\") {{ 1 }} else {{ usize::MAX }}).flat_map(|case| &case.primitive_loads) {{\n"),
    # cap_rows
    ("retained_memory.rs",
     "        row(K::TotalLoads, n.total_loads as usize, TOTAL_LOADS),\n",
     f"        row(K::TotalLoads, if {M}(\"C03\") {{ n.primitive_loads.length }} else {{ n.total_loads as usize }}, if {M}(\"C01\") {{ LOADS }} else {{ TOTAL_LOADS }}),\n"),
    ("retained_memory.rs",
     "        row(K::LoadCasesCapacity, t.load_cases.capacity, LOAD_CASES),\n",
     f"        row(K::LoadCasesCapacity, t.load_cases.capacity, if {M}(\"C02\") {{ 1 }} else {{ LOAD_CASES }}),\n"),
    # G-B observations
    ("retained_memory.rs",
     "        o(P::CaseLoads, count(f.case.primitive_loads.len())),\n"
     "        o(P::CaseLoadsTotal, count(f.capture.late_loads_total)),\n",
     f"        o(P::CaseLoads, count(if {M}(\"D04\") {{ f.capture.late_loads_total }} else {{ f.case.primitive_loads.len() }})),\n"
     f"        o(P::CaseLoadsTotal, count(if {M}(\"D01\") {{ 0 }} else if {M}(\"D02\") || {M}(\"D04\") {{ f.case.primitive_loads.len() }} else {{ f.capture.late_loads_total }})),\n"),
    # T-3 (e) at the call site
    ("retained_memory.rs",
     "        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture, f.requested_cases))),\n",
     f"        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture, if {M}(\"F02\") {{ f.capture.ordinary.len() }} else {{ f.requested_cases }}))),\n"),
    # T-3 (e) predicate
    ("retained_memory.rs",
     "    requested >= 1 && capture.ordinary.len() == requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())\n",
     f"    if {M}(\"F01\") {{ return !capture.ordinary.is_empty() && capture.ordinary.iter().all(|seed| seed.initial.is_some()); }}\n"
     f"    let n = capture.ordinary.len();\n"
     f"    let count = if {M}(\"F03\") {{ n >= requested }} else if {M}(\"F04\") {{ n <= requested }} else if {M}(\"F07\") {{ n == caps::LOAD_CASES }} else {{ n == requested }};\n"
     f"    let each = if {M}(\"F06\") {{ capture.ordinary.iter().any(|seed| seed.initial.is_some()) }} else {{ capture.ordinary.iter().all(|seed| seed.initial.is_some()) }};\n"
     f"    ({M}(\"F05\") || requested >= 1) && count && each\n"),
    # phase_caps: run-time only here
    ("retained_memory.rs",
     "pub(super) const fn phase_caps() -> PhaseCaps {\n",
     "pub(super) fn phase_caps() -> PhaseCaps {\n"),
    ("retained_memory.rs",
     "            c * P_FINAL,\n"
     "            push_capacity(c * P_FINAL),\n"
     "            2 * c * P_FINAL * text_atoms::ROW,\n",
     f"            if {M}(\"E01\") {{ P_FINAL }} else {{ c * P_FINAL }},\n"
     f"            push_capacity(if {M}(\"E02\") {{ P_FINAL }} else {{ c * P_FINAL }}),\n"
     f"            2 * (if {M}(\"E03\") {{ 1 }} else {{ c }}) * P_FINAL * text_atoms::ROW,\n"),
    ("retained_memory.rs",
     "            c * (3 * m + 2 * g),\n"
     "            c * (3 + m + g),\n"
     "            c * (9 + 15 * m + 2 * g),\n"
     "            c * (m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64)),\n"
     "            c * ((9 + 15 * m + 2 * g) * 40),\n",
     f"            (if {M}(\"E04\") {{ 1 }} else {{ c }}) * (3 * m + 2 * g),\n"
     f"            (if {M}(\"E05\") {{ 1 }} else {{ c }}) * (3 + m + g),\n"
     f"            (if {M}(\"E06\") {{ 1 }} else {{ c }}) * (9 + 15 * m + 2 * g),\n"
     f"            (if {M}(\"E07\") {{ 1 }} else {{ c }}) * (m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64)),\n"
     f"            (if {M}(\"E08\") {{ 1 }} else {{ c }}) * ((9 + 15 * m + 2 * g) * 40),\n"),
    ("retained_memory.rs",
     "            c * (3 * m + 1) * text_atoms::ERR,\n",
     f"            (if {M}(\"E09\") {{ 1 }} else {{ c }}) * (3 * m + 1) * text_atoms::ERR,\n"),
    ("retained_memory.rs",
     "        late: [n, m, m, g, LOADS as u64, TOTAL_LOADS as u64, k,",
     f"        late: [n, m, m, g, LOADS as u64, if {M}(\"D03\") {{ LOADS as u64 }} else {{ TOTAL_LOADS as u64 }}, k,"),
    # retained_error_text
    ("retained_memory.rs",
     "    let own = Bytes::ZERO.add(text(&capture.error)).add(text(&capture.observable_error));\n"
     "    capture.parked_cases().iter().fold(own, |sum, slot| sum.add(text(&slot.error)).add(text(&slot.observable_error)))\n",
     f"    let own = if {M}(\"G02\") {{ Bytes::ZERO }} else {{ Bytes::ZERO.add(text(&capture.error)).add(text(&capture.observable_error)) }};\n"
     f"    if {M}(\"G01\") {{ return own; }}\n"
     f"    let parked = capture.parked_cases();\n"
     f"    let parked = if {M}(\"G04\") {{ &parked[..parked.len().min(1)] }} else {{ parked }};\n"
     f"    parked.iter().fold(own, |sum, slot| if {M}(\"G03\") {{ sum.add(text(&slot.error)) }} else {{ sum.add(text(&slot.error)).add(text(&slot.observable_error)) }})\n"),
    # probe hook: record (case loads, capture bytes) immediately before G-B's site, with or without a permit
    ("retained_product.rs",
     "        if let Err(e)=checked {self.error=Some(e);return;}\n"
     "        // U3, G-B (I51 COMPOSITION §2): immediately before the late old-source\n",
     "        if let Err(e)=checked {self.error=Some(e);return;}\n"
     "        #[cfg(test)] super::retained_memory::zz_rv112_probe::record_before_g_b(&*self, case.primitive_loads.len());\n"
     "        // U3, G-B (I51 COMPOSITION §2): immediately before the late old-source\n"),
]


def install(src):
    texts = {}
    for f, old, new in EDITS:
        path = f"{src}/{f}"
        if f not in texts:
            texts[f] = open(path, encoding="utf-8").read()
        n = texts[f].count(old)
        if n != 1:
            raise SystemExit(f"snippet count {n} in {f}: {old[:80]!r}")
        texts[f] = texts[f].replace(old, new)
    for f, t in texts.items():
        open(f"{src}/{f}", "w", encoding="utf-8").write(t)
    print(f"installed {len(EDITS)} edits; {len(MUTANTS)} runtime mutants")


if __name__ == "__main__":
    if sys.argv[-1] == "list":
        for m in MUTANTS:
            print("\t".join(m))
    else:
        install(sys.argv[1])
