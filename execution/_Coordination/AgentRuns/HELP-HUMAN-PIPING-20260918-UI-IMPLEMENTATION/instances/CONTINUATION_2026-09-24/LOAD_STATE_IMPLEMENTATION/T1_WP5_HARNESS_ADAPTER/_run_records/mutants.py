#!/usr/bin/env python3
"""Refusal-removal mutants for the WP5 load-reference-1 adapter.

Usage: mutants.py <scratch project root> <python>. Run only on a scratch copy
(a shared sparse clone at the candidate commit, or a git archive, plus the
three WP5 files), never in place. Each mutant replaces one refusal predicate
with an always-true/neutral form; a mutant is killed when the WP5 suite fails.
PinTests are excluded: they check pinned checkout files that no mutant
touches, and a git-archive scratch lacks their history. The original bytes are
restored after each mutant.
"""
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
PYTHON = sys.argv[2]
ADAPTER = 'tools/validation/qualification_load_reference.py'
HELPER = 'tools/validation/qualification_load_reference_helper.py'
CLASSES = ['CompleteRunTests', 'ValueFaultTests', 'BindingTests', 'TransportRefusalTests', 'StandingTests', 'ReaderTests',
           'AdmissionTests', 'ProcessTests', 'OutputLimitTests', 'PureSemanticsTests']
