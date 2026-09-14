from pathlib import Path
import collections
import difflib
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent
OWN = Path('/Users/ryan/.codex/worktrees/8728/chirality-results-integrity-20260913')
INTEGRATION = Path('/Users/ryan/.codex/worktrees/8728/chirality-results-engineering-3d-20260913')
PROJECT = Path('projects/chirality-piping')
RUN = PROJECT / 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/RESULTS/_run_records'

def sha(data):
    return hashlib.sha256(data).hexdigest()

def emit(name, data):
    (HERE / name).write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n')

origins = []
def source(rel, root=INTEGRATION, scope='live maintained source'):
    path = root / rel
    data = path.read_bytes()
    origins.append({'path': str(path), 'sha256': sha(data), 'bytes': len(data), 'scope': scope})
    return data

table = json.loads(source(PROJECT / 'fixtures/results/semantic_contract_v0_2.json'))
assert table['source_signature_count'] == len(table['rows']) == 60
assert not any(row['governing_ratio_eligible'] for row in table['rows'])
source(PROJECT / 'core/product_physics/src/lib.rs')
source(PROJECT / 'apps/desktop/src/App.tsx')
source(PROJECT / 'apps/desktop/src/services/previewService.ts')
for rel in ['features/results/resultSemantics.ts', 'features/results/ResultsPanel.tsx',
            'features/stress-neutral/StressNeutralExportPanel.tsx',
            'features/headless-runner/HeadlessRunnerPanel.tsx',
            'features/handoff/HandoffPanel.tsx', 'features/native-package/NativePackagePanel.tsx']:
    source(PROJECT / 'apps/desktop/src' / rel)
source(PROJECT / 'fixtures/product_preview/invented_preview_model.json')
raw = json.loads(source(PROJECT / 'fixtures/product_preview/invented_mechanics_result.json', scope='maintained raw fixture; not a fresh App solve capture'))
carrier_rel = RUN / 'TASK_WRITER_IMPLEMENTATION_V1/captures/034-current-schema-repair/desktop/current-enriched.received.json'
carrier = json.loads(source(carrier_rel, OWN, 'earlier 034 qualified legacy-enriched fixture carrier; not actual browser/App capture'))
source(RUN / 'DISPLAY_TEST_REPAIR_V1/WHOLE_BLOCK_EXPECTATION_FREEZE_V2.json', OWN, 'accepted earlier display oracle and scope disclosures')

def derive(document, label):
    derived = []
    for row in document['results']:
        hits = [s for s in table['rows'] if s['kind'] == row['kind'] and s['unit'] == row['unit']
                and (s['component'] is None or s['component'] == (row.get('metadata') or {}).get('component'))]
        assert len(hits) == 1, (row['id'], len(hits))
        signature = hits[0]
        family = signature['family'] if signature['category'] == 'physical_quantity' else 'other'
        dimension = signature['derivative_target_dimension'] or 'TBD'
        derived.append({'id': row['id'], 'kind': row['kind'], 'unit': row['unit'],
                        'component': (row.get('metadata') or {}).get('component'),
                        'signature_id': signature['signature_id'], 'category': signature['category'],
                        'physical_family': family, 'canonical_disposition': signature['canonical_disposition'],
                        'source_physical_semantic_dimension': signature['source_physical_semantic_dimension'],
                        'derivative_target_dimension': signature['derivative_target_dimension'],
                        'received_dimension': row.get('dimension'),
                        'stress_neutral_dimension': dimension,
                        'stress_neutral_witness_eligible': isinstance(row.get('dimension'), str) and row['dimension'] == dimension,
                        'received_reference_witness_eligible': isinstance(row.get('dimension'), str) and bool(row['dimension'])})
    counts = dict(collections.Counter(r['physical_family'] for r in derived))
    return {'origin_label': label, 'source_rows': len(derived), 'family_counts': counts,
            'canonical_counts': dict(collections.Counter(r['canonical_disposition'] for r in derived)),
            'stress_neutral_eligible': sum(r['stress_neutral_witness_eligible'] for r in derived),
            'reference_eligible': sum(r['received_reference_witness_eligible'] for r in derived), 'rows': derived}

raw_oracle = derive(raw, 'Maintained raw fixture, dimensions absent; not App wire/carrier bytes.')
qualified = derive(carrier, 'Earlier 034 qualified legacy-enriched fixture, not fresh App/browser output.')
families = {'other': 52, 'reaction': 29, 'displacement': 66, 'rotation': 45, 'force': 180, 'moment': 180, 'stress': 278}
assert raw_oracle['family_counts'] == qualified['family_counts'] == families
assert raw_oracle['source_rows'] == qualified['source_rows'] == 830
assert qualified['stress_neutral_eligible'] == 828 and qualified['reference_eligible'] == 830
withheld = [r for r in qualified['rows'] if not r['stress_neutral_witness_eligible']]
assert {r['id'] for r in withheld} == {'result:nonlinear-support:free-dof-work-residual', 'result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual'}
assert all(r['category'] == 'diagnostic_work' and r['received_dimension'] == 'moment' for r in withheld)
emit('SOURCE_DERIVED_ROW_ORACLE_V1.json', {'contract_signature_count': 60, 'raw': raw_oracle, 'qualified_earlier_fixture': qualified})

