"""CP3 review static checks (read-only). Run from WORKING_ROOT with a Python that has PyYAML.
1. Carrier schema pointer diff 404cd9c6b -> 485cc2ed0 (only list appends expected).
2. results.v0.3 ResultEnvelope oneOf branches: each pins producer id and profile.
3. Semantic-table diffs: physics-source-1 -> load-reference-source-1, physics-1 -> load-reference-1; old tables unchanged.
4. sha256 of cited records."""
import hashlib, json, subprocess, yaml, pathlib
def show(rev, path):
    return subprocess.run(['git', 'show', f'{rev}:projects/chirality-piping/{path}'], capture_output=True, text=True, check=True).stdout
def load(rev, path):
    t = show(rev, path); return yaml.safe_load(t) if path.endswith('yaml') else json.loads(t)
def diff(a, b, p=''):
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        out += [('REMOVED', p + '/' + k) for k in a if k not in b]
        out += [('ADDED_KEY', p + '/' + k) for k in b if k not in a]
        for k in a:
            if k in b: out += diff(a[k], b[k], p + '/' + k)
    elif isinstance(a, list) and isinstance(b, list):
        if len(b) < len(a): out.append(('LIST_SHRUNK', p))
        for i, (x, y) in enumerate(zip(a, b)): out += diff(x, y, p + f'/{i}')
        if len(b) > len(a): out.append(('LIST_APPEND', p, len(a), len(b)))
    elif a != b: out.append(('CHANGED', p))
    return out
print('== 1 carrier schema pointer diffs')
for f in ['schemas/results.v0.3.schema.yaml', 'schemas/analysis_run.v0.3.schema.json', 'schemas/stress_neutral_export.v0.3.schema.json']:
    d = diff(load('404cd9c6b', f), load('485cc2ed0', f)); print(f, len(d))
    for x in d: print('  ', x)
print('== 2 results.v0.3 ResultEnvelope branches')
s = load('485cc2ed0', 'schemas/results.v0.3.schema.yaml')['$defs']['ResultEnvelope']
print('required producer/formulation_basis:', 'producer' in s['required'], 'formulation_basis' in s['required'])
for i, b in enumerate(s['oneOf']):
    pr = b.get('properties', {})
    print('  ', i, pr['producer']['properties']['semantic_contract_id'], pr['formulation_basis']['properties']['profile_id'], 'contract_evidence', pr.get('contract_evidence'))
print('== 3 semantic tables')
for f in ['physics_1', 'physics_source_1', 'precision_1', 'source_blocks_1']:
    p = f'fixtures/results/semantic_contract_v0_3_{f}.json'
    print(f, 'unchanged' if show('404cd9c6b', p) == show('485cc2ed0', p) else 'CHANGED')
a = json.load(open('fixtures/results/semantic_contract_v0_3_physics_source_1.json'))
b = json.load(open('fixtures/results/semantic_contract_v0_3_load_reference_source_1.json'))
print('physics-source-1 -> load-reference-source-1:', [x[1] for x in diff(a, b)])
print('==4 record hashes')
L = 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION/'
for f in ['CP2_WIRE.md', 'CP2_WIRE_ADDENDUM_1.md', 'CP2_WIRE_ADDENDUM_2.md', 'CP3_WIRE_ADDENDUM.md', 'CP3_READERS/RETURN.md', 'CP3_READERS/BRIEF.md', 'CP3_SCHEMAS/BRIEF.md', 'CP2_RUNTIME_TESTS/EXTENSION_1/BRIEF.md', 'CP2_RUNTIME_TESTS/EXTENSION_1/RETURN.md', 'REVIEW_CHECKPOINT_2/RETURN.md', '_run_records/session2/CP3_ROOT_SELECTIONS.json']:
    print(hashlib.sha256(open(L + f, 'rb').read()).hexdigest(), f)
for f in ['schemas/load_reference_state.schema.json', 'schemas/results.v0.3.schema.yaml', 'schemas/analysis_run.v0.3.schema.json', 'schemas/stress_neutral_export.v0.3.schema.json', 'fixtures/results/semantic_contract_v0_3_load_reference_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_source_1.json', 'core/handoff/stress_neutral/package_v0_3.py', 'core/product_physics/tests/load_reference_state_runtime_extension.rs', 'core/product_physics/tests/load_reference_state_runtime.rs']:
    print(hashlib.sha256(open(f, 'rb').read()).hexdigest(), f)
