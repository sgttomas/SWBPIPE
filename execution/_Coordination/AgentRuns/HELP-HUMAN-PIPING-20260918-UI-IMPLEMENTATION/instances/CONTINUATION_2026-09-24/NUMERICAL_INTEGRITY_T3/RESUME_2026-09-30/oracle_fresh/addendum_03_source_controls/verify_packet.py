#!/usr/bin/env python3
"""Verify exactly the three frozen controls; synthetic TSV is not a solve."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import struct
import sys
import source_controls as c


def decode_source(hexbytes):
    """Separate cursor reader of source.rs's declared encoding."""
    b=bytes.fromhex(hexbytes);pos=0
    def take(n):
        nonlocal pos
        out=b[pos:pos+n];assert len(out)==n;pos+=n;return out
    def u32():return int.from_bytes(take(4),'little')
    def bits():return f'{int.from_bytes(take(8),"little"):016x}'
    def dof():return 6*u32()+take(1)[0]
    assert take(6)==b'K4SRC\x01'
    p={'nodes_bits':[[bits() for _ in range(3)] for _ in range(u32())]}
    members=[]
    for _ in range(u32()):
        m={k:u32() for k in ('id','node_i','node_j')}
        m.update({k:bits() for k in ('elastic_modulus_bits','shear_modulus_bits','area_bits',
                                    'second_moment_y_bits','second_moment_z_bits','torsion_constant_bits')})
        m['y_reference_bits']=[bits() for _ in range(3)];members.append(m)
    p['members']=members
    p['springs']=[{'id':u32(),'global_dof':dof(),'stiffness_bits':bits()} for _ in range(u32())]
    assert u32()==0;p['directional_springs']=[]
    p['constraints']=[{'global_dof':dof(),'value_bits':bits()} for _ in range(u32())]
    loads=[]
    for _ in range(u32()):
        g=dof();label=take(u32()).decode();value=bits()
        loads.append({'global_dof':g,'source_id':label,'value_bits':value})
    p['loads']=loads;assert u32()==0 and u32()==0;p['stations']=[];p['supports']=[]
    assert pos==len(b)
    return p


def main(scratch,output):
    scratch.mkdir(parents=True,exist_ok=True)
    with contextlib.redirect_stdout(io.StringIO()):c.freeze(scratch/'regeneration')
    assert (scratch/'regeneration/TRUTH.json').read_bytes()==(c.HERE/'TRUTH.json').read_bytes()
    truth=json.loads((c.HERE/'TRUTH.json').read_text());summary=[];rounding_count=0
    for case in truth['cases']:
        rows=case['rows'];bykey={r['key']:r for r in rows};cid=case['id']
        assert len(rows)==({'EXTRA-FM-01':40,'EXTRA-MF-01':40,'EXTRA-ZR-01':36}[cid])
        assert decode_source(case['input_identity']['source_encoding_hex'])==case['primitive']
        for r in rows:
            v=c.q.decoded(r['truth'])
            b=struct.unpack('>Q',struct.pack('>d',float(v)))[0]
            assert f'{b:016x}'==r['direct_bits'] and r['direct_outcome']=='Value'
            rounding_count+=1
        exact=lambda key:c.q.decoded(bykey[key]['truth'])
        # All external axial/torsional actions balance the separately authored loads.
        for component in (0,3):
            f=sum((c.q.value(l['value_bits']) for l in case['primitive']['loads'] if l['global_dof']%6==component),c.q.F(0))
            support=sum((exact(r['key']) for r in rows if
                         (r['key'].startswith('S:') and r['key'].endswith(':'+str(component))) or
                         (r['key'].startswith('R:') and int(r['key'][2:])%6==component)),c.q.F(0))
            assert f+support==0
        if cid=='EXTRA-FM-01':
            assert exact('D:6')==exact('E:1:J:0')==5*c.q.H/4
            assert all(exact('S:'+str(i)+':0')==-5*c.q.H/4 for i in (1,2,3))
            kind='Moment'
        elif cid=='EXTRA-MF-01':
            assert exact('D:9')==exact('E:1:J:3')==5*c.q.H/4
            assert all(exact('S:'+str(i)+':3')==-5*c.q.H/4 for i in (1,2,3))
            kind='Force'
        else:
            assert exact('D:6')==5*c.q.H/4 and exact('D:9')==0 and not bykey['D:9']['input_derived']
            kind='Rotation'
        exact_scale=c.q.decoded(case['exact_truth_coupled_scales'][kind])
        published_scale=c.q.value(case['direct_coupled_scale_bits_pinned_baseline'][kind])
        gap=(exact_scale-published_scale)/exact_scale
        assert gap==(1 if cid=='EXTRA-ZR-01' else c.q.F(1,5))
        records=['FORMAT\ta1-public-tsv-v1','CASE\t'+cid,'STATUS\tSelected','SELECTED\t128\t256',
                 'SOURCE_ENCODING\t'+case['input_identity']['source_encoding_hex'],
                 'SOURCE_ENCODING_SELECTED\t'+case['input_identity']['source_encoding_hex'],
                 'STIFFNESS_ENCODING\t'+case['input_identity']['stiffness_encoding_hex']]
        records+=['SCALE\t'+kind+'\t'+bits for kind,bits in case['direct_coupled_scale_bits_pinned_baseline'].items()]
        for r in rows:
            records.append('\t'.join(['LAYOUT',r['key'],r['kind'],'0',str(r['input_derived']).lower()]))
            records.append('\t'.join(['ROW',r['key'],r['kind'],r['direct_outcome'],r['direct_bits'] or 'none',
                                      r['direct_class_pinned_baseline'],r['direct_bound_bits_pinned_baseline'] or 'none']))
        tsv=scratch/(cid+'.synthetic.tsv');tsv.write_text('\n'.join(records)+'\n')
        report=scratch/(cid+'.synthetic-comparison.json')
        with contextlib.redirect_stdout(io.StringIO()):assert c.compare(tsv,report)==0
        summary.append({'id':cid,'rows':len(rows),'input_source_encoding_roundtrip':True,
                        'external_force_and_torque_balance':True,'coverage_kind':kind,
                        'exact_truth_coupled_scale':c.q.encoded(exact_scale),
                        'direct_published_coupled_scale':c.q.encoded(published_scale),'relative_scale_loss':str(gap),
                        'synthetic_interface_pass':True})
    result={'scope':'Independent arithmetic/encoding checks and synthetic exact-output interface only; no solver or constructor execution',
            'python_version':sys.version,'truth_regeneration_identical':True,'rounding_crosschecks':rounding_count,'cases':summary}
    output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'cases':len(summary),'rows':rounding_count,'all_checks':'pass'}))


if __name__=='__main__':main(Path(sys.argv[1]),Path(sys.argv[2]))