def assertion_end(text, start):
    # Lexical extraction only, never loads or executes TypeScript/tests.
    quote = None
    escaped = False
    line_comment = False
    block_comment = False
    for i in range(start, len(text)):
        c = text[i]
        nxt = text[i:i+2]
        if line_comment:
            if c == '\n': line_comment = False
            continue
        if block_comment:
            if text[i-1:i+1] == '*/': block_comment = False
            continue
        if quote:
            if escaped: escaped = False
            elif c == '\\': escaped = True
            elif c == quote: quote = None
            continue
        if c in '\"\'`': quote = c
        elif nxt == '//': line_comment = True
        elif nxt == '/*': block_comment = True
        elif c == ';': return i + 1
    raise AssertionError('unclosed assertion')

inventory = []
files = []
snapshots = HERE / 'source_snapshots'
snapshots.mkdir(exist_ok=True)
for path in sorted((INTEGRATION / PROJECT / 'apps/desktop/e2e').glob('*.ts')):
    rel = path.relative_to(INTEGRATION)
    data = source(rel)
    assert data == (OWN / rel).read_bytes(), rel
    (snapshots / path.name).write_bytes(data)
    text = data.decode()
    tests = [(m.start(), m.group(1)) for m in re.finditer(r'test\("([^"\n]+)"', text)]
    entries = []
    for match in re.finditer(r'\bexpect(?:\.poll)?\(', text):
        start = match.start()
        end = assertion_end(text, start)
        statement = text[start:end]
        line = text.count('\n', 0, start) + 1
        owner = next((name for pos, name in reversed(tests) if pos < start), 'shared helper')
        semantic = bool(re.search(r'unit|dimension|conversion|result|stress|reaction|force|moment|ratio|displacement|redact|private|protect|report|export|solve|mechanic|binding|diagnostic|professional|acceptance', statement, re.I))
        category = 'preserve existing assertion; unrelated UI/authoring/layout/runtime seam'
        if semantic: category = 'preserve existing semantic/authored-unit/privacy assertion; no physical-policy change'
        if path.name == 'r2-smoke.spec.ts' and 620 <= line <= 776:
            category = 'reviewed complete result/report/export block; preserve except four confirmed contradictions'
        contradiction = None
        if path.name == 'r2-smoke.spec.ts':
            for old, new in [('"33"', '"29"'), ('"33 of 830 results match filter"', '"29 of 830 results match filter"'),
                             ('"Showing 1 to 33 of 33 matching results; page 1 of 1"', '"Showing 1 to 29 of 29 matching results; page 1 of 1"')]:
                if old in statement and ('reaction' in statement or '33 of 830' in statement or '1 to 33' in statement):
                    contradiction = {'old': old, 'replacement': new, 'basis': 'frozen exact source physical family classification: 29 reactions, four review deltas belong to other'}
            if 'stress-neutral-unit-witnesses' in statement and '"count=830"' in statement:
                contradiction = {'old': '"count=830"', 'replacement': '"count=828"', 'basis': 'two exact diagnostic_work rows are retained but legacy moment declaration is incompatible with semantic TBD'}
        entries.append({'file': str(rel), 'line': line, 'test_or_helper': owner, 'assertion': statement,
                        'semantic_or_policy_related': semantic, 'disposition': category, 'confirmed_contradiction': contradiction})
    inventory.extend(entries)
    files.append({'path': str(rel), 'sha256': sha(data), 'lines': len(text.splitlines()), 'assertion_count': len(entries),
                  'semantic_or_policy_assertions': sum(e['semantic_or_policy_related'] for e in entries),
                  'contradictions': sum(e['confirmed_contradiction'] is not None for e in entries), 'own_lane_bytes_equal': True})
assert len(files) == 7
assert sum(e['confirmed_contradiction'] is not None for e in inventory) == 4
emit('COMPLETE_E2E_ASSERTION_INVENTORY_V1.json', {'method': 'Every expect()/expect.poll() assertion statement lexically extracted with source line and complete statement; all seven files captured and read. No tests imported or executed.', 'files': files, 'assertions': inventory})

r2 = INTEGRATION / PROJECT / 'apps/desktop/e2e/r2-smoke.spec.ts'
original = r2.read_text()
proposed = original
for item in inventory:
    if item['confirmed_contradiction']:
        old = item['assertion']
        new = old.replace(item['confirmed_contradiction']['old'], item['confirmed_contradiction']['replacement'])
        assert proposed.count(old) == 1
        proposed = proposed.replace(old, new)
