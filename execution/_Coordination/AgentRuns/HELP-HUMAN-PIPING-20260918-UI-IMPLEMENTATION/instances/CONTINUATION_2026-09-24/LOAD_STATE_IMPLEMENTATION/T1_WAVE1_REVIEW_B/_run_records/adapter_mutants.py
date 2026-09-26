"""REVIEW_B independent adapter mutants (run on a scratch clone at the candidate commit)."""
import subprocess, sys, os
from pathlib import Path
ROOT = Path(sys.argv[1])  # WORKING_ROOT of the scratch clone
F = ROOT / 'tools/validation/qualification_load_reference.py'
MUTANTS = {
 'X01 coverage flag never cleared': ("                covered = False\n", "                pass\n"),
 'X02 standing evidence codes ignored': ("and row['evidence_codes'] and all(code == 'NUMERICAL_INTEGRITY_CHECKS_PASSED' for code in row['evidence_codes'])", "and True"),
 'X03 support component unit unchecked': ("        require(item.get('unit') == selector['unit'], 'support component unit differs')", "        pass"),
 'X04 applied load dimension unchecked': ("        require(item.get('dimension') == selector['dimension'] and LOAD_UNITS.get(item.get('dimension')) == selector['unit'],", "        require(True or item.get('dimension') == selector['dimension'] and LOAD_UNITS.get(item.get('dimension')) == selector['unit'],"),
 'X05 criterion family unchecked': ("    require(family is not None and rule.get('result_family') == family, 'criterion result family mismatch')", "    pass"),
 'X06 wrong-value pair by text (signed zero distinct)': ("item['expected'])\n             for item in selected", "repr(item['expected']))\n             for item in selected"),
 'X07 connector evidence unchecked': ("        require(mechanics['contract_evidence']['connector'] == [], 'connector evidence unsupported')", "        pass"),
 'X08 wrapper warning count unchecked': ("summary['warning_count'] == sum(d['severity'].lower() == 'warning' for d in doc['findings'])", "True"),
 'X09 row identity of signature match dropped': ("len(matches) == 1 and matches[0] is by_id[0]", "len(matches) >= 1"),
 'X10 evidence definition type ignored': ("type(item.get(name)) is type(value) and item.get(name) == value", "item.get(name) == value"),
 'X11 negative evaluated like positive': ("                state = 'failed' if within else 'matched'", "                state = 'matched'"),
 'X12 record contract/profile unchecked': ("require(record.get('contract') == 'openpipestress.load_reference_state/1.0.0' and record.get('profile') == PROFILE,", "require(True,"),
}
orig = F.read_text()
env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1')
for name, (old, new) in MUTANTS.items():
    assert orig.count(old) == 1, (name, orig.count(old))
    F.write_text(orig.replace(old, new))
    try:
        p = subprocess.run([sys.executable, '-m', 'unittest', 'tests.test_qualification_load_reference'], cwd=ROOT, env=env,
                           capture_output=True, text=True, timeout=900)
        tail = p.stderr.strip().splitlines()[-1]
        print(f"{name}: {'KILLED' if p.returncode else 'SURVIVED'} ({tail})", flush=True)
    finally:
        F.write_text(orig)
