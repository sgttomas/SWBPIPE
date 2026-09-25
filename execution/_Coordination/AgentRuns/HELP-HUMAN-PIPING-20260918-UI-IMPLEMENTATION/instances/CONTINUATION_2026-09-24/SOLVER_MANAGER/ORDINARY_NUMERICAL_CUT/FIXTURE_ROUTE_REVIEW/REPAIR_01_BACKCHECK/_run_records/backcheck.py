from pathlib import Path
import hashlib, json, difflib, datetime, subprocess, sys

R = Path('/private/tmp/piping-numerical-corrections-20260924')
P = R / 'projects/chirality-piping'
B = P / 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
F = B / 'FIXTURE_CONSUMER/REPAIR_01'
E = B / 'FIXTURE_ROUTE_REVIEW/REPAIR_01_BACKCHECK/_run_records'
h = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
j = lambda p: json.loads(p.read_text())
old = j(B / 'FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json')
freeze = j(F / 'SOURCE_FREEZE.json')
item = freeze['files'][0]
before, after = F / '_run_records/App.test.before.tsx', F / '_run_records/App.test.after.tsx'
checks = {
    'only App.test.tsx in repair freeze': len(freeze['files']) == 1 and item['path'] == 'projects/chirality-piping/apps/desktop/src/App.test.tsx',
    'preimage equals reviewed prior source': h(before) == item['before_sha256'] == '97e6b7af0f7a88dfe77928d953ba21b0643d1ee9633c6ab38c805ee4e7a00131',
    'repaired live source equals frozen after': h(after) == h(R / item['path']) == item['sha256'] == '475aa1b3e82adcfb3ed5a9ff017bc6dc17a44bc21e11a3913f6babf700d59a2d',
    'full delta reconstructs exactly': ''.join(difflib.unified_diff(before.read_text().splitlines(True),after.read_text().splitlines(True),fromfile='before/'+item['path'],tofile='after/'+item['path'])) == (F / 'SOURCE.diff').read_text(),
}
mismatches = [x['path'] for x in old['source_state'] if x['path'] != item['path'] and h(R / x['path']) != x['sha256']]
checks['all other prior reviewed source state unchanged'] = not mismatches
generation = j(P / 'fixtures/product_preview/precision_fixture_generation.json')
checks['all generation input hashes unchanged'] = all(h(R / p) == digest for p,digest in generation['source_input_files'].items())
checks['actual stdout fixture bytes unchanged'] = all(h(P / x['path']) == x['sha256'] for x in generation['outputs'])
checks['all prior instruction origins unchanged'] = all(h(R / x['path']) == x['sha256'] for x in old['instructions_fully_read'])
command = [sys.executable,str(R / 'tools/software_workflow/validate_change_scope.py'),str(R),'--allowed',item['path'],'--path',item['path']]
scope = subprocess.run(command,capture_output=True,text=True)
(E / 'scope.stdout.json').write_text(scope.stdout)
(E / 'scope.stderr.log').write_text(scope.stderr)
checks['explicit-path scope validation without Git'] = scope.returncode == 0
record = {'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'agent':'/root/solver_manager/fixture_route_review','parent':'/root/solver_manager','mechanism':'delegated-harness-native same-reviewer followup; TASK Type 2; no descendants','scope':'F1 App-only repair backcheck','instruction_basis':'Inherited from prior read; actual paths and hashes recorded in FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json; unchanged checked here','checks':checks,'all_passed':all(checks.values()),'mismatches':mismatches,'source_sha256':h(after),'scope_command':command,'limits':'Python data/source checks only; no Git, builds, tests, native/browser, production or test edits'}
(E / 'STATIC_BACKCHECK.json').write_text(json.dumps(record,indent=2)+'\n')
additional = ['apps/desktop/src/features/workspace/statusLabels.ts','apps/desktop/src/features/workspace/shellLayout.ts','apps/desktop/src/features/workspace/solveProof.ts','apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx','apps/desktop/src/features/viewport/PipeViewport.tsx']
origins = {'agent':record['agent'],'parent':record['parent'],'mechanism':record['mechanism'],'role':'TASK Type 2','prior_instruction_basis':'FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json','instruction_origins':old['instructions_fully_read'],'repair_input_origins':[{'path':str(p.relative_to(R)),'sha256':h(p)} for p in sorted(F.rglob('*')) if p.is_file()],'additional_relevant_source_slices':[{'path':str((P/p).relative_to(R)),'sha256':h(P/p)} for p in additional],'production_unchanged_check':'All source_state from prior independent review except the owned App test matched; all 54 generator inputs and both outputs matched.','limits':record['limits']}
(E / 'ORIGINS.json').write_text(json.dumps(origins,indent=2)+'\n')
print(json.dumps({'all_passed':record['all_passed'],'checks':checks,'after_sha256':record['source_sha256']},indent=2))
