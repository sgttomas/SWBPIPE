"""Reference self-consistency and negative-control tests, not product validation."""
import json
import unittest
from decimal import Decimal as D, localcontext
from pathlib import Path
from generate import encoded, residual

HERE=Path(__file__).parent
F=json.loads((HERE/'fixtures.json').read_text())
class ReferenceTests(unittest.TestCase):
    def test_frozen_bytes(self):
        self.assertEqual((HERE/'fixtures.json').read_text(),encoded())
    def test_annular_integration(self):
        s=F['section']
        with localcontext() as c:
            c.prec=100
            self.assertLess(abs(D(s['J_m4'])-2*D(s['I_m4'])),D('1e-100'))
    def test_residual_controls(self):
        with localcontext() as c:
            c.prec=100
            for name,target in [('R01',D(1)/201),('R02',D(1)/201),('R03',D(1)),('R04',D(1)),('R05',D(1)/9)]:
                row=F['R'][name]
                observed=residual([[D(v) for v in r] for r in row['K_original']],[D(v) for v in row['candidate']],[D(v) for v in row['f_original']],row['free'])
                self.assertLess(abs(observed['eta']-target),D('1e-90'),name)
                self.assertGreater(observed['eta'],D('1e-9'),name)
            self.assertEqual(F['R']['R03']['work_residual'],0)
    def test_stored_assembly_is_not_intended_physics(self):
        a,b=F['NP']['A'][:2]
        self.assertEqual(D(a['k_stored_exact']),D('0.00009999983012676239013671875'))
        self.assertGreater(abs(D(a['root_relative_physical_error'])),D('1e-9'))
        self.assertEqual(D(b['k_stored_exact']),0)
        self.assertGreater(D(b['k_intended']),0)
    def test_banded_controls(self):
        for r in F['NP']['B']:
            n=r['n'];entries=r['K_entries']
            self.assertTrue(all(abs(i-j)<=2 for i,j,v in entries))
            self.assertEqual(sorted(r['orderings']['even_odd']),list(range(n)))
            if n>=64:self.assertGreater(D(r['z_squared_u']),1)
    def test_precision_loss_control(self):
        for row in F['N']['N08']['torque_cases']:
            x=float(row['theta_x_rad']);self.assertNotEqual(x,0);self.assertEqual(round(x,6),0)
            self.assertEqual((x>0),(D(row['T_Nm'])>0))
    def test_internal_mode_is_independent_of_rigid_rank(self):
        c=F['NP']['C'];self.assertEqual(c['K_internal_mechanism_diagonal'][-1],0)
        self.assertEqual(sum(c['rigid_restraint_B'][i][i] for i in range(6)),6)
    def test_negative_energy_and_zero_residual_are_separate(self):
        k=F['N']['N07']['K'];v=F['N']['N07']['negative_vector']
        self.assertEqual(sum(v[i]*k[i][j]*v[j] for i in range(2) for j in range(2)),-2)
        self.assertFalse(F['R']['R07']['unique'])

if __name__=='__main__': unittest.main()
