#!/usr/bin/env python3
"""RV109 (R3′ early read): mutants of SP's first part (T-2, T-5, T-6, T-7, the domain re-check).
Each is an exact, single-occurrence replacement in one PP src file of the reviewer's SP copy.

Usage: sp_mutants.py <PP src dir> apply <ID>   |   sp_mutants.py list
"""
import sys

P, L = "retained_product.rs", "lib.rs"
MUTANTS = {
    "S1_no_parking": (P,
        "if self.prepared_one_case_seen && self.parked.len()+1<model.load_cases.len() {self.park_case(model.load_cases.len())?;}",
        "",
        "T-2: a later case's early hook does not park the earlier case"),
    "S2_unqualified_ids": (P,
        "self.observation_fields_of(row, &captured.case, captured.mode, is_parity, true, index > 0, Some(snapshot))?;",
        "self.observation_fields_of(row, &captured.case, captured.mode, is_parity, true, false, Some(snapshot))?;",
        "decision 19: a later case's row id is not case-qualified"),
    "S3_no_per_case_presence": (P,
        "if counts[index] != [1, usize::from(captured.parity_produced)] {",
        "if false && counts[index] != [1, usize::from(captured.parity_produced)] {",
        "decision 19: the per-case final mode/parity presence check is dropped"),
    "S4_finish_ignores_parked_late_hooks": (P,
        "|| self.parked.iter().any(|slot|slot.prepared_late_calls!=1",
        "|| false && self.parked.iter().any(|slot|slot.prepared_late_calls!=1",
        "T-2/T-6: the final hook no longer checks the parked cases' late captures"),
    "S5_custody_ignores_parked_errors": (P,
        "if let Some(slot)=self.parked.iter_mut().find(|slot|slot.error.is_some()) {",
        "if let Some(slot)=self.parked.iter_mut().find(|slot|false && slot.error.is_some()) {",
        "T-6: a parked case's prior capture error is not a custody refusal"),
    "S6_custody_no_case_count": (P,
        "if self.cases_seen()!=requested {",
        "if false && self.cases_seen()!=requested {",
        "T-6: the requested case count is not checked"),
    "S7_custody_ignores_parked_native": (P,
        "|| self.native.is_some() || self.parked.iter().any(|slot|slot.native.is_some())",
        "|| self.native.is_some()",
        "T-6: native work already in a parked slot is not refused"),
    "S8_stop_after_failed_attempt": (P,
        "attempts.push(self.with_case(request,|capture|capture.prepare_attempt(request,attempt)));",
        "let a=self.with_case(request,|capture|capture.prepare_attempt(request,attempt)); let stop=!a.prepared; attempts.push(a); if stop {break;}",
        "T-7: a failed preparation stops the later cases' attempts"),
    "S9_no_failure_snapshot": (P,
        "Err(e)=>{trace.fail_entered();self.error=Some(e);trace.freeze(self);CaseAttempt{request,attempt,prepared:false,parts,trace}}",
        "Err(e)=>{trace.fail_entered();self.error=Some(e);CaseAttempt{request,attempt,prepared:false,parts,trace}}",
        "T-7/T-11: a failed attempt takes no terminal snapshot"),
    "S10_failure_not_in_slot": (P,
        "Err(e)=>{trace.fail_entered();self.error=Some(e);trace.freeze(self);CaseAttempt{request,attempt,prepared:false,parts,trace}}",
        "Err(e)=>{trace.fail_entered();let _=e;trace.freeze(self);CaseAttempt{request,attempt,prepared:false,parts,trace}}",
        "T-7: a failed attempt's error is not kept in its case's slot"),
    "S11_attempt_id_is_request": (P,
        "attempts.push(self.with_case(request,|capture|capture.prepare_attempt(request,attempt)));",
        "attempts.push(self.with_case(request,|capture|capture.prepare_attempt(request,request)));",
        "T-7: attempt ids are request indices, not start order"),
    "S12_attempt_on_wrong_slot": (P,
        "attempts.push(self.with_case(request,|capture|capture.prepare_attempt(request,attempt)));",
        "attempts.push(self.with_case(attempt,|capture|capture.prepare_attempt(request,attempt)));",
        "T-7: the attempt runs on the slot at its start index, not its request index"),
    "S13_one_reserved_slot": (L,
        "ordinary.diagnostics.try_reserve_exact(case_ids.len()).ok()?;",
        "ordinary.diagnostics.try_reserve_exact(1).ok()?;",
        "T-5: one diagnostics slot reserved, whatever |A|"),
    "S14_no_cross_notice_collision": (L,
        "if ordinary.diagnostics.iter().any(|d| d.id == id) || ids[..k].contains(&id) {",
        "if ordinary.diagnostics.iter().any(|d| d.id == id) {",
        "T-5: two notices with one id are not refused"),
    "S15_domain_allows_combination": (L,
        "if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0 {",
        "if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES {",
        "domain re-check: a combination is admitted"),
    "S16_domain_allows_no_case": (L,
        "if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0 {",
        "if cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0 {",
        "domain re-check: c = 0 is admitted"),
    "S17_rows_bound_to_first_case": (P,
        "                if same {\n                    owner = Some((index, captured));",
        "                if index == 0 {\n                    owner = Some((index, captured));",
        "decision 19: every mode/parity row is bound to request case 0"),
    "S18_one_case_binding_at_c_ge_2": (P,
        "if self.parked.is_empty() {self.bind_observations(ordinary)} else {self.bind_observations_by_case(ordinary)}",
        "self.bind_observations(ordinary)",
        "T-6: custody binds observations with the one-case binder at c >= 2"),
    "S19_late_hook_case_zero": (P,
        "if !self.checked_same(&model.load_cases[self.parked.len()].id,&case.id)?",
        "if !self.checked_same(&model.load_cases[0].id,&case.id)?",
        "T-2: the late hook checks request case 0's id for every case"),
    "S20_observation_bytes_assigned": (P,
        "self.observation_capacity_bytes[slot] = self.observation_capacity_bytes[slot]\n            .checked_add(value.capacity())\n            .ok_or(CaptureError::CountRange(\"observation capacity total\"))?;",
        "self.observation_capacity_bytes[slot] = value.capacity();",
        "the observation capacity record is the last case's, not the sum"),
}


def main():
    if sys.argv[1] == "list":
        for k, (f, _, _, why) in MUTANTS.items():
            print(f"{k}\t{f}\t{why}")
        return
    src, action, mid = sys.argv[1], sys.argv[2], sys.argv[3]
    assert action == "apply"
    f, old, new, _ = MUTANTS[mid]
    path = f"{src}/{f}"
    text = open(path, encoding="utf-8").read()
    hits = text.count(old)
    if hits != 1:
        sys.exit(f"{mid}: {hits} hits in {f}")
    open(path, "w", encoding="utf-8").write(text.replace(old, new))
    print(f"applied {mid} to {f}")


if __name__ == "__main__":
    main()
