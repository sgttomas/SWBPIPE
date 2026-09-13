from pathlib import Path
import hashlib, json, math

run = Path(__file__).parent.parent
raw = run / '_run_records'
def read(name):
    return json.loads((raw / (name + '.json')).read_text())
def model(row):
    return json.loads(row['model_json'])
def model_hash(row):
    return json.loads(row['model_hash_json'])['value']

hash350 = 'sha256:2932edda205a4531a1f0b3dd209b7d487b7f0c694fb1c26440d436fa2aab1788'
hash500 = 'sha256:6c909e3114442b2fbbc78c44c98b1bf41520f4c0fa201c3813e046f59de8d0bf'
names350 = ['gui-authored350', 'gui-human-undo350', 'gui-agent-input350', 'gui-solved350']
names500 = ['gui-human-edited500', 'gui-human-redo500', 'gui-agent-edited500', 'gui-solved500', 'gui-historical-before-save', 'gui-historical-after-save', 'gui-resolved500']
for names, expected in [(names350, hash350), (names500, hash500)]:
    original = read(names[0])['model_json']
    for name in names:
        row = read(name)
        assert row['model_json'] == original, name
        assert model_hash(row) == expected, name

attachments = []
for name in names350 + names500:
    m = model(read(name))
    assert len(m['load_cases']) == 1 and m['load_cases'][0]['id'] == 'load:WF'
    assert len(m['load_cases'][0]['primitive_loads']) == 1
    pipe, support, load = m['pipe_segments'][0], m['supports'][0], m['load_cases'][0]['primitive_loads'][0]
    assert (pipe['from'], pipe['to'], pipe['material'], support['node'], load['target']['node']) == ('node:WF-ROOT', 'node:WF-TIP', 'material:WF', 'node:WF-ROOT', 'node:WF-TIP')
    assert load['direction'] == 'global_y' and load['magnitude']['unit'] == 'N'
    assert pipe['y_reference'] == {'x': 0, 'y': 0, 'z': 1}
    assert support['restraints'] == ['UX', 'UY', 'UZ', 'RX', 'RY', 'RZ']
    assert pipe['section'] == {'outside_diameter': {'unit': 'm', 'value': .168}, 'wall_thickness': {'unit': 'm', 'value': .007}}
    assert m['nodes'][0]['position'] == {'x': 0, 'y': 2.4, 'z': 0}
    assert m['nodes'][1]['position'] == {'x': 3.2, 'y': 2.4, 'z': 0}
    assert m['materials'][0]['elastic_modulus'] == {'unit': 'Pa', 'value': 200000000000}
    assert m['materials'][0]['shear_modulus'] == {'unit': 'Pa', 'value': 77000000000}
    for group in ['nodes', 'pipe_segments', 'supports', 'materials', 'load_cases']:
        assert all(x['provenance'] == 'invented_synthetic_workflow_input' for x in m[group])
    assert load['provenance'] == 'invented_synthetic_workflow_input'
    attachments.append({'snapshot': '_run_records/' + name + '.json', 'all_exact': True, 'no_temporary_case': True, 'force_N': load['magnitude']['value']})

fields = ['model_json', 'mechanics_result_json', 'analysis_run_json', 'model_hash_json', 'project_envelope_hash_json', 'editor_intents_json', 'proposal_json', 'selected_review_target_json']
saved, before, after, resolved = [read(n) for n in ['gui-solved500', 'gui-historical-before-save', 'gui-historical-after-save', 'gui-resolved500']]
preservation = {k: saved[k] == before[k] == after[k] for k in fields}
assert all(preservation.values())
assert saved['mechanics_result_json'] == resolved['mechanics_result_json']
assert all('dimension' not in row for row in json.loads(after['mechanics_result_json'])['results'])

I = math.pi / 64 * (.168**4 - (.168 - 2 * .007)**4)
assert I == 1.1493591335983794e-05
numerics = []
for name, force in [('gui-solved350', 350), ('gui-solved500', 500), ('gui-resolved500', 500)]:
    result = json.loads(read(name)['mechanics_result_json'])
    assert result['status']['mechanics'] == 'MECHANICS_SOLVED'
    assert len(result['results']) == 67
    lookup = {row['id']: row for row in result['results']}
    expectations = [
        ('result:disp:node-WF-TIP:uy', force * 3.2**3 / (3 * 200e9 * I) * 1000, 'mm'),
        ('result:disp:node-WF-TIP:rz', force * 3.2**2 / (2 * 200e9 * I), 'rad'),
        ('result:reaction:support-WF-ROOT', force, 'N'),
        ('result:force:pipe-WF:shear-z', force, 'N'),
        ('result:moment:pipe-WF:bending-y', -force * 3.2, 'N*m'),
    ]
    checks = []
    for row_id, expected, unit in expectations:
        row = lookup[row_id]
        assert row['unit'] == unit
        error = abs(row['value'] - expected)
        assert error <= .5e-6 + 1e-10, (name, row_id, error)
        checks.append({'row_id': row_id, 'actual': row['value'], 'expected': expected, 'unit': unit, 'absolute_error': error, 'allowance': .5e-6 + 1e-10, 'passed': True})
    assert lookup['result:force:pipe-WF:shear-z']['metadata']['location'] == 'end_i'
    assert lookup['result:moment:pipe-WF:bending-y']['metadata']['location'] == 'end_i'
    numerics.append({'snapshot': name, 'force_N': force, 'checks': checks, 'transformed_global_root_reaction_y_N': -lookup['result:force:pipe-WF:shear-z']['value'], 'transformed_global_root_moment_z_Nm': lookup['result:moment:pipe-WF:bending-y']['value'], 'direct_support_resultant_N': lookup['result:reaction:support-WF-ROOT']['value']})

