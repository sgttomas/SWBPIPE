"""REVIEW_B: the generator without ADMISSION.json reproduces the frozen 6824b6b6b candidates byte for byte,
and with it reproduces the admitted c1e130818 files (run from WORKING_ROOT of a scratch clone)."""
import importlib.util, subprocess, sys, shutil
from pathlib import Path
PKG = Path('validation/qualification/fixtures/load_reference')
spec = importlib.util.spec_from_file_location('gen', PKG / 'generate_reference_values.py')
gen = importlib.util.module_from_spec(spec); spec.loader.exec_module(gen)
def compare(rev):
    outputs = gen.generate()
    same = sum(subprocess.check_output(['git', 'show', f'{rev}:./{PKG / p.name}']) == data for p, data in outputs.items())
    return same, len(outputs)
print('with ADMISSION.json vs c1e130818:', compare('c1e130818'))
adm = PKG / 'ADMISSION.json'; moved = PKG.parent / 'ADMISSION.json.reviewB-aside'
shutil.move(adm, moved)
try:
    print('without ADMISSION.json vs 6824b6b6b:', compare('6824b6b6b'))
finally:
    shutil.move(moved, adm)
# A tampered review hash is refused.
orig = adm.read_bytes()
try:
    adm.write_bytes(orig.replace(b'795b0421', b'795b0422'))
    try:
        gen.generate(); print('tampered review hash: ACCEPTED (defect)')
    except (ValueError, SystemExit, AssertionError, Exception) as exc:
        print('tampered review hash: refused:', type(exc).__name__, str(exc)[:80])
finally:
    adm.write_bytes(orig)
