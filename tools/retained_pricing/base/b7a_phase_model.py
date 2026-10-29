"""Current W1 stage-2 route and successor premises.

Refused/foreign ordinary paths keep their full
candidate; this helper does not admit a request or construct a permit.
"""
from pathlib import Path
import runpy
import json
import hashlib
import re
from collections import Counter
from copy import deepcopy

VERSION = "w1-stage2-v1"
MARKER = "w1_stage2_phase"
CONTEXT_FILE = "w1_stage2_phase.g4.json"
COUNT_ROLES = {"actual_authored_cases": "lc", "typed_storage_ceiling": "C_eq",
               "caps_storage_key": "c",
               "combination_slots": "z", "per_authored_case_inputs": "l"}
COUNTS = {
    "L": {"force_per_case": "12*l", "formed_per_case": "12*l",
          "named": "C_eq", "states": "1+C_eq", "ordinary_findings": "C_eq",
          "success_findings": "lc"},
    "E": {"force_per_case": "l", "formed_per_case": "0",
          "named": "0", "states": "1", "ordinary_findings": "0",
          "success_findings": "0"},
}
FAILED = {"route": "L", "cases": 1, "force_per_case": "l",
          "formed_per_case": 0, "named": 0, "states": 1, "findings": 0,
          "held": ["blocked_original", "failed_manifest"],
          "during_T16": ["blocked_original", "failed_manifest", "staged_manifest"]}
INVALIDATION = ("The stage-1 lc-1 preparation-refusal credit is retired. Every authored "
                "route-L case can carry formed member loads; ordinary and successful "
                "formation owners therefore both keep the full authored-case count.")
FG_FILE = "core/product_physics/src/formation_guard.rs"
FG = FG_FILE + ":304:load_row_finding"
DECIDE = FG_FILE + ":223:decide_row"
SOLVE = "core/product_physics/src/lib.rs:5112:solve_prepared_load_case"   # OP1A (§8.31): +9, unchanged text; §8.30 (5080 -> 5103 and 2876 -> 2877, unchanged text); W-16 stage 2 (§8.29: 5072 -> 5080, unchanged text); W-16 slice b/c (§8.27: 5044 -> 5072, unchanged text); model document 0.8.0 (§8.26: 5023 -> 5044, unchanged text); W-16 slice a (§8.23: 4917, unchanged text); W-15 (§8.21: 4910); W-7 (§8.20: 4850; model document 0.7.0: 4845; silent-zero guard: 4775; W-8: 4774; pre-0.4 unification: 4772; O4 S5: 4687; T4-S5 epoch: 4684; final source 588de796a4: 4507; PP: 4500; B7b: 4427; B7a: 4442)
ROW_LOOP = "for row in &rows"
LOOP_RULE = {
    "fn": "formation_guard.rs:304:load_row_finding",
    "loop": r"^for row in \&rows$", "total": "cases*N",
    "why": "SQ2 G5: Ledger.finish(built.nodes.len()*6), private AssembledForce constructors and one guard call per requested case",
}
TOTAL_WHY = "SQ2 G5: B7a exact formed guard owner: full independent generation; <=cases guards, N DOF rows, q<=12*l aggregate escaped source IDs"
TOTALS = {
    239: "2*cases*N*59",
    241: "2*cases*(87*N+(2*N+(10*128+4)*(12*l)))",
    251: "2*cases*(321*N+(2*N+(10*128+4)*(12*l)))",
    322: "2*cases*(N*(600+2+321)+(2*N+(10*128+4)*(12*l)))",
}
FORMAT_ROWS = [
    {"file": FG_FILE, "line": 239, "kind": "format", "template": "range failure ({failure})", "literal_bytes": 16,
     "placeholders": [{"spec": "", "arg": "failure"}]},
    {"file": FG_FILE, "line": 241, "kind": "format", "template": "no conservative formation bound (CannotBound: curved-span consistent load vector from {:?})", "literal_bytes": 87,
     "placeholders": [{"spec": "?", "arg": "row.cannot_bound_sources"}]},
    {"file": FG_FILE, "line": 251, "kind": "format", "template": "net formation defect {net:e}, self-equilibrated defect {self_equilibrated:e}, bound {:e}, intended net at least {:e}, body scale {s_star:e}, threshold {t0:e} (self-equilibrated threshold {tf:e}); formed sources {:?}", "literal_bytes": 153,
     "placeholders": [{"spec": "e", "arg": x} for x in ("net", "self_equilibrated", "row.bound", "row.intended_net_lower", "s_star", "t0", "tf")] + [{"spec": "?", "arg": "row.formed_sources"}]},
    {"file": FG_FILE, "line": 322, "kind": "format", "template": "{}: {reason}", "literal_bytes": 2,
     "placeholders": [{"spec": "", "arg": "dof_label(row.dof)"}, {"spec": "", "arg": "reason"}]},
]
PHASE_NAMES = (
    "W1 ordinary span (W1 not yet run)",
    "W2 G-B, G-C and the W1 phases (T12-T15) with the N1 reserve",
    "W3 publication (T16) with the staged copy",
    "W4 precommit validation (T17) with the successor and the invocation Value",
    "W5 transfer and Direct completion (T18, T19)",
    "X1 ordinary span with T25 (selected finalization)",
    "X2 X completion: retained receipt + reserve + Direct completion",
)
# PU-X3 (pre-0.4 failure unification D3, retained-pricing-rules.md §8.14): on a source whose captured route and Direct
# entry republish a selected source that could not finalize, the republication is its own phase: the second ordinary
# span on the same ledger, after the first run's envelope (reduced to its fail-closed form) and, on the Direct entry,
# the observer dropped. W1 does not start (Coexistence), so no W phase follows it. Earlier sources keep seven phases.
X3_NAME = "X3 republication: the second ordinary span after a selected source could not finalize"