hax = (raw / 'gui-reopened-historical-ax.txt').read_text()
rax = (raw / 'gui-resolved500-ax.txt').read_text()
report = (raw / 'gui-historical-report-ax.txt').read_text()
first = (raw / 'first-case-explicit-empty-ax.txt').read_text()
assert 'Historical saved run' in hax and 'HISTORICAL_INPUT_MANIFEST_MISSING' in hax and 'HASH_MISMATCH' not in hax
assert 'Deformation · unavailable' in hax and 'result_rows=0' in hax and 'analysis_run=not generated' in hax
assert 'Rendered calculation report' not in report and 'Render report' not in report
assert 'seam=tauri backend job' in rax and 'job=backend-solve-job-1' in rax and hash500 in rax and 'Historical saved run' not in rax
assert 'Select load case' in first and 'Select node' in first
fixture = read('fresh-packaged-selftest')
assert fixture['hashes']['edit'] == fixture['hashes']['equivalent_agent'] == fixture['hashes']['redo_checkpoint']
assert fixture['hashes']['baseline'] == fixture['hashes']['undo_checkpoint']

output = {
    'source_sha': '7e45f6ec8714250319afb94c1633dee61d48e3c4',
    'accepted_upstream_binding': 'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-WORKFLOW-CONTACT/INTEGRATED_CANDIDATE_BINDING_V2.json',
    'derivative_evidence_only': True,
    'gate_verdict': 'native_workflow_witness_passed_pending_independent_review_and_DEC025',
    'actual_gui_project_id': read('gui-authored350')['project_id'],
    'gui_hash350': hash350, 'gui_hash500': hash500,
    'gui_history_full_model_byte_exact': True, 'attachments': attachments,
    'unchanged_historical_save_all_8_raw_fields': preservation,
    're_solve_all_67_native_rows_raw_byte_equal': True,
    'numerics': numerics,
    'reaction_evidence_boundary': 'Direct support DTO publishes reaction_resultant only. Signed global root Y force is analytical transform -local end-i shear-z because local z=-globalY; global root Z moment is local end-i bending-y because local y=globalZ. No direct support component DTO rows are claimed.',
    'historical': {'only_expected_missing_manifest': True, 'no_false_result_hash_mismatch': True, 'raw_native_rows_still_without_dimension': True, 'current_overlay_unavailable': True, 'current_result_rows_zero': True, 'current_analysis_absent': True, 'current_report_not_renderable': True, 'saved_rows_preserved': 67},
    'native_fixes': {'NV-001': 'cleared: final blank sole-case first authoring observed explicit empty selection and no temporary-case workaround', 'NV-002': 'cleared: baseline V1 and new final native500 correct Historical; unchanged saves preserve raw fields'},
    'agent_equivalence': {'actual_gui_review_intake_and_native_apply_observed': True, 'actual_input_snapshot': '_run_records/gui-agent-input350.json', 'draft': '_run_records/agent-load-batch-source-bound.json', 'expected_frozen_oracle': '_run_records/agent-load-independent-expectation.json', 'actual_output_snapshot': '_run_records/gui-agent-edited500.json', 'input_full_model_equal_gui350': True, 'output_full_model_equal_human500': True, 'source_identity_file_supplied_not_authenticated': True, 'connected_provider_executed': False, 'initial_incomplete_source_draft_correctly_blocked': True},
    'packaged_selftest': {'raw_record': '_run_records/fresh-packaged-selftest.json', 'exit_code': 0, 'fixture_hashes_separate_from_actual_gui': fixture['hashes'], 'not_substitute_for_gui': True},
    'source_build_identity': 'FRESH_BUILD_BINDING.json',
    'post_gui_binding': 'POST_GUI_SOURCE_BUILD_BINDING.json',
    'machine_store_and_process_identity': '_run_records/GUI_STORE_IDENTITY.json',
    'screenshots': ['_run_records/gui-solved500.png', '_run_records/gui-reopened-historical.png', '_run_records/gui-resolved500-tip.png'],
    'unexecuted': ['Connected external agent provider identity/authentication', 'Whole saved-result schema compatibility beyond exact malformed-analysis safety', 'Professional acceptance, protected rule criteria, publication or lifecycle transition', 'Final independent 100percent integrated review and complete clean DEC025 (parent stages)'],
    'normal_app_store_or_daemon_touched': False,
    'isolated_processes_closed': True,
}
(run / 'FINAL_ASSERTIONS.json').write_text(json.dumps(output, indent=2) + '\n')
print('PASS: exact GUI hashes/history/attachments, all 8 saved fields, 67 re-solved rows, 15 numeric checks, Historical/report fences, actual GUI agent native equivalence.')
