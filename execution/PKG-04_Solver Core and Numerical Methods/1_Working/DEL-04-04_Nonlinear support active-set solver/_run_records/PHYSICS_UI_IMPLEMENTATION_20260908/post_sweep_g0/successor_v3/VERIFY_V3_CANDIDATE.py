import base64, hashlib, json, re, sys
from pathlib import Path

envelope_path, output_path = map(Path, sys.argv[1:])
repo = Path.cwd()
envelope = json.loads(envelope_path.read_text())
patch = base64.b64decode(envelope['bytes_base64'])
assert hashlib.sha256(patch).hexdigest() == envelope['decoded_sha256']
lines = patch.decode().splitlines(keepends=True)
files = []
i = 0
while i < len(lines):
    assert lines[i].startswith('--- a/')
    old_path = lines[i][6:].rstrip('\n')
    i += 1
    assert lines[i].startswith('+++ b/')
    new_path = lines[i][6:].rstrip('\n')
    assert old_path == new_path
    i += 1
    hunks = []
    while i < len(lines) and not lines[i].startswith('--- a/'):
        match = re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@', lines[i])
        assert match, lines[i]
        header = tuple(int(value) if value is not None else 1 for value in match.groups())
        i += 1
        body = []
        while i < len(lines) and not lines[i].startswith('@@ ') and not lines[i].startswith('--- a/'):
            assert lines[i][0] in ' +-'
            body.append(lines[i])
            i += 1
        hunks.append((header, body))
    files.append((old_path, hunks))

allowed = {
    'projects/chirality-piping/core/product_physics/src/lib.rs',
    'projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json',
    'projects/chirality-piping/tests/product_preview/test_product_preview_service.py',
    'projects/chirality-piping/apps/desktop/src/services/previewService.test.ts',
}
assert {name for name, _ in files} == allowed
expected = {
    'projects/chirality-piping/core/product_physics/src/lib.rs': 'f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5',
    'projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json': 'e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13',
    'projects/chirality-piping/tests/product_preview/test_product_preview_service.py': '8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735',
    'projects/chirality-piping/apps/desktop/src/services/previewService.test.ts': '02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f',
}
post = {}
for name, hunks in files:
    source = (repo / name).read_text().splitlines(keepends=True)
    out = []
    cursor = 0
    for (old_start, old_count, new_start, new_count), body in hunks:
        assert old_start - 1 >= cursor
        out.extend(source[cursor:old_start - 1])
        cursor = old_start - 1
        seen_old = seen_new = 0
        for line in body:
            payload = line[1:]
            if line[0] == ' ':
                assert source[cursor] == payload
                out.append(payload); cursor += 1; seen_old += 1; seen_new += 1
            elif line[0] == '-':
                assert source[cursor] == payload
                cursor += 1; seen_old += 1
            else:
                out.append(payload); seen_new += 1
        assert (seen_old, seen_new) == (old_count, new_count)
    out.extend(source[cursor:])
    body = ''.join(out)
    digest = hashlib.sha256(body.encode()).hexdigest()
    assert digest == expected[name], (name, digest)
    post[name] = body

rust_path = 'projects/chirality-piping/core/product_physics/src/lib.rs'
pre = (repo / rust_path).read_text()
rust = post[rust_path]
start_token = '    fn valid_invented_model_exposes_nonlinear_support_loop_evidence()'
end_token = '    fn two_node_nonlinear_preview_request('
pre_start, pre_end = pre.index(start_token), pre.index(end_token, pre.index(start_token))
post_start, post_end = rust.index(start_token), rust.index(end_token, rust.index(start_token))
assert pre[:pre_start] == rust[:post_start]
assert pre[pre_end:] == rust[post_end:]
block = rust[post_start:post_end]
for token in (
    'let friction_reaction = result_value(',
    'assert_eq!(friction_reaction, 0.489527);',
    'assert_eq!(normal_evidence.value, 48.952719);',
    'assert_eq!(friction_reaction, round6(0.01 * normal_evidence.value));',
    'result:nonlinear-support:iteration-count',
    'result:nonlinear-support:final-residual-count',
    'result:nonlinear-support:converged-flag',
    'result:nonlinear-support:support-NL-130-FRIC:state-code',
    'nonlinear_support_friction_normal_reaction_derived',
    'derived_support_reaction',
    'source_ref=support:S-130',
    'source_dof=uy',
    'NONLINEAR_SUPPORT_STATE_REVIEW',
    'NONLINEAR_SUPPORT_LOOP_CONVERGED',
    'NONLINEAR_SUPPORT_LOOP_BLOCKED',
):
    assert block.count(token) >= 1, token
for stale in ('0.490101', '48.952652'):
    assert stale not in block
result = {
    'status': 'PASS',
    'decoded_patch_sha256': envelope['decoded_sha256'],
    'applied_in_memory': True,
    'allowed_paths_exact': True,
    'candidate_post_sha256': expected,
    'rust_change_confined_to_affected_test': True,
    'required_rust_assertions_present_exactly_once': all(block.count(token) == 1 for token in (
        'assert_eq!(friction_reaction, 0.489527);',
        'assert_eq!(normal_evidence.value, 48.952719);',
        'assert_eq!(friction_reaction, round6(0.01 * normal_evidence.value));',
    )),
    'stale_rust_literals_absent_from_affected_test': True,
    'old_state_metadata_diagnostic_assertions_retained': True,
}
with output_path.open('w', encoding='utf-8', newline='\n') as f:
    json.dump(result, f, indent=2, sort_keys=True)
    f.write('\n')
