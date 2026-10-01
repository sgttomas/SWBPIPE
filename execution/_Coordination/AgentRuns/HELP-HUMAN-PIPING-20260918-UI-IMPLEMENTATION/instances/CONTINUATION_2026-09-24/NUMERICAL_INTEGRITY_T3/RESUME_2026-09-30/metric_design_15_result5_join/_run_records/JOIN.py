# One-off exact metadata composition evidence. No source algorithms or candidate calculators run.
from pathlib import Path
import json,hashlib,os,subprocess,datetime
OUT=Path(__file__).resolve().parent.parent; IN=OUT/'_inputs'; RAW=OUT/'_run_records'
K=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c')
REV='40129a225d73860ac2a53da9a2fa73869df668f3';ENV={**os.environ,'GIT_OPTIONAL_LOCKS':'0'}
def get(p):return json.loads((IN/p).read_text())
def put(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
CHECKS=[];GAPS=[]
def ck(name,ok,**detail):
    CHECKS.append({'check':name,'passed':bool(ok),**detail})
    if not ok:GAPS.append({'cell':name,**detail})
def keyed(rows,label):
    out={r['id']:r for r in rows};ck(label+'_unique',len(out)==len(rows),rows=len(rows));return out
CREF=get('caller14/REFERENCE193_CALLERS.json');CCLI=get('caller14/CLI24_CALLERS.json')
KREF=get('kernel22/KERNEL_193.json');KCLI=get('kernel22/CLI24_EXACT_KERNEL.json');KIF=get('kernel22/RESULT5_KERNEL_INPUTS.json')
cr=keyed(CREF['rows'],'caller193');cc=keyed(CCLI['rows'],'caller24');kr=keyed(KREF['rows'],'kernel193');kc=keyed(KCLI,'kernel24');ki=keyed(KIF,'kernel_interface193')
ck('reference_roster_equal',set(cr)==set(kr)==set(ki) and len(cr)==193)
ck('cli_roster_equal',set(cc)==set(kc) and len(cc)==24)
roster=get('metric_design_13_api_contract___run_records__ROSTER_CONTEXT.json')
meta=get('I21__source_09__FIXTURE_INPUTS.json');mm={r['id']:r for r in meta['vr_rows']}
expected_storage={};storage_bindings=[];attempts=0
for src in roster['storage_sources']:
    b=subprocess.check_output(['git','show',REV+':'+src['path']],cwd=K,env=ENV);h=hashlib.sha256(b).hexdigest();ck('original_storage_hash',h==src['sha256'],path=src['path']);f=IN/'storage'/Path(src['path']).name;f.parent.mkdir(exist_ok=True);f.write_bytes(b);storage_bindings.append({'path':src['path'],'sha256':h,'bytes':len(b)})
    for r in json.loads(b):
        if not r['attempts']:continue
        a=r['attempts'][0]['storage'];pair=(a['pattern_entries'],a['profile_entries'])
        for x in r['attempts']:
            q=x['storage'];ck('every_recorded_attempt_storage',pair==(q['pattern_entries'],q['profile_entries']),id=r['id']);attempts+=1
        expected_storage[r['id']]=pair
expected={r['id'] for r in meta['vr_rows'] if r['embedded_model'] and r['id'] in expected_storage}
ck('original_test_membership_193',set(cr)==expected and len(expected)==193)
ck('fixed_overlap_12',len(set(cr)&set(cc))==12)
# Same immutable family bytes appear in both candidate input binding tables.
cf=get('caller14/_run_records/INPUT_AUDIT.json');kf=get('kernel22/DESCRIPTORS.json')['sources'];kfiles={x['path']:x['sha256'] for x in kf}
for f in cf:ck('same_family_input_hash',kfiles.get(f['file'])==f['sha256'],path=f['file'])
put(RAW/'STORAGE_BINDING.json',{'source_basis':REV,'files':storage_bindings,'recorded_attempt_pairs':attempts,'test_rows':193,'no_counts_algorithm_run':True})
OUTCOME=['selected_or_other_outcome_comparison','record1_build_while_outcome_alive','expected_list_initialization_while_outcome_alive','nonselected_diagnostic_while_outcome_alive']
SHARED=['N','m','r','l','t','n','f','z','q','X','B','b']
ALL_PHASES=[];RESULTS={};MATCHES=[]
for context,cs,ks in [('reference193',cr,kr),('actual_cli24',cc,kc)]:
    results=[]
    for id,c in cs.items():
        k=ks[id];shape=c['shape'];d=k['descriptor'];own=k['owners'];schedules=k['schedules']
        start_gap=len(GAPS)
        for key in SHARED:ck('matching_descriptor_'+key,shape[key]==d[key],context=context,id=id,caller=shape[key],kernel=d[key])
        ck('matching_h',shape['h_W1_unused_for_sparse']==d['h'],context=context,id=id)
        ck('matching_source_id_bytes',shape['load_id_bytes']==d['source_id_bytes'],context=context,id=id)
        if context=='reference193':
            for key in ['s','d','u']:ck('matching_descriptor_'+key,shape[key]==d[key],context=context,id=id)
            ck('reference_upper_policy',shape['B']==shape['N'] and shape['b']==shape['f'] and ki[id]['B_policy']['kind']=='upper' and ki[id]['b_policy']['kind']=='upper',context=context,id=id)
            pair=expected_storage[id];ck('original_pattern_assertion_premise',shape['z']==d['z']==pair[0],id=id);ck('original_profile_assertion_premise',shape['h_W1_unused_for_sparse']==d['h']==pair[1],id=id)
        else:
            # s/d/u are omitted from caller24's original no-spring shape, not unknown symbols.
            ck('CLI24_original_adapter_zero_facts',d['s']==d['d']==d['u']==0,context=context,id=id)
        ck('no_unresolved_kernel_cell',not k['unresolved_kernel_cells'],context=context,id=id)
        for metric in ['requested','moving']:
            cell=c[metric];ck('no_unresolved_caller_cell',not cell['unbound_source_addends'],context=context,id=id,metric=metric)
            ck('same_SRC0_owner',cell['details']['source_retained']==own['SRC0'],context=context,id=id,metric=metric)
            ck('same_source_constructor',cell['details']['source_constructor']==k['source_constructor'][metric],context=context,id=id,metric=metric,caller=cell['details']['source_constructor'],kernel=k['source_constructor'][metric])
        ck('caller_solve_stationary',c['requested']['kernel_phase_addends']['solve_max']==c['moving']['kernel_phase_addends']['solve_max'],context=context,id=id)
        for schedule,sch in schedules.items():
            for metric in ['requested','moving']:
                field=metric+'_bytes';maximum=max(v[field] for v in sch['phases'].values());ties=sorted(key for key,v in sch['phases'].items() if v[field]==maximum)
                ck('kernel_phase_max_recomputed',maximum==sch['kernel_'+metric],context=context,id=id,schedule=schedule,metric=metric)
                ck('kernel_phase_attribution',ties==sorted(sch['dominant_'+metric]),context=context,id=id,schedule=schedule,metric=metric)
                ck('R7_max_recomputed',max(v[metric] for v in sch['R7_by_precision'].values())==sch['R7_max'][metric],context=context,id=id,schedule=schedule,metric=metric)
            u=sch['outcome_union'];ck('stationary_return_union',max(u['selected'],u['refused'],u['unresolved'])==u['stationary_requested']==u['stationary_moving'],context=context,id=id,schedule=schedule)
        # Recheck the subset warrants numerically from full phase rows, not their booleans.
        full=schedules['all'];short=schedules['selected128'];B0=own['B0'];S=own['S']['128'];U=own['U']['128']
        ck('B0_positive',B0>0,context=context,id=id);ck('S128_U128_positive',S+U>0,context=context,id=id)
        ck('O128_contains_B0_S_U',short['outcome_union']['stationary_requested']>=B0+S+U,context=context,id=id)
        ck('full_return_ge_short',full['outcome_union']['stationary_requested']>=short['outcome_union']['stationary_requested'],context=context,id=id)
        ck('short_roster_subset',set(short['phases'])<=set(full['phases']),context=context,id=id)
        for ph,v in short['phases'].items():
            for metric in ['requested','moving']:ck('each_short_phase_le_full',v[metric+'_bytes']<=full['phases'][ph][metric+'_bytes'],context=context,id=id,phase=ph,metric=metric)
        if context=='reference193':
            z=ki[id];ck('published_kernel_interface_matches',z['B0']==B0 and z['K_all']=={'requested':full['kernel_requested'],'moving':full['kernel_moving']} and z['K_128']=={'requested':short['kernel_requested'],'moving':short['kernel_moving']} and z['O_all']==full['outcome_union']['stationary_requested'] and z['O_128']==short['outcome_union']['stationary_requested'] and z['R7']==full['R7_max'],context=context,id=id)
        MATCHES.append({'context':context,'id':id,'descriptor':d,'matching_input_policy':'upper B=N/b=f, exact stored z/h, reference paths' if context=='reference193' else 'original exact B/b/z/h and CLI paths','source_bindings_match':len(GAPS)==start_gap})
        row={'context':context,'id':id,'family':mm[id]['family'],'descriptor':d,'status':'conditional composed candidate pending independent component and join review','metrics':{}}
        for metric in ['requested','moving']:
            cell=c[metric];comp=cell['result5_components'];fields={'model':comp['model_retained'],'fixed':B0+comp['fixed_caller_retained_addend'],'decide':full['R7_max'][metric]};maxima={};dominants={}
            ck('model_component_identity',fields['model']==comp['M']+comp['P_pub'],context=context,id=id,metric=metric)
            ck('fixed_component_identity',comp['fixed_caller_retained_addend']==comp['C_cut']+comp['E_lane']+comp['LIST']+comp['T_ctrl']+comp['P_pub'],context=context,id=id,metric=metric)
            ck('Ccut_ge_M',comp['C_cut']>=comp['M'],context=context,id=id,metric=metric)
            ck('comparison_contains_fixed_caller',cell['kernel_phase_addends'][OUTCOME[0]]>=comp['fixed_caller_retained_addend'],context=context,id=id,metric=metric)
            for schedule,sch in [('all',full),('selected128',short)]:
                phases={}
                for name,val in cell['caller_only_phases'].items():phases['caller_only:'+name]={'caller':val,'kernel':0,'total':val,'kernel_role':'not yet invoked or already dropped; source-defined absence'}
                add=cell['kernel_phase_addends'];phases['kernel_join:solve_max']={'caller':add['solve_max'],'kernel':sch['kernel_'+metric],'total':add['solve_max']+sch['kernel_'+metric],'kernel_phases':sch['dominant_'+metric],'kernel_role':'solve, matching metric; caller stationary'}
                for name in OUTCOME:
                    out=sch['outcome_union']['stationary_requested'];phases['kernel_join:'+name]={'caller':add[name],'kernel':out,'total':add[name]+out,'kernel_role':'stationary requested return, including explicit finite padding'}
                maximum=max(x['total'] for x in phases.values());dominant=[{'phase':name,**value} for name,value in phases.items() if value['total']==maximum];maxima[schedule]=maximum;dominants[schedule]=dominant
                ALL_PHASES.append({'context':context,'id':id,'metric':metric,'schedule':schedule,'phases':phases,'maximum':maximum,'dominant':dominant})
            fields['max']=maxima['all'];fields['sel128']=maxima['selected128'];ck('original_strict_ordering',fields['model']<fields['fixed']<fields['sel128']<=fields['max'],context=context,id=id,metric=metric,fields=fields)
            ck('all_five_u64',all(isinstance(v,int) and 0<=v<2**64 for v in fields.values()),context=context,id=id,metric=metric)
            row['metrics'][metric]={'fields':fields,'dominant':dominants,'ordering_margins':{'fixed_minus_model':fields['fixed']-fields['model'],'sel128_minus_fixed':fields['sel128']-fields['fixed'],'max_minus_sel128':fields['max']-fields['sel128']},'caller_fixed_addend':comp['fixed_caller_retained_addend'],'kernel_B0':B0}
        for field in ['model','fixed','decide','max','sel128']:ck('moving_ge_requested',row['metrics']['moving']['fields'][field]>=row['metrics']['requested']['fields'][field],context=context,id=id,field=field)
        row['legacy_estimate_fields']=row['metrics']['moving']['fields'];results.append(row)
    RESULTS[context]=results
# Preserve the already-reviewed actual CLI full global composition exactly.
old={x['id']:x for x in get('I21__vr_join_20__JOINED_PHASES.json')['rows']}
for row in ALL_PHASES:
    if row['context']!='actual_cli24' or row['schedule']!='all':continue
    prior=old[row['id']][row['metric']]['phases'];ck('CLI_full_phase_set_preserved',set(prior)==set(row['phases']),id=row['id'],metric=row['metric'])
    for name,value in row['phases'].items():ck('CLI_full_numeric_phase_preserved',value['total']==prior[name]['total'],id=row['id'],metric=row['metric'],phase=name)
# Summary is computed from composed totals, never a caller/kernel subtotal mislabeled global.
SUMMARY={}
for context,rows in RESULTS.items():
    SUMMARY[context]={'rows':len(rows),'metrics':{}}
    for metric in ['requested','moving']:
        vals={}
        for field in ['model','fixed','decide','sel128','max']:
            v=max(x['metrics'][metric]['fields'][field] for x in rows);vals[field]={'bytes':v,'ids':[x['id'] for x in rows if x['metrics'][metric]['fields'][field]==v]}
        SUMMARY[context]['metrics'][metric]=vals
put(OUT/'RESULT5_REFERENCE193.json',{'basis':REV,'context':'SingleCaseFamilyReferenceV1','status':'conditional pending independent component/join review; not implementation/artifact/admission acceptance','legacy_columns':'moving; requested column is same-owner requested-byte diagnostic','rows':RESULTS['reference193']})
put(OUT/'RESULT5_CLI24.json',{'basis':REV,'context':'actual RF-LARGE-only CLI24, original exact count/launch/input policy','status':'conditional pending independent component/join review; not implementation/artifact/admission acceptance','rows':RESULTS['actual_cli24']})
put(OUT/'COMPOSED_PHASES.json',{'metric_rule':'one active old buffer; solve caller stationary, returned kernel stationary','rows':ALL_PHASES})
put(OUT/'SUMMARY.json',SUMMARY);put(RAW/'MATCHES.json',MATCHES);put(RAW/'CHECKS.json',{'checks':len(CHECKS),'passing':sum(x['passed'] for x in CHECKS),'failing':len(GAPS),'details':CHECKS});put(OUT/'GAPS.json',GAPS)
print(json.dumps({'rows_reference':len(RESULTS['reference193']),'rows_cli':len(RESULTS['actual_cli24']),'phase_rows':len(ALL_PHASES),'checks':len(CHECKS),'gaps':GAPS[:5],'gap_count':len(GAPS),'summary':SUMMARY,'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2))
