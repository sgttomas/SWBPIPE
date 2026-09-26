"""T1_WP3_NATIVE_INPUTS mutants. Run against a scratch copy (git archive of the
base plus this change), never in place:

    python run_mutants.py <scratch>/projects/chirality-piping/apps/desktop <out.json>

Each mutant is one exact string replacement (the source must contain it exactly
once). A mutant is killed when any selected vitest file fails. The source file
is restored after every mutant.
"""
import json
import os
import subprocess
import sys
import time

INPUTS = "src/features/load-cases/LoadReferenceStateInputs.tsx"
LAWS = "src/features/material-temperature/ExpansionLawsEditor.tsx"
SHARED = "src/features/load-cases/loadStateAuthoring.tsx"
BLOCK = "src/features/solve/LoadReferenceStatesBlock.tsx"
T_INPUTS = "src/features/load-cases/LoadReferenceStateInputs.test.tsx"
T_BLOCK = "src/features/solve/LoadReferenceStatesBlock.test.tsx"
T_GUARD = "src/features/solve/LoadReferenceStatesBlock.joinedGuard.test.tsx"
MANAGER = "src/features/load-cases/LoadCaseManagerPanel.tsx"
SOLVE = "src/features/solve/SolvePanel.tsx"
T_FIXTURES = "src/features/load-cases/loadStateFixtures.test.tsx"

MUTANTS = [
    # Pre-0.4 gating removed.
    ("G1 gating removed (load-case inputs)", INPUTS,
     '  if (!isLoadStateModel(props.model)) return <LoadStateNeeds040 testId="load-state-needs-040" />;\n', "", [T_INPUTS]),
    ("G2 gating removed (expansion laws)", LAWS,
     '  if (!isLoadStateModel(props.model)) return <LoadStateNeeds040 testId="expansion-laws-needs-040" />;\n', "", [T_INPUTS]),
    ("G3 version predicate always true", SHARED,
     "  return model.schema_version === LOAD_STATE_MODEL_VERSION;", "  return true;", [T_INPUTS]),
    # Not-Current labelling of joined and sensitive results removed.
    ("L1 joined route label loses needs-recompute/not-Current", BLOCK,
     "    ? `joined load-reference-source-1; ${NEEDS_RECOMPUTE_NOT_CURRENT} in T1`", '    ? "joined load-reference-source-1"', [T_BLOCK, T_GUARD]),
    ("L2 case label ignores route and standing (joined labelled from solve quality)", BLOCK,
     'const label = !joined && standing.eligible && published?.solve_quality === "checks_passed" ? INTEGRITY_CHECKED : NEEDS_RECOMPUTE_NOT_CURRENT;',
     'const label = published?.solve_quality === "checks_passed" ? INTEGRITY_CHECKED : NEEDS_RECOMPUTE_NOT_CURRENT;', [T_BLOCK, T_GUARD]),
    ("L3 sensitive accepted as integrity checked", BLOCK,
     'const label = !joined && standing.eligible && published?.solve_quality === "checks_passed" ? INTEGRITY_CHECKED : NEEDS_RECOMPUTE_NOT_CURRENT;',
     'const label = !joined && (published?.solve_quality === "checks_passed" || published?.solve_quality === "sensitive") ? INTEGRITY_CHECKED : NEEDS_RECOMPUTE_NOT_CURRENT;', [T_BLOCK, T_GUARD]),
    ("L4 joined guard removed (WP2 standing is never eligible for joined; killed by the stubbed-standing guard test)", BLOCK,
     'const label = !joined && standing.eligible &&', 'const label = standing.eligible &&', [T_BLOCK, T_GUARD]),
    ("L5 values shown without reader admission", BLOCK,
     '    {admitted.state === "admitted" && <AdmittedStates', '    {admitted.state !== "reading" && <AdmittedStates', [T_BLOCK, T_GUARD]),
    # One operation submission removed (each field group).
    ("S1 reference-configuration submission removed", INPUTS,
     'onClick={() => void state.submit(spec, current, () => draft === undefined ? NOT_PRESENT : toPayload(draft))}>Queue reference configurations',
     'onClick={() => undefined}>Queue reference configurations', [T_INPUTS]),
    ("S2 analysis-state submission removed", INPUTS,
     'onClick={() => void state.submit(spec, current, () => toPayload(draft))}>Queue analysis state',
     'onClick={() => undefined}>Queue analysis state', [T_INPUTS]),
    ("S3 expansion-law submission removed", LAWS,
     'onClick={() => void state.submit(spec, current, () => draft === undefined ? NOT_PRESENT : toPayload(draft))}>Queue expansion laws',
     'onClick={() => undefined}>Queue expansion laws', [T_INPUTS]),
    ("S4 queue hand-off removed", SHARED,
     "      current.onQueueIntent(intent);\n", "", [T_INPUTS]),
    ("S5 engine pre-check bypassed (refusals would be queued)", SHARED,
     "      if (blocking.length) {", "      if (false) {", [T_INPUTS]),
    ("S6 removal sends a JSON string instead of not_present", SHARED,
     "  if (after === NOT_PRESENT) intent.change = { ...intent.change, after: NOT_PRESENT };\n", "", [T_INPUTS]),
    # Addendum: tolerant rendering of headlessly authored 0.4.0 models (guards reverted).
    ("A1 absent label/kind/status guard reverted (LoadCaseManagerPanel)", MANAGER,
     '  const value: unknown = loadCase[field];\n  return typeof value === "string" ? value : "";', "  return loadCase[field];", [T_FIXTURES]),
    ("A2 absent model.diagnostics guard reverted (SolvePanel readiness)", SOLVE,
     "  const diagnostics = [...(model.diagnostics ?? []), ...(result?.diagnostics ?? [])];", "  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])];", [T_FIXTURES, T_BLOCK]),
    ("A3 absent model.diagnostics guard reverted (SolvePanel job packet)", SOLVE,
     "  return [...(model.diagnostics ?? []), ...(result?.diagnostics ?? [])];", "  return [...model.diagnostics, ...(result?.diagnostics ?? [])];", [T_FIXTURES, T_BLOCK]),
    ("A4 delete before-value no longer mirrors the engine's TBD for absent fields", MANAGER,
     '  return typeof value === "string" ? value : "TBD";', "  return String(value);", [T_FIXTURES]),
]


def main() -> None:
    root, out = sys.argv[1], sys.argv[2]
    env = dict(os.environ)
    results = []
    for name, rel, old, new, tests in MUTANTS:
        path = os.path.join(root, rel)
        with open(path, encoding="utf-8") as handle:
            source = handle.read()
        if source.count(old) != 1:
            raise SystemExit(f"{name}: expected one site in {rel}, found {source.count(old)}")
        try:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(source.replace(old, new))
            started = time.time()
            run = subprocess.run(["npx", "vitest", "run", *tests], cwd=root, env=env, capture_output=True, text=True, timeout=900)
            summary = [line for line in run.stdout.splitlines() if "Tests " in line or "Test Files " in line]
            status = "killed" if run.returncode != 0 else "survived"
        finally:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(source)
        results.append({"mutant": name, "file": rel, "tests": tests, "status": status, "summary": summary, "seconds": round(time.time() - started, 1)})
        print(f"{status:8} {name} {summary}", flush=True)
    with open(out, "w", encoding="utf-8") as handle:
        json.dump(results, handle, indent=2)


if __name__ == "__main__":
    main()
