# RV105: classify I68's probe inputs from the committed probe log (read-only).
# For each input and mode: the published integrity code, and whether the ordinary route
# emitted SOURCE_BLOCK_RECOVERY_UNAVAILABLE. In PP lib.rs `solve_load_case_observed`,
# legacy source recovery runs only when needs_source_recovery = report_sensitive ||
# attempt_err || load_row_finding; a load-row finding demotes a Passed report to Sensitive.
# So a CHECKS_PASSED case that carries SOURCE_BLOCK_RECOVERY_UNAVAILABLE had attempt_err,
# i.e. its initial ordinary attempt failed and W2 then published (the case solved).
import re, sys, json
log = open(sys.argv[1]).read().splitlines()
rows = []
for line in log:
    if not line.startswith('I68_ORDINARY'):
        continue
    parts = line.split()
    name, mode = parts[1], parts[2]
    codes = sorted(set(re.findall(r'NUMERICAL_INTEGRITY_[A-Z_]+', line)))
    legacy = 'SOURCE_BLOCK_RECOVERY_UNAVAILABLE' in line
    mech = re.search(r'"mechanics":"([A-Z_]+)"', line).group(1)
    passed = codes == ['NUMERICAL_INTEGRITY_CHECKS_PASSED']
    rows.append({'input': name, 'mode': mode, 'mechanics': mech, 'integrity': codes,
                 'legacy_source_unavailable': legacy,
                 'inferred': ('W2-published, CHECKS_PASSED' if passed and legacy and mech == 'MECHANICS_SOLVED'
                              else 'not_triggered, CHECKS_PASSED' if passed else '-')})
for r in rows:
    print(json.dumps(r))
dump = json.load(open(sys.argv[2]))
o = dump['source']['retained_precision']['body']['ordinary_attempts'][0]
q = dump['source']['numerical_quality']['cases'][0]['solve_quality']
print('two_body_a dense dump: initial', json.dumps(o['initial']), 'w2', o['w2']['kind'], o['w2'].get('force_scale_exponent'), 'quality', q)