MUTANTS = [
    ('M01 duplicate case ID', ADAPTER, "require(cid not in seen, 'duplicate case ID')", "require(True, 'duplicate case ID')"),
    ('M02 zero-case manifest', ADAPTER, "0 < len(cases) <= gate.MAX_CASES", "0 <= len(cases) <= gate.MAX_CASES"),
    ('M03 no case required in mode', ADAPTER, "require(bool(ledger), 'no manifest case", "require(True, 'no manifest case"),
    ('M04 duplicate assertion ID', ADAPTER, "require(aid not in ids, 'duplicate assertion ID within case')", "require(True, 'x')"),
    ('M05 duplicate selector', ADAPTER, "require(identity not in identities and ('id', selector_id) not in identities, 'duplicate required selector')", "require(True, 'x')"),
    ('M06 required_scalar_rows count', ADAPTER, "require(len(positive) == case['required_scalar_rows']", "require(True"),
    ('M07 selector producer contract', ADAPTER, "require(selectors['producer_contract'] == CONTRACT", "require(True"),
    ('M08 raw producer/contract', ADAPTER, "require(mechanics.get('schema_version') == '0.2.0' and mechanics.get('producer') == PRODUCER", "require(True"),
    ('M09 raw profile', ADAPTER, "and formulation['profile_id'] == PROFILE\n", "\n"),
    ('M10 foreign namespaces', ADAPTER, "require('source_block_recovery' not in mechanics and 'carrier_evidence' not in mechanics", "require(True"),
    ('M11 load_reference_states namespace', ADAPTER, "require(type(evidence) is dict and set(evidence) == {'pressure', 'connector', 'exact_cases', 'load_reference_states'}", "require(True"),
    ('M12 standing shape (missing standing)', ADAPTER, "require(type(quality) is dict and set(quality) == {'value_representation'", "require(True or set(quality) == {'value_representation'"),
    ('M13 insufficient standing', ADAPTER, "require(standing['standing'] == 'checks_passed'", "require(True"),
    ('M14 unknown standing value', ADAPTER, "require(quality['status'] in STANDING_VALUES", "require(True"),
    ('M15 standing coverage', ADAPTER, "require(len(ids) == len(set(ids)) and set(ids) == set(request_cases)", "require(True"),
    ('M16 sensitive classified as passed', ADAPTER, "passed = quality['status'] == 'checks_passed' and all(", "passed = True or all("),
    ('M17 duplicate result ID', ADAPTER, "and len(ids) == len(set(ids)), 'missing/duplicate result ID')", ", 'missing/duplicate result ID')"),
    ('M18 nonnumeric row value', ADAPTER, "            _text(row['basis_ref']['ref_id'], 'raw basis ID')\n", "            _text(row['basis_ref']['ref_id'], 'raw basis ID')\n        continue\n"),
    ('M19 mode evidence', ADAPTER, "require(bool(mode_rows) and all(", "require(True or all("),
    ('M20 model identity (substitution)', ADAPTER, "require(mechanics.get('model_ref') == model['project']['id']", "require(True"),
    ('M21 blocked wrapper', ADAPTER, "require(doc['blocked'] is False and", "require("),
    ('M22 mechanics solved', ADAPTER, "and mechanics['status'].get('mechanics') == 'MECHANICS_SOLVED'", ""),
    ('M23 row resolves once by signature', ADAPTER, "require(len(matches) == 1 and matches[0] is by_id[0]", "require(True"),
    ('M24 evidence resolves once', ADAPTER, "require(len(found) == 1, label + ' does not resolve exactly once')", "require(len(found) >= 1, 'x')"),
    ('M25 null evidence value', ADAPTER, "require(value is not None, 'evidence value unavailable (null is not zero)')", "value = 0.0 if value is None else value"),
    ('M26 evidence definition', ADAPTER, "require(type(item.get(name)) is type(value) and item.get(name) == value", "require(True"),
    ('M27 negative assertion inversion', ADAPTER, "state = 'failed' if within else 'matched'", "state = 'matched'"),
    ('M28 process outcome', ADAPTER, "if process['outcome'] != 'completed' or not process['stdin_delivery_complete']:", "if False:"),
    ('M29 interruption stops later cases', ADAPTER, "ledger['diagnostics'].append('supervisor interrupted; subsequent cases remain not_run')\n                break", "ledger['diagnostics'].append('supervisor interrupted; subsequent cases remain not_run')"),
    ('M30 KeyboardInterrupt stops later cases', ADAPTER, "ledger['diagnostics'].append('interrupted; subsequent cases remain not_run')\n            break", "ledger['diagnostics'].append('interrupted; subsequent cases remain not_run')"),
    ('M31 owned reader verdict', ADAPTER, "require(reader['response'].get('verdict') == 'consistent', 'owned reader refused the envelope')", "require(True, 'x')"),
    ('M32 reference readiness', ADAPTER, "require(reference.get('readiness') == 'ready' and", "require("),
    ('M33 synthetic target for comparison', ADAPTER, "require(purpose == 'harness_development' or kind != 'harness_synthetic'", "require(True"),
    ('M34 reviewed criteria', ADAPTER, "require(type(profile) is dict and profile.get('profile_status') == 'reviewed'", "require(type(profile) is dict"),
    ('M35 product request binding', ADAPTER, "require(request['solve']['preview_model'] == product", "require(True"),
    ('M36 analytical case key', ADAPTER, "and case['analytical_reference']['case_key'] in analytical['cases']", ""),
    ('M37 WORKING_ROOT-relative paths', ADAPTER, "require(not relative.is_absolute() and '..' not in relative.parts", "require(True"),
    ('M38 reference denominator', ADAPTER, "require(len(indexed) == len(values) and set(indexed) == {row['id'] for row in rows}", "require(True"),
    ('M39 row selector semantics', ADAPTER, "require(len(signatures) == 1 and signatures[0]['source_physical_semantic_dimension'] == selector['dimension']", "require(True"),
    ('M40 evidence field vocabulary', ADAPTER, "require(field in fields, 'field outside the closed evidence vocabulary')", "require(True, 'x')"),
    ('M41 evidence unit/dimension', ADAPTER, "require(selector['unit'] == unit and selector['dimension'] == dimension", "require(True"),
    ('M42 selector basis in bound input', ADAPTER, "require(basis_of(selector) in request_cases", "require(True"),
    ('M43 raw load cases = input (substitution)', ADAPTER, "require(sorted(exact) == wanted and sorted(records) == wanted", "require(True"),
    ('M44 executable digest', ADAPTER, "require(file_sha256(executable) == runner['executable_sha256']", "require(True"),
    ('M45 candidate commit', ADAPTER, "require(actual_commit == runner['candidate_commit']", "require(True"),
    ('M46 reviewed reader binding', ADAPTER, "require(binding.get('status') == 'reviewed_candidate' and bool(binding.get('review_basis'))", "require(True"),
    ('M47 reader dependency pin', ADAPTER, "require(row['sha256'] == DEPENDENCIES[row['path']]", "require(True"),
    ('M48 bound files unchanged during run', ADAPTER, "require(file_sha256(Path(binding['path']), max_bytes=LIMIT) == binding['sha256']", "require(True"),
    ('M49 criterion rule binding', ADAPTER, "require(len(found) == 1, 'criterion rule missing')", "require(True, 'x')"),
    ('M50 run transport', ADAPTER, "require(run['transport'] == TRANSPORT", "require(True"),
    ('M51 locked manifest hash', ADAPTER, "manifest_bytes, manifest_path = gate.read_bound(run['case_manifest'], run_path.parent)",
     "manifest_path = run_path.parent / run['case_manifest']['path']; manifest_bytes = manifest_path.read_bytes()"),
    ('M52 record mode/recovery method', ADAPTER, "require(type(solve) is dict and solve.get('requested_mode') == mode and solve.get('recovery_method') == RECOVERY_METHODS[mode]", "require(True"),
    ('M53 record not joined', ADAPTER, "require(record.get('source_recovery') == NOT_JOINED", "require(True"),
    ('M54 record reference configuration', ADAPTER, "require(record.get('reference_configuration_id') == state.get('reference_configuration_ref')", "require(True"),
    ('M55 foreign raw load cases', ADAPTER, "require(not foreign, 'raw rows carry load cases outside the bound input')", "require(True, 'x')"),
    ('M56 selector-inventory refusal blocks case', ADAPTER, "        if refusal:\n            fail_case(case, 'blocked', refusal)", "        pass"),
    ('M57 negative (selector, wrong value) pair uniqueness', ADAPTER, "require(len(set(pairs)) == len(pairs), 'duplicate negative (selector, wrong value) pair')", "require(True, 'x')"),
    ('M58 positive selectors unique', ADAPTER, "            if unique_selectors:\n", "            if False:\n"),
    ('M59 stdout admitted only to the gate limit', ADAPTER, "process['stdout_bytes'], output_limit_bytes)", "process['stdout_bytes'], min(output_limit_bytes, LIMIT))"),
    ('M60 unwrap ignores the selected limit', ADAPTER, "unwrap(stdout_bytes, prepared['request'], mode, limit=output_limit_bytes)", "unwrap(stdout_bytes, prepared['request'], mode)"),
    ('M61 parser byte bound removed', ADAPTER, "require(len(data) <= limit, 'JSON byte limit exceeded')", "require(True, 'x')"),
    ('M62 parser limit validation removed', ADAPTER, "require(type(limit) is int and 0 < limit <= MAX_OUTPUT_LIMIT, 'invalid JSON byte limit')", "require(True, 'x')"),
    ('M63 large parse: underflow rule dropped', ADAPTER, "gate.require(value != 0.0 or not any(c in '123456789' for c in significand), 'nonzero numeric token underflow')", "pass"),
    ('M64 large parse: duplicate members allowed', ADAPTER, "object_pairs_hook=gate._pairs, ", ""),
    ('M65 large parse: NaN/Infinity allowed', ADAPTER, "parse_constant=forbidden, ", ""),
    ('M66 large parse: nonfinite token allowed', ADAPTER, "gate.require(math.isfinite(value), 'nonfinite numeric token')", "pass"),
    ('M67 reader snapshot limit not passed to helper', ADAPTER, "                 '--source-limit-bytes', str(input_limit)]", "                 ]"),
    ('M68 reader snapshot bound check removed', ADAPTER, "and len(request_bytes) <= input_limit, 'reader input limit')", ", 'reader input limit')"),
    ('M69 reader snapshot retained only to the gate limit', ADAPTER, "helper_process['stdin_bytes_delivered'], output_limit_bytes)", "helper_process['stdin_bytes_delivered'], LIMIT)"),
    ('M70 reader snapshot custody at the gate limit', ADAPTER, "helper_bytes,\n                                                                                       output_limit_bytes))", "helper_bytes))"),
    ('H05 helper parses source at the file limit', HELPER, "source = parsed(source_bytes, source_limit)", "source = parsed(source_bytes)"),
    ('H06 helper source limit validation removed', HELPER, "require(type(source_limit) is int and 0 < source_limit <= MAX_SOURCE_LIMIT, 'invalid source byte limit')", "require(True, 'x')"),
    ('H01 helper isolation', HELPER, "require(sys.flags.isolated == 1 and sys.flags.no_site == 1", "require(True"),
    ('H02 helper dependency digest', HELPER, "require(digest(data) == row['sha256']", "require(True"),
    ('H03 helper binding digest', HELPER, "require(digest(binding_bytes) == binding_sha256", "require(True"),
    ('H04 helper producer identity', HELPER, "require(source.get('producer') == PRODUCER", "require(True"),
]


def main():
    results = []
    for label, relative, old, new in MUTANTS:
        path = ROOT / relative
        original = path.read_bytes()
        text = original.decode()
        count = text.count(old)
        if count != 1:
            results.append((label, 'INVALID (pattern count %d)' % count))
            print(label, results[-1][1], flush=True)
            continue
        path.write_text(text.replace(old, new))
        try:
            process = subprocess.run([PYTHON, '-m', 'unittest', *('tests.test_qualification_load_reference.' + c for c in CLASSES)],
                                     cwd=ROOT, capture_output=True, text=True, env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1'),
                                     timeout=900)
            tail = [line for line in process.stderr.splitlines() if line.startswith(('FAILED', 'OK', 'Ran'))]
            verdict = 'killed' if process.returncode != 0 else 'SURVIVED'
            results.append((label, verdict + ' ' + ' '.join(tail)))
        finally:
            path.write_bytes(original)
        print(label, results[-1][1], flush=True)
    killed = sum(r[1].startswith('killed') for r in results)
    print(f'TOTAL {len(results)} mutants, killed {killed}, other {len(results) - killed}')


if __name__ == '__main__':
    main()
