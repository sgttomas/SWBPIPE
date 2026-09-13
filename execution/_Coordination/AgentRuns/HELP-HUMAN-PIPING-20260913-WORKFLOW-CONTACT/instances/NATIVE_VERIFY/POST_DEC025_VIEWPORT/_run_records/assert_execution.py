from pathlib import Path
import hashlib, json, math, subprocess

packet = Path(__file__).parent.parent
raw = packet / '_run_records'
run = packet.parents[2]
repo = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
source = 'd51169a2c89e98d49175c23dea2c4b78d55ea8be'
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == source
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def load(name): return json.loads((raw / (name + '.json')).read_text())
def model(row): return json.loads(row['model_json'])
def modelhash(row): return json.loads(row['model_hash_json'])['value']
def ax(name): return (raw / (name + '-ax.txt')).read_text()
binding = json.loads((run / 'INTEGRATED_CANDIDATE_BINDING_V3.json').read_text())
for group in ['code_paths', 'doc_paths']:
    for file, expected in binding[group].items(): assert digest(repo / file) == expected, file
build = json.loads((packet / 'FRESH_BUILD_BINDING.json').read_text())
bundle = repo / build['bundle_repo_relative']
dist = repo / 'projects/chirality-piping/apps/desktop/dist'
def tree(root): return {str(f.relative_to(root)): digest(f) for f in sorted(root.rglob('*')) if f.is_file()}
assert tree(bundle) == build['bundle_files']
assert tree(dist) == build['dist_files']
binary = str((repo / build['binary_repo_relative']).resolve())
processes = [line for line in subprocess.check_output(['ps', '-axo', 'pid,lstart,command'], text=True).splitlines() if line.strip().endswith(binary)]
assert not processes, processes
(raw / 'after-final-close-process-absence.txt').write_text('Exact isolated bundled executable absent after actual final GUI close.\n')
store = Path.home() / 'Library/Application Support/org.openpipestress.technical-preview.workflow-contact-20260913/openpipestress-projects.sqlite3'
identity = {'isolated_store_raw_path': str(store), 'isolated_store_sha256_at_final_close': digest(store), 'binary_raw_path': binary, 'binary_sha256': build['binary_sha256'], 'initial_process': (raw / 'initial-process.txt').read_text().strip(), 'fresh_reopened_process': (raw / 'fresh-reopened-process.txt').read_text().strip(), 'actual_quit_process_absence_record': 'after-quit-process-absence.txt', 'final_process_absent': True, 'normal_user_store_access': False, 'daemon_operations': False}
(raw / 'STORE_PROCESS_IDENTITY.json').write_text(json.dumps(identity, indent=2) + '\n')

old = json.loads((run / 'instances/NATIVE_VERIFY/FINAL_V2/_run_records/gui-resolved500.json').read_text())
firstbefore, firstafter, solved, reduced, blocked, undo, recovery, reopenedbefore, reopenedafter = [load(n) for n in ['initial-historical-before-save', 'initial-historical-after-save', 'initial-solved500', 'support-reduced-before-solve', 'blocked-native', 'undo-restored500', 'recovery-solved500', 'fresh-reopen-before-save', 'fresh-reopen-after-save']]
hash500 = 'sha256:6c909e3114442b2fbbc78c44c98b1bf41520f4c0fa201c3813e046f59de8d0bf'
hashedited = 'sha256:ec3e6eb0d9dfb4063bc1efb0252a4b0b673226b01afa73031f6155d07fde2fe9'
for row in [firstbefore, firstafter, solved, undo, recovery, reopenedbefore, reopenedafter]:
    assert row['model_json'] == old['model_json']
    assert modelhash(row) == hash500
expectededit = model(old)
expectededit['supports'][0]['restraints'] = ['UX', 'UY', 'UZ']
for row in [reduced, blocked]:
    assert model(row) == expectededit
    assert modelhash(row) == hashedited
