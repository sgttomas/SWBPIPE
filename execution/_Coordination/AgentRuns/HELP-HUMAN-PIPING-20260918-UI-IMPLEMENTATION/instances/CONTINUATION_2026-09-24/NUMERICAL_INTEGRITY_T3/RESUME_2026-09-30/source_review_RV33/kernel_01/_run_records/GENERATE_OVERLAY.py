"""Generate independent comparison literals from sealed reference tables, no formula execution.
Usage: python3 GENERATE_OVERLAY.py <k6c-root>
This is review scratch, not maintained test content or a production dependency.
"""
from pathlib import Path
import json,sys,re,hashlib
k=Path(sys.argv[1]); r=k/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'; out=r/'source_review_RV33/kernel_01/_run_records'
a=Path(json.loads((out/'ARCHIVE.json').read_text())['archive']); dest=a/'projects/chirality-piping/core/solver/performance_harness/tests/rv33_reference.rs'
phase_map=dict(zip('group_geometry group_pattern_tagging group_order_rcm group_free_blocks prep_ledger_extent_encoding_layout scaffolding_encoding_geometry_copy shared_build own_initial_solve own_residual own_correction own_fallback fallback_selected_clone own_recovery verify_shared_build verify_shared_refusal resolution_TOP resolution_HATCHECK first_formation_scale pass_delta_solve pass_recovery pass_formation_scale pass_one_contribution_operand shift_factor_Nl_retry report_build attempt_refusal_transfer R7_tracker_decision canonical_before_R7 canonical_certificate publication_draft certificate_H_RU selected_finish'.split(),'GroupGeometry PatternTagging Ordering FreeBlocks Preparation Scaffolding SharedBuild InitialSolve Residual Correction Fallback ChosenFallbackClone Recovery VerificationSharedBuild VerificationRefusal ResolutionTop ResolutionHatCheck FirstFormationScale PassDeltaSolve PassRecovery PassFormationScale ContributionOperand Shift ReportBuild AttemptRefusalTransfer StopRule CanonicalBeforeRule CanonicalCertificate PublicationDraft Certificate SelectedFinish'.split()))
lines=['use open_pipe_stress_solver_performance_harness::k6::w1::envelope::*;','fn b(r:u128,m:u128)->MetricBytes{MetricBytes{requested:r,moving:m}}']
records=[]; counters={'fixtures':0,'phase_pairs':0,'r7_pairs':0,'scalar_or_pair_assertions':0}; bindings=[]
ref=json.loads((r/'I21/kernel_reference_22/KERNEL_193.json').read_text())['rows'];cli=json.loads((r/'I21/kernel_reference_22/CLI24_EXACT_KERNEL.json').read_text()); h=json.loads((r/'I21/h_numeric_19/PHASES.json').read_text())['rows'];vrmeta={x['id']:x for x in json.loads((r/'I21/source_09/FIXTURE_INPUTS.json').read_text())['vr_rows']}
for family,rows in [('ref193',ref),('cli24',cli),('h33',h)]:
 for ix,row in enumerate(rows):
  id=row.get('id',row.get('model')); d=row.get('descriptor'); own=row['owners']; is_h=family=='h33'
  if is_h:
   hd=row['descriptors']; d={s:hd[t] for s,t in {'N':'w1_nodes','m':'w1_members','r':'w1_constraints','l':'w1_loads','t':'w1_stations','n':'w1_dofs','f':'w1_free_dofs','q':'w1_rows','X':'w1_source_encoding_len','z':'w1_pattern_entries','h':'w1_profile_entries','B':'w1_bodies','b':'w1_blocks'}.items()};d.update(s=0,d=0,u=0,source_id_bytes=row['input_upper']['source_id_bytes']);maxid=3+len(str(d['n']-1)) if d['l'] else 0;max_origin='existing H reference constructor grammar upper; no new actual-source assertion'
  elif family=='ref193':maxid=d['max_source_id_bytes'];max_origin='sealed reference193 descriptor'
  else:
   meta=vrmeta[id]
   if meta.get('source_id_lengths') is not None:maxid=max(meta['source_id_lengths'],default=0);max_origin='source09 retained embedded ID lengths'
   else:
    lengths=[1+len(str(i)) for i in range(d['l'])];assert sum(lengths)==d['source_id_bytes'];maxid=max(lengths,default=0);max_origin='source09 c<index> external constructor grammar, aggregate independently matched to CLI table'
  bindings.append({'family':family,'id':id,'descriptor':d,'max_id_bytes':maxid,'max_id_origin':max_origin})
  vals={s:d[t] for s,t in {'nodes':'N','members':'m','axis_springs':'s','directional_springs':'d','constraints':'r','load_terms':'l','stations':'t','load_id_bytes':'source_id_bytes','support_groups':'u'}.items()}; vals.update(max_load_id_bytes=maxid,nonzero_prescribed_terms=0)
  st={s:d[t] for s,t in {'dofs':'n','free_dofs':'f','quantities':'q','source_encoding_bytes':'X','pattern_entries':'z','profile_entries':'h'}.items()}
  lines.append(f'#[test] fn {family}_{ix:03}() {{ let label={json.dumps(id)};')
  lines.append('let input=KernelInput{'+','.join(f'{s}:{v}' for s,v in vals.items())+',structure:Some(StructuralCounts{'+','.join(f'{s}:{v}' for s,v in st.items())+'})};')
  pol='PopulationPolicy::NodesAndFreeDofsUpper' if family=='ref193' else f'PopulationPolicy::Exact{{bodies:{d["B"]},free_blocks:{d["b"]}}}'
  lines.append(f'let d=KernelDescriptor::new(input,SourceConstruction::{"HModelV1" if is_h else "VrModelV1"},{pol}).expect(label);let e=kernel_envelope(&d,&ReferenceKernelProfile::source40129_rust1971_aarch64_v1()).expect(label);')
  def eq(actual,expected):
   counters['scalar_or_pair_assertions']+=1;lines.append(f'assert_eq!({actual},{expected},"{{label}}: {actual}");')
  for field,key in [('base','BASE' if is_h else 'B0'),('original_source','SRC0'),('prepared_source_clone','source_clone' if is_h else 'PREP_source_clone')]:eq('e.'+field,str(own[key]))
  for field,key in [('shared','S'),('solved','U'),('verification','V')]:eq('e.'+field,'['+','.join(str(v) for v in own[key].values())+']')
  source=row['schedules']['all']['phases']['source_constructor'] if is_h else row['source_constructor'];eq('e.source_constructor',f'b({source.get("requested",source.get("requested_bytes"))},{source.get("moving",source.get("moving_bytes"))})')
  for name,member in [('all','full'),('selected128','selected128')]:
   sched=row['schedules'][name];ex='e.'+member
   eq(ex+'.retained_prefix',str(sched['Kpad']));eq(ex+'.solve',f'b({sched["kernel_requested"]},{sched["kernel_moving"]})');eq(ex+'.returned.selected',str(sched['selected_return_upper']));eq(ex+'.returned.refused',str(sched['Kpad']));eq(ex+'.returned.unresolved',str(sched['Kpad']));eq(ex+'.returned.union',str(max(sched['selected_return_upper'],sched['Kpad'])))
   phases=[(n,v) for n,v in sched['phases'].items() if n!='source_constructor']; eq(ex+'.phases().len()',str(len(phases)))
   lines.append('let expected=&[')
   for n,v in phases:
    parts=n.rsplit('_',1);precision=int(parts[1]) if parts[-1].isdigit() else None; key=parts[0] if precision else n
    assert key in phase_map,key
    lines.append(f'(PhaseId{{phase:KernelPhase::{phase_map[key]},precision:{"Some("+str(precision)+")" if precision else "None"}}},b({v["requested_bytes"]},{v["moving_bytes"]})),')
    counters['phase_pairs']+=1
   lines.append(f'];for (id,expected) in expected{{let actual={ex}.phases().iter().find(|p|p.id==*id).unwrap();assert_eq!(actual.bytes,*expected,"{{label}} {member} {{id:?}}");}}')
   r7=[]
   for precision in [256,512,1024]:
    key='R7_tracker_decision_'+str(precision)
    if key in sched['details']:
     cell=sched['details'][key];rv=cell['R7_requested_extra'];mv=cell['R7_moving_extra'];r7.append((rv,mv));eq(f'{ex}.r7[{[256,512,1024].index(precision)}]',f'Some(b({rv},{mv}))');counters['r7_pairs']+=1
    else:eq(f'{ex}.r7[{[256,512,1024].index(precision)}]','None')
   eq(ex+'.r7_max',f'b({max(x[0] for x in r7)},{max(x[1] for x in r7)})')
   # Tie handling: the public API promises one dominant phase, accepted if any tied reference maximum.
   lines.append(f'assert!({ex}.phases().iter().any(|p|p.id=={ex}.dominant_requested && p.bytes.requested=={ex}.solve.requested));assert!({ex}.phases().iter().any(|p|p.id=={ex}.dominant_moving && p.bytes.moving=={ex}.solve.moving));')
  lines.append('}');counters['fixtures']+=1
text='\n'.join(lines)+'\n';dest.write_text(text);(out/'rv33_reference.rs').write_text(text)
(out/'OVERLAY_BINDING.json').write_text(json.dumps({'path':str(dest),'sha256':hashlib.sha256(text.encode()).hexdigest(),'counters':counters,'bindings':bindings,'phase_mapping':phase_map,'method':'literal expectation projection only; no author arithmetic script executed, no duplicate kernel formula'},indent=2)+'\n')
print(json.dumps(counters))