(HERE / 'r2-smoke.spec.ts.proposed').write_text(proposed)
patch = ''.join(difflib.unified_diff(original.splitlines(True), proposed.splitlines(True),
                                   fromfile='a/' + str(r2.relative_to(INTEGRATION)), tofile='b/' + str(r2.relative_to(INTEGRATION))))
(HERE / 'RECOMMENDED_ONE_FILE_DELTA_V1.patch').write_text(patch)
emit('RECOMMENDED_ASSERTION_CONTRACT_V1.json', {
    'status': 'READ_ONLY_PREPARATION_NOT_IMPLEMENTED_NOT_EXECUTED',
    'integration_basis_reported_by_root': '0306db0b481def374ebfdddea3e0f92f0ea8a626',
    'fence': ['projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts'],
    'confirmed_contradictions': [e for e in inventory if e['confirmed_contradiction']],
    'bounded_delta': 'Exactly four stale literal corrections. No added, deleted, weakened or otherwise altered assertions; all other source/stress policy assertions preserved.',
    'family_counts': families, 'all_rows': 830, 'pipe_P_120_text_filter': 170,
    'reaction_page': 'Showing 1 to 29 of 29 matching results; page 1 of 1',
    'stress_neutral': {'rows_retained': 830, 'witnesses': 828, 'validation': 'blocked', 'withheld_rows': withheld,
                       'existing_JSON_redaction_policy': 'unchanged', 'existing_CSV_href_policy': 'unchanged'},
    'preserved': {'headless_received_declaration_witnesses': 830, 'handoff_received_declaration_witnesses': 830,
                  'native_package_result_quantity_witnesses': 832, 'native_explanation': '830 exact value/unit rows plus two actual summary quantities, not canonical physical quantities',
                  'source_unit_lists': 'MPa,N,N*m,mm,rad unchanged; diagnostic N*m retained without energy-unit expansion',
                  'report_lint_targets': 44, 'report_lint_conversion_witness_targets': 2,
                  'PCF_conversion_witnesses': 23, 'CAEPIPE_MBF_conversion_witnesses': 15,
                  'external_harness_witnesses': 3, 'review_geometry_witnesses': 75,
                  'native_project_unit_declarations': 6, 'native_model_quantity_witnesses': 50,
                  'adapter_framework_witnesses': 1, 'adapter_SDK_witnesses': 5,
                  'authored_units_and_rule_library_dimensions': 'unchanged; authored quantities and private rule AST are not new product result meaning',
                  'privacy': 'Every current redacted payload assertion, absent protected report DOM, explicit private intent, browser desktop-only seam and no-conversion safeguard retained verbatim'},
    'not_added': 'No canonical encoding, model, solver policy, headless/handoff/API feature or readiness promotion; no dated AgentRuns dependency in proposed maintained test.',
    'static_product_defect_found': None,
    'qualification_limits': 'Earlier qualified fixture is declaration compatibility evidence, not actual fresh browser carrier or byte identity. Default source physical families independently agree in maintained raw fixture. Actual first Chromium failure separately confirms reaction expected33/actual29; remaining full sweep output still awaited; no E2E pass claimed.',
    'next_execution': 'Wait actual root DEC025_V3 return, sealed one-file writer extension and exclusive runtime grant; then existing sole TASK writer may apply and run exact root-approved focused E2E only.'
})
failure_rel = PROJECT / 'apps/desktop/test-results/r2-smoke-R2-desktop-previe-4ab18-report-and-viewport-overlay-chromium-desktop/error-context.md'
failure = source(failure_rel, INTEGRATION, 'actual root DEC025_V3 first Chromium failure; read-only copied capture, full sweep still running')
(HERE / 'ACTUAL_CHROMIUM_FIRST_FAILURE_ERROR_CONTEXT_V1.md').write_bytes(failure)
assert b'Expected substring: "33"' in failure and b'Received string:    "29"' in failure
emit('ACTUAL_FAILURE_DISPOSITION_V1.json', {'origin_path': str(INTEGRATION / failure_rel), 'sha256': sha(failure),
    'surface': 'chromium-desktop', 'test': 'R2 desktop preview smoke covers solve, results, report, and viewport overlay',
    'assertion_line': 631, 'expected_old': '33', 'actual': '29', 'verdict': 'actual browser failure agrees with independent frozen source oracle',
    'not_observed_yet': 'Later stress-neutral 828 expectation is source-derived preparation, not a claim that failed browser reached that assertion.',
    'full_sweep_status': 'still running when parent relayed this first failure; no runtime release'})
emit('ORIGIN_MANIFEST_V1.json', {'origins': origins, 'integration_repo_unmodified': True,
                               'own_product_and_tests_unmodified': True, 'runtime_executed': False,
                               'evidence_only_static_python': True})
print(json.dumps({'files': files, 'assertions': len(inventory), 'contradictions': 4,
                  'recommended_patch_sha256': sha(patch.encode()), 'origins': len(origins)}, indent=2))
