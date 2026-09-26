"""Exact metadata-only derivation controls; no solver invocation or target generation."""
from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path
import sys
import tempfile
import unittest

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation.build_first_static_selection import build, digest
from tools.validation.qualification_physics import TRANSPORT
from qualification_fixture_support import fixture_package


class SelectionDerivationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.package = fixture_package(self, self.root/'fixture-package')
        self.binding = self.root/'reader-binding.json'
        self.binding.write_text(json.dumps({'status': 'reviewed_candidate', 'review_basis': ['synthetic metadata-only test; not validator admission']}))

    def test_two_modes_preserve_inputs_values_selectors_and_criteria(self):
        basis = json.loads((self.package/'INPUT_REFERENCE_BINDINGS.json').read_text())
        before = {path: (self.package/path).read_bytes() for path in ['INPUT_REFERENCE_BINDINGS.json', 'axial.reference.candidate.json', 'bending_torsion.reference.candidate.json']}
        for mode in ('sparse_interactive', 'dense_scrutiny'):
            out = self.root/mode
            selected = json.loads(build(self.package, out, '0'*40, '1'*64, self.binding, mode).read_text())
            self.assertEqual(selected['transport'], TRANSPORT)
            self.assertEqual(selected['runner']['solver_mode'], mode)
            self.assertEqual(len(selected['cases']), 2)
            self.assertEqual(sum(len(case['assertions']) for case in selected['cases']), 146)
            for actual, original in zip(selected['cases'], basis['cases']):
                ref = json.loads((out/actual['reference']['path']).read_text())
                old_ref = json.loads((self.package/original['reference']['path']).read_text())
                self.assertEqual(ref['values'], old_ref['values'])
                self.assertEqual(ref['readiness'], 'ready')
                self.assertEqual(actual['input']['sha256'], original['input']['sha256'])
                selectors = json.loads((self.package/original['selectors']['path']).read_text())['assertions']
                self.assertEqual(actual['assertions'], [{key: row[key] for key in ('id','selector','criterion_rule_id')} for row in selectors])
                criterion = json.loads((out/actual['criterion']['path']).read_text())
                old_criterion = json.loads((self.package/original['criterion']['path']).read_text())
                criterion['tolerance_profile']['profile_status'] = old_criterion['tolerance_profile']['profile_status']
                for changed, old in zip(criterion['tolerance_profile']['rules'], old_criterion['tolerance_profile']['rules']):
                    changed['review'] = old['review']
                self.assertEqual(criterion, old_criterion)
            derived = json.loads((out/'DERIVATION.json').read_text())
            self.assertEqual((derived['scalar_obligations'],derived['structural_obligations'],derived['section_subchecks']), (146,20,18))
            self.assertEqual(derived['actual_runs_performed'],0)
        for path, content in before.items():
            self.assertEqual((self.package/path).read_bytes(),content)

    def test_unreviewed_binding_unknown_mode_and_bad_identity_refuse(self):
        self.binding.write_text(json.dumps({'status':'prepared_not_admitted'}))
        with self.assertRaises(ValueError): build(self.package,self.root/'not-reviewed','0'*40,'1'*64,self.binding,'sparse_interactive')
        with self.assertRaises(ValueError): build(self.package,self.root/'bad-mode','0'*40,'1'*64,self.binding,'dense')
        with self.assertRaises(ValueError): build(self.package,self.root/'bad-hash','pending','1'*64,self.binding,'sparse_interactive')

    def test_existing_output_never_overwritten(self):
        path = self.root/'existing';path.mkdir();(path/'sentinel').write_text('unchanged')
        with self.assertRaises(FileExistsError): build(self.package,path,'0'*40,'1'*64,self.binding,'sparse_interactive')
        self.assertEqual((path/'sentinel').read_text(),'unchanged')


if __name__ == '__main__': unittest.main()