fields = ['model_json', 'mechanics_result_json', 'analysis_run_json', 'model_hash_json', 'project_envelope_hash_json', 'editor_intents_json', 'proposal_json', 'selected_review_target_json']
firstpreserve = {key: old[key] == firstbefore[key] == firstafter[key] for key in fields}
reopenpreserve = {key: recovery[key] == reopenedbefore[key] == reopenedafter[key] for key in fields}
assert all(firstpreserve.values()) and all(reopenpreserve.values())
assert old['mechanics_result_json'] == solved['mechanics_result_json'] == recovery['mechanics_result_json']
for name in ['initial-historical-results', 'fresh-reopened-historical']:
    text = ax(name)
    assert 'Historical saved run' in text and 'HISTORICAL_INPUT_MANIFEST_MISSING' in text and 'HASH_MISMATCH' not in text
    assert 'Deformation · unavailable' in text and 'analysis_run=not generated' in text and 'result_rows=0' in text
historicalreport = ax('fresh-reopened-historical-report')
assert 'Rendered calculation report' not in historicalreport and 'Render report' not in historicalreport
blockedresult = json.loads(blocked['mechanics_result_json'])
assert blockedresult['status']['mechanics'] == 'MODEL_INCOMPLETE' and not blockedresult['results']
diagnostic = next(x for x in blockedresult['diagnostics'] if x['code'] == 'SOLVER_SYSTEM_BLOCKED')
assert diagnostic['id'] == 'diagnostic:physics:under-restrained'
assert 'RX,RY,RZ' in diagnostic['message']
blockedtext = ax('blocked-viewport-diagnostics')
assert 'Deformation · blocked' in blockedtext and 'blocked; mechanics=model incomplete; rows=0' in blockedtext
assert 'scale=not_generated; professional_claim=false' in blockedtext and 'SOLVER_SYSTEM_BLOCKED' in blockedtext
assert 'not started' not in blockedtext
blockedreport = ax('blocked-report')
assert 'button (disabled) Render report' in blockedreport and 'button (disabled) Save Report Package' in blockedreport
recoverytext = ax('recovery-solved500')
assert 'Deformation · normalized' in recoverytext and 'available; nodes=' in recoverytext
assert 'seam=tauri backend job' in recoverytext and hash500 in recoverytext and 'job=backend-solve-job-3' in recoverytext
recoveryreport = ax('recovery-solved500-report')
assert 'button Render report' in recoveryreport and 'button Save Report Package' in recoveryreport

I = math.pi / 64 * (.168**4 - (.168 - 2 * .007)**4)
assert I == 1.1493591335983794e-05
checks = []
for name, row in [('initial-solved500', solved), ('recovery-solved500', recovery)]:
    result = json.loads(row['mechanics_result_json'])
    assert result['status']['mechanics'] == 'MECHANICS_SOLVED' and len(result['results']) == 67
    lookup = {x['id']: x for x in result['results']}
    expectations = [('result:disp:node-WF-TIP:uy', 500 * 3.2**3 / (3 * 200e9 * I) * 1000, 'mm'), ('result:disp:node-WF-TIP:rz', 500 * 3.2**2 / (2 * 200e9 * I), 'rad'), ('result:reaction:support-WF-ROOT', 500, 'N'), ('result:force:pipe-WF:shear-z', 500, 'N'), ('result:moment:pipe-WF:bending-y', -1600, 'N*m')]
    for rowid, expected, unit in expectations:
        actual = lookup[rowid]
        assert actual['unit'] == unit and actual['basis_ref'] == {'ref_id': 'load:WF', 'ref_type': 'load_case'}
        error = abs(actual['value'] - expected)
        assert error <= .5e-6 + 1e-10
        checks.append({'snapshot': name, 'row_id': rowid, 'actual': actual['value'], 'expected': expected, 'unit': unit, 'absolute_error': error, 'allowance': .5e-6 + 1e-10, 'passed': True})
    assert lookup['result:force:pipe-WF:shear-z']['metadata']['location'] == 'end_i'
    assert lookup['result:moment:pipe-WF:bending-y']['metadata']['location'] == 'end_i'
    text = ax(name)
    assert 'input manifest sha256=58ae395f84e13f711365857329952e29ac1c88e15bc91cac05f6433b5e91f2c6' in text
    assert all('dimension' not in x for x in result['results'])
