"""Selection/transport/scalar/structure integration with synthetic observations.

The observations are constructed from authored reference data. These tests do
not invoke or stand in for a solver or the owned physical-consistency helper.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys
import subprocess
import tempfile
import unittest
from unittest.mock import patch

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
sys.path.insert(0, str(Path(__file__).resolve().parent))
from tools.validation import qualification_gate as gate, qualification_physics as physics
from tools.validation.build_first_static_selection import build
from tools.validation.qualification_physics_structure import check_first_static_structure
from test_qualification_physics_structure import synthetic_case, BINDINGS

TABLE = json.loads((PROJECT/'validation/qualification/fixtures/ordinary_physics_1_semantics.json').read_text())


def wrapper(observation):
    raw = deepcopy(observation['raw'])
    raw['results'][-1]['entity_ref'] = 'solver:linear_static_preview'
    request = observation['request']
    return {'blocked': False, 'decisions': [], 'findings': [],
            'summary': {'blocking_count': 0, 'decision_count': 0, 'finding_count': 0, 'warning_count': 0},
            'payload': {'artifact': 'openpipestress.headless_runner_cli_output', 'schema_version': '1.0.0',
                        'command': 'solve', 'operation': 'solve', 'request_validation': {'diagnostics': []},
                        'result_validation': {'diagnostics': []}, 'diagnostics': [],
                        'runner_result': {'run_id': 'run:headless-preview:'+request['request']['request_id'],
                                          'job': {'state':'COMPLETED'}, 'analysis_status':['MECHANICS_SOLVED'], 'diagnostics':[]},
                        'mechanics_envelope': raw}}


class PhysicsIntegrationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        binding = self.root/'synthetic-binding.json'
        binding.write_text(json.dumps({'status':'reviewed_candidate','review_basis':['synthetic derivation-only stub']}))
        path = build(BINDINGS,self.root/'selection','0'*40,'1'*64,binding,'sparse_interactive')
        self.selection_path = path
        self.manifest = json.loads(path.read_text())
        self.basis = {'path': BINDINGS/'INPUT_REFERENCE_BINDINGS.json',
                      'value': json.loads((BINDINGS/'INPUT_REFERENCE_BINDINGS.json').read_text())}

    def prepare(self, case):
        result = gate.prepare_case(case,self.selection_path.parent,'development_comparison',TABLE['rows'],transport=physics.TRANSPORT)
        physics.verify_selected_case(case,result,self.basis)
        return result

    def test_four_case_modes_keep_scalar_and_structural_denominators(self):
        for mode in ('sparse_interactive','dense_scrutiny'):
            manifest = deepcopy(self.manifest)
            manifest['runner']['solver_mode'] = mode
            ledger = gate.predeclare(manifest)
            for case, case_ledger in zip(manifest['cases'],ledger['cases']):
                slug = 'axial' if case['id']=='original_static_axial_v1' else 'bending_torsion'
                observation = synthetic_case(slug,mode)
                prepared = self.prepare(case)
                raw, rows = gate.unwrap(json.dumps(wrapper(observation)).encode(),observation['request'],mode,transport=physics.TRANSPORT)
                gate.evaluate_rows(case_ledger,prepared,rows)
                self.assertEqual(case_ledger['state'],'matched')
                by_id = {row['id']:row for row in rows}
                coverage = all(gate.selector_matches(by_id[a['selector']['id']],a['selector']) for a in case['assertions'])
                self.assertTrue(coverage)
                checks = check_first_static_structure(raw,observation['request'],observation['case_binding'],
                    prepared['structural']['section_reference']['value'],prepared['structural']['section_criteria']['value'],coverage,True)
                # True here is a declared stub for this synthetic composition
                # test, not independent physical-consistency evidence.
                self.assertEqual({row['state'] for row in checks},{'matched'})
                case_ledger['structural_checks'] = checks
            gate.summarize(ledger)
            self.assertEqual(ledger['outcome'],'in_progress')
            self.assertEqual((ledger['summary']['required_assertions'],ledger['summary']['required_structural_checks']),(146,20))
            # No final publication/process custody is asserted by these pure tests.

    def test_future_composite_sensitive_duplicate_diagnostics_and_wrong_mode_refuse(self):
        observation = synthetic_case()
        mutations = [lambda raw: raw['producer'].update(semantic_contract_id='openpipestress.result_semantics/0.3.0/physics-source-1'),
                     lambda raw: raw.update(source_block_recovery=None),
                     lambda raw: raw['numerical_quality'].update(status='sensitive'),
                     lambda raw: raw['diagnostics'].append(deepcopy(raw['diagnostics'][0])),
                     lambda raw: raw['results'][-1].update(value=2)]
        for mutate in mutations:
            value = wrapper(observation);mutate(value['payload']['mechanics_envelope'])
            with self.assertRaises(ValueError):
                gate.unwrap(json.dumps(value).encode(),observation['request'],'sparse_interactive',transport=physics.TRANSPORT)

    def test_metadata_absence_is_not_null_or_invented_text(self):
        case = self.manifest['cases'][0]
        prepared = self.prepare(case)
        magnitude = next(x for x in case['assertions'] if x['id']=='node.tip.magnitude')
        row = next(x for x in synthetic_case()['raw']['results'] if x['id']==magnitude['selector']['id'])
        self.assertNotIn('metadata',row)
        self.assertTrue(gate.selector_matches(row,magnitude['selector']))
        row['metadata'] = None
        self.assertFalse(gate.selector_matches(row,magnitude['selector']))
        with self.assertRaises(ValueError):
            gate.prepare_case(case,self.selection_path.parent,'development_comparison',TABLE['rows'])

    def test_selected_targets_criteria_selectors_and_addendum_cannot_drift(self):
        case = self.manifest['cases'][0]
        for mutate in [lambda p: p['profile']['rules'][0].update(absolute_tolerance_value=1),
                       lambda p: p['structural']['section_criteria'].update(sha256='0'*64),
                       lambda p: p.update(input=p['input']+b' ')]:
            prepared = self.prepare(case);mutate(prepared)
            with self.assertRaises(ValueError): physics.verify_selected_case(case,prepared,self.basis)
        changed = deepcopy(case);changed['assertions'][0]['selector']['unit']='m'
        with self.assertRaises(ValueError): self.prepare(changed)

    def test_missing_scalar_or_structural_binding_does_not_reduce_predeclared_ledger(self):
        ledger = gate.predeclare(self.manifest)
        case = deepcopy(self.manifest['cases'][0]);case['assertions'].pop()
        with self.assertRaises(ValueError): self.prepare(case)
        gate.fail_case(ledger['cases'][0],'blocked','incomplete selection')
        gate.summarize(ledger)
        self.assertEqual(ledger['summary']['required_assertions'],146)
        self.assertEqual(ledger['summary']['required_structural_checks'],20)
        self.assertEqual(ledger['summary']['structural_checks']['blocked'],10)

    def test_authoritative_inventory_matches_selected_original_ids(self):
        self.assertEqual(tuple(case['id'] for case in self.manifest['cases']),physics.REQUIRED_CASE_IDS)
        for case in self.manifest['cases']:
            self.assertEqual(tuple(row['id'] for row in case['assertions']),physics.REQUIRED_ASSERTION_IDS)
        for change in [lambda m:m['cases'].pop(),lambda m:m['cases'][0]['assertions'].pop(),
                       lambda m:m.update(cases=None),lambda m:m['cases'].append(deepcopy(m['cases'][0]))]:
            manifest=deepcopy(self.manifest);change(manifest)
            ledger=gate.predeclare(manifest)
            for case in ledger['cases']: gate.fail_case(case,'blocked','synthetic admission refusal')
            gate.summarize(ledger)
            self.assertEqual((ledger['summary']['required_cases'],ledger['summary']['required_assertions'],
                              ledger['summary']['required_structural_checks'],ledger['summary']['required_section_subchecks']),(2,146,20,18))
            self.assertEqual(ledger['summary']['section_subchecks']['blocked'],18)
            self.assertNotEqual(ledger['outcome'],'all_required_assertions_matched')

    def test_legacy_dispatch_refuses_all_newer_identity_or_evidence_presence(self):
        observation=synthetic_case();value=wrapper(observation)
        raw=value['payload']['mechanics_envelope'];raw['schema_version']='0.1.0'
        namespaces=('producer','numerical_quality','formulation_basis','contract_evidence','source_block_recovery','carrier_evidence')
        for key in namespaces: raw.pop(key,None)
        # Genuine header-free shape remains the legacy positive control.
        gate.unwrap(json.dumps(value).encode(),observation['request'],'sparse_interactive')
        for key in namespaces:
            for present_value in (None,{},False):
                modified=deepcopy(value);modified['payload']['mechanics_envelope'][key]=present_value
                with self.assertRaisesRegex(ValueError,'not legacy raw0.1'):
                    gate.unwrap(json.dumps(modified).encode(),observation['request'],'sparse_interactive')


class PhysicsProcessCompositionTests(unittest.TestCase):
    """Real supervised processes with deliberately synthetic solver/reader stubs.

    Only the closed reader dependency constants/origin are patched for this test.
    Transport dispatch, byte custody, isolated helper invocation, selected target
    binding, scalar comparison and all ten structural checks execute normally.
    This cannot qualify a solver or the production physical validator.
    """
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.source = self.root/'synthetic-reader'
        data = {
            physics.MODULE: b'def validate_physics_evidence(source):\n    if source.get("test_force_reader_refusal"): raise ValueError("synthetic reader refusal")\n',
            physics.TABLE: (PROJECT/'validation/qualification/fixtures/ordinary_physics_1_semantics.json').read_bytes(),
            physics.UNITS: b'// explicit synthetic test authority; no production claim\n',
        }
        self.dependencies = {name: hashlib.sha256(content).hexdigest() for name,content in data.items()}
        for name,content in data.items():
            path=self.source/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(content)
        review=self.source/'test-review.txt';review.write_text('Synthetic stub test fixture; not an independent review.\n')
        self.binding=self.root/'synthetic-binding.json'
        self.binding.write_text(json.dumps({'format':'openpipestress.ordinary_physics_consistency_binding/1',
            'status':'reviewed_candidate','contract_id':physics.CONTRACT,'entrypoint':'validate_physics_evidence',
            'files':[{'path':name,'sha256':digest} for name,digest in self.dependencies.items()],
            'review_basis':[{'path':'test-review.txt','sha256':hashlib.sha256(review.read_bytes()).hexdigest()}]}))
        self.executable=self.root/'synthetic-runner'
        self.commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=PROJECT,text=True).strip()
        self.count=0

    def run_composition(self, mode='sparse_interactive', mutate_manifest=None, child_extra='', refuse_reader=False):
        self.count+=1
        packets={}
        for slug in ('axial','bending_torsion'):
            observation=synthetic_case(slug,mode)
            value=wrapper(observation)
            if refuse_reader: value['payload']['mechanics_envelope']['test_force_reader_refusal']=True
            packets[observation['request']['request']['request_id']]=value
        packet_path=self.root/f'packets-{self.count}.json';packet_path.write_text(json.dumps(packets))
        code='import json,sys\nfrom pathlib import Path\nrequest=json.load(sys.stdin)\n'
        code+=child_extra+'\n'
        code+='print(json.dumps(json.loads(Path('+repr(str(packet_path))+').read_text())[request["request"]["request_id"]]))\n'
        code+='print(json.dumps(sys.argv[1:]),file=sys.stderr)\n'
        self.executable.write_text('#!'+sys.executable+'\n'+code);self.executable.chmod(0o700)
        selection=build(BINDINGS,self.root/f'selection-{self.count}',self.commit,
            hashlib.sha256(self.executable.read_bytes()).hexdigest(),self.binding,mode)
        if mutate_manifest:
            manifest=json.loads(selection.read_text());mutate_manifest(manifest);selection.write_text(json.dumps(manifest))
        out=self.root/f'output-{self.count}'
        with patch.object(physics,'DEPENDENCIES',self.dependencies), patch.object(physics,'project_root',return_value=self.source):
            result=gate.run_selection(selection,self.executable,PROJECT,out)
        return result,out

    def test_actual_synthetic_process_and_isolated_reader_both_modes(self):
        for mode in ('sparse_interactive','dense_scrutiny'):
            result,out=self.run_composition(mode)
            self.assertEqual(result['outcome'],'all_required_assertions_matched',result)
            self.assertEqual((result['summary']['required_assertions'],result['summary']['required_structural_checks']),(146,20))
            self.assertEqual(result['summary']['required_section_subchecks'],18)
            self.assertEqual(result['summary']['section_subchecks']['matched'],18)
            for case in result['cases']:
                self.assertEqual(case['process']['command'][1:],['solve','--input','-','--solver-mode',mode,'--explicit-local-private-intent'])
                self.assertTrue(case['physical_consistency']['response']['isolation']['isolated'])
                self.assertEqual(case['custody_at_publication'],'checked')
                self.assertEqual(case['physical_consistency']['process']['command'][1:3],['-I','-S'])
            self.assertEqual(json.loads((out/'ledger.json').read_text()),result)

    def test_reader_refusal_retains_all_obligations(self):
        result,_=self.run_composition(refuse_reader=True)
        self.assertNotEqual(result['outcome'],'all_required_assertions_matched')
        self.assertEqual(result['summary']['assertions']['error'],146)
        self.assertEqual(result['summary']['structural_checks']['error'],20)

    def test_malformed_submitted_inventory_keeps_all_required_obligations(self):
        mutations=[lambda m:m['cases'].pop(),lambda m:m['cases'][0]['assertions'].pop(),
                   lambda m:m['cases'].append(deepcopy(m['cases'][0])),
                   lambda m:m['cases'][0]['assertions'].append(deepcopy(m['cases'][0]['assertions'][0])),
                   lambda m:m['cases'][0]['assertions'][0].update(id='wrong'),
                   lambda m:m['cases'].reverse(),lambda m:m.update(cases=None)]
        for mutation in mutations:
            result,_=self.run_composition(mutate_manifest=mutation)
            self.assertTrue(all(case['state']=='blocked' and case['process'] is None for case in result['cases']))
            self.assertEqual((result['summary']['required_cases'],result['summary']['required_assertions'],
                              result['summary']['required_structural_checks'],result['summary']['required_section_subchecks']),(2,146,20,18))
            self.assertEqual(result['summary']['section_subchecks']['blocked'],18)
            self.assertIn('submitted_inventory',result)

    def test_retained_reader_or_original_target_tampering_cannot_pass(self):
        changes=["Path('../physics-reader-basis/core/analysis_runs/physics_evidence.py').write_text('changed')",
                 "Path('../case-0000.original-reference.json').write_text('{}')"]
        for code in changes:
            result,_=self.run_composition(child_extra=code)
            self.assertNotEqual(result['outcome'],'all_required_assertions_matched')
            self.assertEqual(result['summary']['required_assertions'],146)
            self.assertEqual(result['summary']['required_structural_checks'],20)
            self.assertTrue(any(case['state']=='error' for case in result['cases']))

    def test_incomplete_structural_checker_return_cannot_shrink_denominator(self):
        from tools.validation import qualification_physics_structure as structure
        with patch.object(structure,'check_first_static_structure',return_value=[]):
            result,_=self.run_composition()
        self.assertNotEqual(result['outcome'],'all_required_assertions_matched')
        self.assertEqual(result['summary']['required_structural_checks'],20)
        self.assertEqual(result['summary']['structural_checks']['error'],20)
        self.assertEqual(result['summary']['required_section_subchecks'],18)
        self.assertEqual(result['summary']['section_subchecks']['error'],18)

    def test_malformed_nested_section_return_cannot_remove_subchecks(self):
        from tools.validation import qualification_physics_structure as structure
        original=structure.check_first_static_structure
        def truncated(*args,**kwargs):
            checks=original(*args,**kwargs)
            next(check for check in checks if check['id']=='complete_case_material_section')['details']['section_quantities'].pop()
            return checks
        with patch.object(structure,'check_first_static_structure',side_effect=truncated):
            result,_=self.run_composition()
        self.assertNotEqual(result['outcome'],'all_required_assertions_matched')
        self.assertEqual(result['summary']['required_structural_checks'],20)
        self.assertEqual(result['summary']['required_section_subchecks'],18)
        self.assertEqual(result['summary']['section_subchecks']['error'],18)


if __name__ == '__main__': unittest.main()
