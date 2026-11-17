"""OWN-PRES (PR-A, retained-pricing-rules.md §8.62): an owner a model prices where its source exists may not silently
disappear from a current source.

Each model below returns a falsy reading when its owner's source is absent ("a snapshot without them adds nothing"),
so that the earlier sources it was written beside keep their registered coefficients. Absence alone cannot tell such an
earlier source from a current one that lost, renamed or moved the owner, and the second would drop the owner's charge
without a stop (U5B-PD did so at PR-A's head before its re-bind). This registry closes that class on an independent,
source-bound ground: owners arrive in epochs, and no later owner arrives on a source that predates an earlier one. A
snapshot holding any owner of epoch k is at or after epoch k, so every owner of every epoch up to k must be present
there; one that is absent stops the run (exit 4). A source older than all of them holds none and adds nothing, as
before.

The epochs follow the governing rules' sections (§8.8 ... §8.62) and the README's order. Two kinds of error in that
order are possible, and both only stop a run: an owner listed too early can stop a historical replay whose source
legitimately predates it; one listed too late leaves it unchecked only on sources older than its listed epoch, which
the current full-file pins do not admit. On the current source, which holds the latest epoch's owner, every owner is
required.

A deliberate rename keeps its charge only through an explicit correspondence in the owner's model (U5B-PD: PD_SHAPES
in u4b_u5b_owners_model.py, published_nodal_displacements -> corrected_nodal_displacements, the same Vec<f64> of N);
the model then reads the owner as present under either name. A retirement needs an entry in RETIRED with the epoch
that removed it and why its charge is no longer owed; there is none today.

`check(snapshot)` returns the census {label: present} and raises SystemExit naming every missing owner."""
import os
import runpy

HERE = os.path.dirname(os.path.abspath(__file__))
# (epoch, label, model file, entry, key in the entry's dict reading or None for the reading's truth)
OWNERS = (
    (10, "KD5C6 first-error inventory", "kd5_first_error_model.py", "inspect", None),
    (20, "B5O1 batch 5 carriers", "batch5_model.py", "active", None),
    (30, "C7 prescribed all-rows field", "prescribed_key_model.py", "all_rows_field", None),
    (30, "C7 borrowed prescribed key", "prescribed_key_model.py", "borrowed", None),
    (40, "RF source congruence", "rf_source_model.py", "active", None),
    (50, "W1 reductions", "w1_reductions_model.py", "inspect", None),
    (60, "U6-ES exact sections", "u6_owners_model.py", "inspect", "exact_sections"),
    (60, "U6-WS reduced workspace", "u6_owners_model.py", "inspect", "reduced_workspace"),
    (60, "U8-FM frame members", "u6_owners_model.py", "inspect", "frame_members"),
    (60, "U6 all rows", "u6_owners_model.py", "inspect", "all_rows"),
    (70, "U5B-PD published displacements", "u4b_u5b_owners_model.py", "inspect", "published_displacements"),
    (70, "U4B-ST station quantities", "u4b_u5b_owners_model.py", "inspect", "station_quantities"),
    (70, "U5B-AB absorption flags", "u4b_u5b_owners_model.py", "inspect", "absorption"),
    (70, "U4B-W1 not-covered operands", "u4b_u5b_owners_model.py", "inspect", "not_covered"),
    (70, "U4B-RD reader sets", "u4b_u5b_owners_model.py", "inspect", "readers"),
    (70, "U4B-NQ not-qualified inputs", "u4b_u5b_owners_model.py", "inspect", "not_qualified"),
    (75, "U5B-CG contribution groups", "u4b_u5b_owners_model.py", "inspect", "contribution_groups"),
    (80, "O4 owners", "o4_owners_model.py", "inspect", None),
    (90, "W1R-FS final scales", "w1r_owners_model.py", "inspect", None),
    (100, "RA-HS/RA-SC shortcuts", "ra_owners_model.py", "active", None),
    (110, "W8 topology", "w8_topology_model.py", "inspect", None),
    (115, "S1-TS typed strings (model 0.7.0)", "typed_strings_model.py", "inspect", None),
    (120, "SD silent-drop guard", "sd_owners_model.py", "inspect", None),
    (130, "W7 global end rows", "w7_owners_model.py", "inspect", None),
    (140, "W15 rigid components", "w15_owners_model.py", "inspect", None),
    (150, "W16 imposed motion", "w16_owners_model.py", "inspect", None),
    (160, "S1 lowering and withholding", "s1_owners_model.py", "inspect", None),
    (170, "RTA3B-PS/MR published index", "u4b_u5b_owners_model.py", "published_index", None),
    (180, "RT-A4 owners", "rta4_owners_model.py", "inspect", None),
    (190, "RTU4-RC region binding", "rtu4_owners_model.py", "inspect", None),
    (200, "RTA3C certificate owners", "rta3c_owners_model.py", "inspect", None),
    (210, "CEF constant-effort fallback", "ce_fallback_model.py", "inspect", None),
    (220, "RTL2 pressure elongation", "rtl2_owners_model.py", "inspect", None),
    (230, "PRA-CF correction failure texts", "correction_failure_model.py", "inspect", None),
)
# label -> (epoch that removed it, why its charge is no longer owed). None today.
RETIRED = {}


def census(snapshot):
    import sys
    if HERE not in sys.path:
        sys.path.insert(0, HERE)
    models, out = {}, {}
    for epoch, label, model, entry, key in OWNERS:
        if model not in models:
            models[model] = runpy.run_path(os.path.join(HERE, model))
        try:
            reading = models[model][entry](snapshot)
        except (SystemExit, ValueError) as error:
            # A model that stops here stops again at its own pricing step (the same model, the same snapshot), which
            # reports it; the census records it as not absent. Only a falsy reading is an absence.
            out[label] = f"stopped: {str(error)[:200]}"
            continue
        if key is not None:
            if not isinstance(reading, dict) or key not in reading:
                raise SystemExit(f"OWN-PRES: {model}:{entry} gave no reading for {key}; re-review (§8.62)")
            reading = reading[key]
        out[label] = bool(reading)
    return out


def check(snapshot):
    present = census(snapshot)
    epochs = {label: epoch for epoch, label, *_ in OWNERS}
    latest = max((epochs[label] for label, ok in present.items() if ok), default=None)   # a stopped model counts as present
    if latest is None:
        return present
    missing = [label for label, ok in present.items()
               if not ok and epochs[label] <= latest and label not in RETIRED]
    if missing:
        raise SystemExit(f"OWN-PRES: owners required at epoch <= {latest} are absent (a current source lost, renamed or moved "
                         f"them without a reviewed correspondence): {missing}; re-review (§8.62)")
    return present


if __name__ == "__main__":
    import json
    import sys
    print(json.dumps(check(sys.argv[1]), indent=1))