def republishes(snapshot):
    """True when the snapshot's lib.rs defines the republication and its fail-closed form (o10_rules' PU-R1 guards
    their order); False on an earlier source; anything partial stops."""
    if not snapshot:
        return False
    text = (Path(snapshot) / "core/product_physics/src/lib.rs").read_text(encoding="utf-8")
    marks = ("\nfn republication(\n" in text, "\nfn fail_closed_form(" in text)
    if any(marks) and not all(marks):
        raise ValueError("PU-X3: the republication is partly present; re-review the phase model")
    return all(marks)


def phase_names(snapshot):
    return PHASE_NAMES + ((X3_NAME,) if republishes(snapshot) else ())


def adjacent(name):
    return runpy.run_path(str(Path(__file__).with_name(name)))


def inspect(snapshot):
    # This binds all current source producers, the full non-generated
    # admission prefix and sole permit/law bodies, failed custody,
    # same-request preparation/publication and the stage-2 member path.
    proof = adjacent("b7a_scoped_text_model.py")["inspect"](snapshot)
    proof["w1_stage2_phase_model_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    proof["w1_stage2_phase_count_roles"] = deepcopy(COUNT_ROLES)
    reduction = adjacent("w1_reductions_model.py")
    reduced = reduction["inspect"](snapshot)
    proof["w1_reductions"] = {"reduced": reduced,
        "source_sha256": deepcopy(reduction["REDUCED_PINS"] if reduced else reduction["RF_PINS"])}
    proof["w1_member_loads"] = adjacent("w1_member_load_model.py")["inspect"](snapshot)
    return proof


def transform(snapshot, route, loops, text):
    proof = inspect(snapshot)
    if route not in COUNTS:
        raise ValueError("B7a phase: unknown canonical route")
    if MARKER in loops or MARKER in text:
        raise ValueError("B7a phase: duplicate context")
    context = {"version": VERSION, "route": route, "branch": "ordinary",
               "counts": deepcopy(COUNTS[route]),
               "count_roles": deepcopy(COUNT_ROLES),
               "failed_preview": deepcopy(FAILED), "member_loads_preparable": True}
    result_loop, result_text = deepcopy(loops), deepcopy(text)
    result_loop[MARKER] = deepcopy(context)
    result_text[MARKER] = deepcopy(context)
    return result_loop, result_text, proof


def state(snapshot, loops, text, graph=None, formats=None, lexical=None,
          extra_zero="", fn_zero=""):
    inspect(snapshot)
    context = loops.get(MARKER)
    if not isinstance(context, dict):
        raise ValueError("B7a phase: missing canonical context")
    route = context.get("route")
    if route not in COUNTS:
        raise ValueError("B7a phase: unknown canonical route")
    branch = context.get("branch")
    if branch not in ("ordinary", "success"):
        raise ValueError("B7a phase: unknown consumer branch")
    expected = {"version": VERSION, "route": route, "branch": branch,
                "counts": deepcopy(COUNTS[route]),
                "count_roles": deepcopy(COUNT_ROLES),
                "failed_preview": deepcopy(FAILED), "member_loads_preparable": True}
    if context != expected or text.get(MARKER) != expected:
        raise ValueError("B7a phase: context/route/count drift")
    verify_rules(loops, text)
    if any(x is not None for x in (graph, formats, lexical)):
        if any(x is None for x in (graph, formats, lexical)):
            raise ValueError("B7a phase: incomplete consumer inventory")
        verify_inventory(graph, formats, lexical)
        if any(FG_FILE.startswith(x) for x in extra_zero.split(",") if x):
            raise ValueError("B7a phase: excluded formation allocation owner")
        if any(x in (FG, DECIDE, short(FG), short(DECIDE)) for x in fn_zero.split(",") if x):
            raise ValueError("B7a phase: zeroed formation allocation owner")
    return deepcopy(expected)


def short(key):
    return key.split("/src/")[-1]


def verify_rules(loops, text):
    # Keep the exact current total as the preimage. The branch-qualified
    # runtime hook is the only replacement; no competing table alias wins.
    selected = [r for r in loops.get("loop_total", [])
                if r.get("fn") in (FG, short(FG)) and re.search(r["loop"], ROW_LOOP)]
    if selected != [LOOP_RULE]:
        raise ValueError("B7a phase: formation loop-total inventory changed")
    aliases = adjacent("kd5_first_error_model.py")["override_aliases"]
    for line, bound in TOTALS.items():
        canonical = f"formation_guard.rs:{line}"
        if text.get("site_total", {}).get(canonical) != {"bytes": bound, "why": TOTAL_WHY}:
            raise ValueError("B7a phase: formation requested-total preimage changed")
        for family in ("site_zero", "site_from", "site_size", "site_aggregate", "lex_site_size", "site_total"):
            for alias in aliases({"file": FG_FILE, "line": line, "kind": "format"}):
                if family == "site_total" and alias == canonical:
                    continue
                if alias in text.get(family, {}):
                    raise ValueError("B7a phase: competing formation override: " + family + ":" + alias)
    for family in ("fn_zero", "fn_cap"):
        if any(r.get("fn") in (FG, DECIDE, short(FG), short(DECIDE)) for r in loops.get(family, [])):
            raise ValueError("B7a phase: competing formation function rule")
    if any(FG_FILE.startswith(r["file"]) for r in loops.get("file_zero", [])):
        raise ValueError("B7a phase: competing formation file exclusion")
    for family in ("edge_zero", "edge_once", "edge_per_call"):
        if any(r.get("caller") in (FG, DECIDE, short(FG), short(DECIDE)) or
               r.get("callee") in (FG, DECIDE, short(FG), short(DECIDE)) for r in loops.get(family, [])):
            raise ValueError("B7a phase: competing formation call rule")
    if any(re.search(r["outer"], ROW_LOOP) or re.search(r["inner"], ROW_LOOP) for r in loops.get("pairs", [])):
        raise ValueError("B7a phase: competing formation loop pair")


def verify_inventory(graph, formats, lexical):
    edges = graph["edges"]
    for target, expected in ((FG, [SOLVE]), (DECIDE, [FG])):
        parents = [k for k, values in edges.items() for value in values if value == target]
        if target not in edges or Counter(parents) != Counter(expected):
            raise ValueError("B7a phase: qualified formation caller inventory changed")
    for caller, callee, stack in ((SOLVE, FG, [[]]), (FG, DECIDE, [[ROW_LOOP]])):
        rows = [r for r in graph["site_loops"] if r[:2] == [caller, callee]]
        if rows != [[caller, callee, stack]]:
            raise ValueError("B7a phase: formation call-stack inventory changed")
    actual = [r for r in formats if r.get("file") == FG_FILE and r.get("line") in TOTALS]
    if actual != FORMAT_ROWS or any(r.get("file") == FG_FILE and r.get("line") in TOTALS for r in lexical):
        raise ValueError("B7a phase: exact formation format/lexical inventory changed")


def finding_witness(context, counts, caps):
    actual = caps.get("lc", caps["c"])
    ordinary_counts(context, actual, caps["c"], caps["l"])
    if type(counts.get(FG)) is not int or counts[FG] != actual:
        raise ValueError("B7a phase: same-invocation formation calls changed")
    return finding_count(context, caps) * 6 * caps["n"]


def selected_publication_context(snapshot, directory, graph):
    """The three product gates bind only selected T25's publication hash."""
    context = consumer_context(snapshot, directory)
    # B7b snapshot lines (B7a: composite 726/952, source_receipt 694/780/796, lib 2543).
    # T4-U6 snapshot lines, unchanged text (B7b: composite 724/950, lib 2532; source_receipt 646/732/748 before
    # U6-SH's hash_text and DomainPayload, +16). Epoch addbdc030e: composite 731/982 -> 726/977 (physical_source_built
    # calls `exact_pipe_sections`, -5). T4-S5 (epoch 4): composite 977 -> 990, lib 2553 -> 2569, unchanged text.
    # T4-S6 (S6-HS: the streamed `hash`, -9): source_receipt 664/750/766 -> 655/741/757, unchanged text.
    # O4 S5 (§8.9): composite 726/990 -> 740/1018 (the basis-section parameters and the second maximum), lib 2569 -> 2572,
    # source_receipt 655/741/757 -> 657/743/759 (a test re-export), unchanged text. Pre-0.4 failure unification (§8.14):
    # lib 2572 -> 2634 and source_receipt 657/743/759 -> 661/747/763 (`requested`'s doc comment), unchanged text. W-8
    # topology warnings (§8.16): lib 2634 -> 2635 (`mod topology;`), unchanged text. W-15 (§8.21): lib 2705 -> 2750
    # (`mod rigid_component;`, ResultItem's record and BuiltModel's scaled members), unchanged text.
    # W-16 slice a (§8.23): lib 2750 -> 2752 (`mod imposed_motion;` and LoadTargetInput's support arm, less the deleted
    # pre-parse refusal call), unchanged text. W-16 slices b and c (§8.27): composite 740/1018 -> 754/1032
    # (physical_source_built's equivalent-static binding), lib 2867 -> 2868 (`mod exact_combinations;`), unchanged text.
    expected = {
        "core/product_physics/src/source_receipt/composite.rs:754:composite_exact": SOLVE,
        "core/product_physics/src/source_receipt/composite.rs:1032:ordinary_physics": SOLVE,
        "core/product_physics/src/source_receipt.rs:661:failed": SOLVE,
        "core/product_physics/src/source_receipt.rs:747:finalize_composite":
            "core/product_physics/src/lib.rs:2879:run_linear_static_preview_observed_inner",
        "core/product_physics/src/source_receipt.rs:763:finalize_for":
            "core/product_physics/src/source_receipt.rs:747:finalize_composite",
    }
    for target, caller in expected.items():
        parents = [key for key, values in graph["edges"].items() for value in values if value == target]
        if target not in graph["edges"] or parents != [caller]:
            raise ValueError("B7a T25: selected constructor/finalizer caller changed")
        stacks = [row for row in graph["site_loops"] if row[1] == target]
        if stacks != [[caller, target, [[]]]]:
            raise ValueError("B7a T25: selected constructor/finalizer stack changed")
    # inspect() binds raw lib/source_receipt/pressure/capture/admission.
    # source_eligible, every per-case constructor and invocation finalizer
    # require exact captured context derived from the same request. L has
    # no such producer. This never zeroes shared checked/hash/serialized.
    return context


def consumer_context(snapshot, directory):
    directory = Path(directory)
    loops = json.loads((directory / "loop_bounds.g4.json").read_bytes())
    text = json.loads((directory / "text_args.g4.json").read_bytes())
    return state(snapshot, loops, text)


def finding_count(context, caps):
    actual = caps.get("lc", caps["c"])
    storage = caps["c"]
    counts = (successful_counts(context, actual, storage, caps["l"])
              if context["branch"] == "success"
              else ordinary_counts(context, actual, storage, caps["l"]))
    # TEXT's generic ordinary loop uses actual authored cases, retaining
    # every ordinary/fallback call; typed finding storage keeps C_eq.
    return counts["finding_cases"] if context["branch"] == "success" else (0 if context["route"] == "E" else actual)


def finding_loop_total(context, owner, stack, caps, counts):
    if owner != "core/product_physics/src/formation_guard.rs:304:load_row_finding":
        return None
    if "for row in &rows" not in stack:
        return None
    if stack != ["for row in &rows"]:
        raise ValueError("B7a phase: finding traversal nesting changed")
    return finding_witness(context, counts, caps)


def allocation_witness(context, file, line, kind, caps, counts, mult, owner, loops):
    if (file, kind) != ("core/product_physics/src/formation_guard.rs", "format"):
        return None
    if line not in (239, 241, 251, 322):
        return None
    expected = finding_witness(context, counts, caps)
    expected_owner = FG if line == 322 else DECIDE
    expected_loops = [ROW_LOOP] if line == 322 else []
    if owner != expected_owner or loops != expected_loops or mult != expected:
        raise ValueError("B7a phase: effective formation allocation witness changed")
    if counts.get(DECIDE) != expected:
        raise ValueError("B7a phase: effective formation row calls changed")
    return expected


def finding_requested(context, file, line, kind, caps, counts, mult, owner, loops):
    if allocation_witness(context, file, line, kind, caps, counts, mult, owner, loops) is None:
        return None
    count = finding_count(context, caps)
    formed = adjacent("b7a_formation_model.py")
    lengths = formed["lengths"](caps["n"], caps["l"], caps["ident"])
    n, debug = lengths["N"], lengths["debug_total"]
    per_case = {239: n * 59, 241: 87 * n + debug,
                251: 321 * n + debug, 322: lengths["fired_total"]}
    return 2 * count * per_case[line]


def success_tables(snapshot, loops, text):
    context = state(snapshot, loops, text)
    if context["branch"] != "ordinary":
        raise ValueError("B7a phase: successor input is not the full ordinary candidate")
    new_loops, new_text = deepcopy(loops), deepcopy(text)
    context["branch"] = "success"
    new_loops[MARKER] = deepcopy(context)
    new_text[MARKER] = deepcopy(context)
    state(snapshot, new_loops, new_text)
    return new_loops, new_text


def ordinary_counts(context, actual_cases, storage_cases, loads):
    if type(actual_cases) is not int or not 1 <= actual_cases <= 3:
        raise ValueError("B7a phase: requested-case cap changed")
    if type(storage_cases) is not int or not actual_cases <= storage_cases <= 3:
        raise ValueError("B7a phase: typed slot ceiling changed")
    if type(loads) is not int or not 0 <= loads <= 128:
        raise ValueError("B7a phase: per-case authored-input cap changed")
    exact = context["route"] == "E"
    return {
        "cases": actual_cases, "storage_cases": storage_cases,
        "authored_inputs": actual_cases * loads,
        "force_per_case": loads if exact else 12 * loads,
        "formed_per_case": 0 if exact else 12 * loads,
        "named": 0 if exact else storage_cases,
        "states": 1 if exact else 1 + storage_cases,
        "finding_cases": 0 if exact else storage_cases,
    }


def successful_counts(context, actual_cases, storage_cases, loads):
    ordinary = ordinary_counts(context, actual_cases, storage_cases, loads)
    formed_cases = 0 if context["route"] == "E" else actual_cases
    return dict(ordinary, formed_cases=formed_cases, finding_cases=formed_cases,
                force_total=12 * loads * formed_cases + loads * (actual_cases - formed_cases),
                formed_total=12 * loads * formed_cases, storage_cases=storage_cases)


def formed_owner(context, actual_cases, storage_cases, loads, ident, *, owner, branch):
    """Closed formed-only owners; generic headers/wrappers stay with callers."""
    counts = successful_counts(context, actual_cases, storage_cases, loads)
    if branch != context["branch"]:
        raise ValueError("B7a phase: formed-owner context mismatch")
    if ident != 128:
        raise ValueError("B7a phase: identifier premise changed")
    if branch == "ordinary":
        owners = 0 if context["route"] == "E" else storage_cases
    elif branch == "success":
        owners = counts["formed_cases"]
    else:
        raise ValueError("B7a phase: unknown formed-owner branch")
    from g3lib import X, V, B
    wide = adjacent("b7a_domain_model.py")
    q = 12 * loads
    # W1-L1a/b/L2: the source pin covers producer, comparison and consuming paths.
    # Mathematical callers with no snapshot keep the historical owner factory;
    # every production pricing invocation supplies RP_SNAPSHOT.
    import os
    snapshot = os.environ.get("RP_SNAPSHOT")
    reduced = adjacent("w1_reductions_model.py")["inspect"](snapshot) if snapshot else False
    if owner == "assembled_children":
        value = wide["primitive_copy"](q) + wide["manifest_copy"](q) + wide["formation_children"](q)
    elif owner == "typed_source_children":
        # L1a borrows primitives; L1b compares one temporary closure entry.
        value = wide["manifest_copy"](1) if reduced and loads else (
            X() if reduced else wide["primitive_copy"](q) + wide["manifest_copy"](q))
    elif owner == "scaled_force_children":
        value = wide["primitive_copy"](q) + wide["manifest_copy"](q)
    elif owner == "recovery_source_children":
        value = X() if reduced else wide["primitive_copy"](q)
    elif owner == "uniform_constructor":
        requested, _ = wide["formation_constructor"]()
        # L2: one straight constructor call and one attach-last staging term
        # coexist at most; moved returned rows remain separately resident.
        value = (wide["formed_constructor"](1) + requested if loads else X()) if reduced else (
            wide["formed_constructor"](q) + requested * loads)
    elif owner == "element_application":
        value = V("ElementUniformLoadContribution", loads) + B(loads * ident)
    else:
        raise ValueError("B7a phase: unknown formed-only owner")
    if any(type(v) is not int or v < 0 or not isinstance(k, str) for k,v in value.items()):
        raise ValueError("B7a phase: noninteger/negative producer coefficient")
    # Later C_eq multiplication preserves >= owners*value independently
    # for every atom. It adds rounding slack; no coefficient rounds down.
    result = {k: (v * owners + storage_cases - 1) // storage_cases for k,v in value.items()}
    if any(storage_cases * result[k] < owners * v for k,v in value.items()):
        raise ValueError("B7a phase: coefficient rounding failed")
    return X(result)


def failed_preview_counts(context, actual_cases, storage_cases, loads, *, custody):
    ordinary_counts(context, actual_cases, storage_cases, loads)
    expected = {"route": "Preview", "requested_cases": 1, "cases_seen": 1,
                "attempted": [0], "combinations": 0, "basis_expected": False,
                "bases": 0, "source_some": True}
    if (context["route"] != "L" or actual_cases != 1 or custody != expected
            or type(storage_cases) is not int or not 1 <= storage_cases <= 3):
        raise ValueError("B7a phase: incompatible failed Preview branch")
    # Actual failed_custody has one Preview/base/nodal case. Keep both
    # original and manifest owners; T16 separately keeps its staged clone.
    return dict(FAILED, authored_inputs=loads, force_per_case=loads,
                storage_cases=storage_cases)

def run_success_branch(work, snapshot, edges, lexicon, eps, caps):
    """Run the existing consumers in a separate, guarded success context."""
    import ast
    import contextlib
    import os
    import shutil
    import subprocess
    import sys
    work = Path(work)
    target = work / "success"
    if target.exists():
        raise ValueError("B7a phase: success work already exists")
    target.mkdir()
    for source in work.iterdir():
        if source.is_file() and (source.suffix in (".py", ".json") or source.name=="crate_dirs.txt"):
            shutil.copyfile(source, target/source.name)
    loop = json.loads((target/"loop_bounds.g4.json").read_bytes())
    text = json.loads((target/"text_args.g4.json").read_bytes())
    loop,text = success_tables(snapshot,loop,text)
    (target/"loop_bounds.g4.json").write_text(json.dumps(loop,indent=1)+"\n")
    (target/"text_args.g4.json").write_text(json.dumps(text,indent=1)+"\n")
    env = dict(os.environ,G4_CAPS=caps,RP_SNAPSHOT=snapshot,RP_STAGE1_EDGES=edges)
    for key in ("TB_FN_ZERO","TB_EXTRA_ZERO"):
        env.pop(key,None)
    def run(args,out,extra=None,cwd=None):
        with (target/out).open("w") as stdout,(target/(out+".err")).open("w") as stderr:
            result=subprocess.run([sys.executable]+[str(a) for a in args],
                cwd=cwd or snapshot,env=dict(env,**(extra or {})),stdout=stdout,stderr=stderr)
        if result.returncode:
            raise ValueError("B7a phase: success consumer failed: "+out)
        if (target/(out+".err")).read_bytes():
            raise ValueError("B7a phase: success consumer stderr: "+out)
    run([target/"composite_text.py",snapshot,"caps"],"composite_text.caps.json")
    # Reuse the exact canonical variant function and all ZW additions.
    source=ast.parse((target/"sens.py").read_bytes())
    selected=[]
    for node in source.body:
        if isinstance(node,(ast.Assign,ast.AugAssign)):
            nodes=node.targets if isinstance(node,ast.Assign) else [node.target]
            if any(isinstance(n,ast.Name) and n.id in ("ROOT","ZW","ENVZERO") for n in nodes):
                selected.append(node)
        if isinstance(node,ast.FunctionDef) and node.name=="tb":
            selected.append(node)
    namespace=dict(os=os,json=json,work=str(target),snap=snapshot,edges=edges,
                   lexicon=lexicon,caps=caps,run=run)
    exec(compile(ast.Module(body=selected,type_ignores=[]),str(target/"sens.py"),"exec"),namespace)
    d=25544*json.loads(caps).get("c",1)
    for _ in range(12):
        newer=namespace["tb"]("",d)["D_diagnostics"]
        if newer==d:
            break
        d=newer
    else:
        raise ValueError("B7a phase: success diagnostic fixpoint did not converge")
    for variant in ("","_env","_X","_W"):
        result=namespace["tb"](variant,d)
        if not result["complete"] or result["unmapped_loop_headers"] or result["unclassified_args"]:
            raise ValueError("B7a phase: incomplete success TEXT: "+variant)
    run([target/"t08_closure.py","caps"],"t08_closure.caps.log")
    run([target/"producer_caps.py","caps"],"producer_caps.caps.out.json")
    run([target/"t07_repair.py","caps"],"t07_repair.caps.out.json")
    run([target/"t25_g4.py","caps",eps],"t25_g4.caps.eps"+eps+".out.json")
    shutil.copyfile(target/("t25_g4.caps.eps"+eps+".out.json"),target/"t25_caps.caps.out.json")
    run([target/"ordinary_caps.py","caps"],"ordinary_caps.caps.out.json")
    run([target/"g4_caps.py","caps",eps,snapshot],"g4_caps.caps.eps"+eps+".out.json",
        {"PROFILE_TREE":str(target/"profile_tree.json")})
    return target

def merge_success_tree(work, successor_work, snapshot):
    """Select success-only phases; keep full ordinary/fallback phase forms."""
    work, successor_work = Path(work), Path(successor_work)
    original = json.loads((work/"profile_tree.json").read_bytes())
    successor = json.loads((successor_work/"profile_tree.json").read_bytes())
    if any(original[k] != successor[k] for k in ("which","EPS","M","R")):
        raise ValueError("B7a phase: branch policy differs")
    if {k:v for k,v in original["text_atoms"].items() if k.startswith("Text(")} != {
            k:v for k,v in successor["text_atoms"].items() if k.startswith("Text(")}:
        raise ValueError("B7a phase: branch TEXT binder differs")
    if original["assumed"] != successor["assumed"]:
        raise ValueError("B7a phase: shared illustrative atom differs")
    if original.get("L") != successor.get("L"):
        raise ValueError("B7a phase: shared spelling maxima differ")
    if set(original["phases"]) != {"sparse", "dense"} or set(successor["phases"]) != {"sparse", "dense"}:
        raise ValueError("B7a phase: native mode inventory changed")
    for mode in ("sparse", "dense"):
        original_phases, successor_phases = original["phases"][mode], successor["phases"][mode]
        names = set(phase_names(snapshot))
        if set(original_phases) != names or set(successor_phases) != names:
            raise ValueError("B7a phase: the phase identities changed (fourteen; sixteen with PU-X3)")
        if any(set(v) != {"requested", "moving"} for v in list(original_phases.values()) + list(successor_phases.values())):
            raise ValueError("B7a phase: requested/moving shape changed")
    names = set(successor["forms"]) | set(successor["exprs"])
    if any(n.startswith("success__") for n in set(original["forms"]) | set(original["exprs"]) | names):
        raise ValueError("B7a phase: branch name collision")
    def renamed(node):
        if isinstance(node,str):
            if node not in names:
                raise ValueError("B7a phase: unknown branch expression")
            return "success__"+node
        if set(node) not in ({"sum"},{"max"}):
            raise ValueError("B7a phase: unknown expression operator")
        return {key:[renamed(x) for x in value] for key,value in node.items()}
    result = deepcopy(original)
    result["forms"].update({"success__"+k:v for k,v in successor["forms"].items()})
    result["exprs"].update({"success__"+k:renamed(v) for k,v in successor["exprs"].items()})
    for mode, phases in result["phases"].items():
        for name in phases:
            if name.split(" ")[0] in ("W3","W4","W5"):
                phase=successor["phases"][mode][name]
                phases[name]={key:[renamed(v) for v in values] for key,values in phase.items()}
    # Full ordinary TAV_W/TAV_X forms and text atoms remain untouched.
    # Runtime's form-name-based upward 5% reserve consequently continues
    # to use the full bounds, even when a selected phase uses success__.
    # The standard evaluator recomputes the illustrative check. Actual native
    # qualification remains a later independent operation.
    import ast,re
    evaluator=Path(snapshot)/"tools/retained_pricing/lib/tree_eval.py"
    if not evaluator.exists():
        raise ValueError("B7a phase: canonical evaluator unavailable")
    api={"json":json,"re":re}
    source=ast.parse(evaluator.read_bytes())
    exec(compile(ast.Module(body=[n for n in source.body if isinstance(n,ast.FunctionDef)],
        type_ignores=[]),str(evaluator),"exec"),api)
    atoms=dict(result["assumed"]);atoms.update(result["text_atoms"])
    _,phases=api["ev_tree"](result,atoms)
    result["check"]={}
    report_path=work/("g4_caps.caps.eps"+str(result["EPS"])+".out.json")
    report=json.loads(report_path.read_bytes())
    for mode,values in phases.items():
        maximum=max(values,key=lambda name:sum(values[name]))
        result["check"][mode]={"max":sum(values[maximum])+result["R"],"phase":maximum}
        actual={}
        for full_name in result["phases"][mode]:
            requested,moving=values[full_name.split(" ")[0]]
            required=requested+moving+result["R"]
            actual[full_name]={"requested":requested,"moving_extra":moving,
                              "E_mov":requested+moving,"E_mov_plus_R":required,
                              "fraction_of_M":round(required/result["M"],4)}
        worst=max(actual,key=lambda name:actual[name]["E_mov_plus_R"])
        report["modes"][mode]["phases"]=actual
        report["modes"][mode]["admission_max"]={"phase":worst,**actual[worst]}
        report["modes"][mode]["margin_rule_0.9M"]=actual[worst]["E_mov_plus_R"]<=9*result["M"]//10
        report["modes"][mode]["fits_M"]=actual[worst]["E_mov_plus_R"]<=result["M"]
    report["b7a_success_context"]={"work":str(successor_work),"selected_phases":["W3","W4","W5"],
        "ordinary_fallback_phases_preserved":True,"failed_storage_and_copy_charges_preserved":True}
    report_path.write_text(json.dumps(report,indent=1)+"\n")
    (work/"profile_tree.json").write_text(json.dumps(result,indent=1,sort_keys=True)+"\n")
    return result