fixture = load('fresh-packaged-selftest')
assert fixture['hashes']['edit'] == fixture['hashes']['equivalent_agent'] == fixture['hashes']['redo_checkpoint']
final = run / 'instances/NATIVE_VERIFY/FINAL_V2'
oldmanifest = json.loads((final / 'FINAL_EVIDENCE_MANIFEST.json').read_text())
for key, expected in oldmanifest['files'].items(): assert digest(final / key) == expected
output = {'source_sha': source, 'gate_verdict': 'bounded_post_DEC025_native_viewport_recheck_passed_pending_parent_review_full_sweep', 'derivative_evidence_only': True, 'source_12_code_8_docs_and_full_bundle_dist_sets_match_after_GUI': True, 'prior_FINAL_V2_93_files_byte_exact': True, 'actual_GUI500_model_hash': hash500, 'actual_reduced_support_model_hash': hashedited, 'support_restraints_only_delta': True, 'all_attachments_and_full_model_bytes_exact_after_undo': True, 'first_historical_preservation_all8': firstpreserve, 'fresh_process_reopen_historical_preservation_all8': reopenpreserve, 'native_current_blocked': {'mechanics': 'MODEL_INCOMPLETE', 'rows': 0, 'diagnostic': diagnostic, 'honest_viewport_blocked_summary': True, 'no_not_started_claim': True, 'scale_not_generated': True, 'deformed_geometry_observation': 'Screenshot plus unchanged source guard: non-solved returns empty nodePositions; renderer draws deformation only in available state. Internal map not directly inspected in native runtime.', 'render_and_save_report_buttons_disabled': True}, 'recovery': {'current_native_solve': True, 'result_rows': 67, 'all67_native_rows_raw_byte_equal_prior500': True, 'same_input_manifest_hash': True, 'viewport_normalized_available': True, 'local_technical_report_controls_reenabled_no_publication': True}, 'published_numerics': checks, 'signed_reaction_boundary': 'Direct support DTO is resultant500N. Authored localz=-globalY transforms published end-i shearZ500N to signed globalY-500N; localy=globalZ transforms end-i bendingY-1600Nm to signed globalZ-1600Nm. No direct support component DTO claim.', 'fresh_build_and_packaged_selftest_exit0': True, 'embedded_native_fixture_hashes_separate': fixture['hashes'], 'genuine_quit_fresh_reopen_observed': True, 'isolated_final_process_closed': True, 'machine_identity_raw_record': '_run_records/STORE_PROCESS_IDENTITY.json', 'source_build_binding': 'FRESH_BUILD_BINDING.json', 'old_full_authoring_history_agent_witness_applicability': 'FINAL_V2 unchanged relevant controls/native source equality proven in SOURCE_EQUALITY_BINDING.json; full blank cycle not repeated in this focused recheck.', 'literal_MECHANICS_NONCONVERGED_native_route': 'unexecuted; maintained owner focused App tests mock IPC result branch; not actual native-authored nonconvergence', 'unexecuted': ['Literal MECHANICS_BLOCKED/MECHANICS_NONCONVERGED native-authored outcomes', 'Connected provider identity/authentication', 'Whole saved-result schema compatibility', 'Professional acceptance, protected rule criteria, publication/lifecycle transition', 'Parent fresh independent integrated review and full clean DEC025'], 'normal_user_store_or_daemon_operations': False, 'exclusive_lease': 'released_when_frozen_return_delivered'}
(packet / 'FINAL_ASSERTIONS.json').write_text(json.dumps(output, indent=2) + '\n')
print('PASS: source12/docs8/full bundle/dist, two Historical 8-field saves, actual typed blocked viewport/diagnostic/report fences, exact Undo and recovery67 rows, 10 numerical checks, genuine process lifecycle and final closure.')
print('Binary SHA256', build['binary_sha256'], 'Store SHA256', identity['isolated_store_sha256_at_final_close'])
