for ci,c in enumerate(cases):
 ident=c['id'];expectedrow=proposed[ident];d=descriptors[ident];m=models[ident];Kc=d
 for field,actual in [('N',len(m['nodes'])),('m',len(m['members'])),('r',len(m['constraints'])),('l',len(m['loads'])),('t',len(m['stations']))]:ck(ident+' descriptor '+field,d[field],actual)
 for field in ['n','f','z','q','B','b']:ck(ident+' count '+field,d[field],Kc[field])
 N,n,f,z,q,B,mm,r=[d[k] for k in ['N','n','f','z','q','B','m','r']];ids=sum(len(t[3].encode()) for t in m['loads']);X=38+24*N+84*mm+17*d['s']+41*d['d']+13*r+17*d['l']+ids+16*d['t'];ck(ident+' encoding length',X,d['X'])
 I=len(ident);FM=len(c['family']);ext=c['model'] is None;mp=str(WT/external[ident]['raw_path'].removeprefix('<wt>/')) if ext else None
 av=[binary,'--case',ident,'--heap-cap-bytes','8053063680']+(['--model-file',mp] if ext else []);cv=[binary,'--case',ident,'--heap-cap-bytes','536870912']+(['--model-file',mp] if ext else [])+['--counts-only'];av20=[binary,'--case',ident,'--heap-cap-bytes','18446744073709551615'];argv=max(24*len(v)+sum(len(x.encode()) for x in v) for v in [av,cv,av20])
 lc=launches['rows'][ci];ck(ident+' argv normal lengths',list(map(len,av)),lc['argument_utf8_lengths']);ck(ident+' argv counts lengths',list(map(len,cv)),lc['counts_argument_utf8_lengths'])
 args=I+(len(mp) if ext else 0);M,Mo=model(m,clone=not ext);C=casebounds[ident][0];base=1700+args+I+C+M+(128 if ext else 0);cut=base+128
 source=24*N+pushed(88,mm)+pushed(24,d['s'])+pushed(48,d['d'])+pushed(16,r)+pushed(40,d['l'])+pushed(16,d['t'])+16*n+4*N+ids;parts=source-16*n-4*N
 rows=len(c['rows']);controls=len(c['controls']);maxkey=max(len(x[0]) for x in c['rows']);maxexpected=max(len(x[1]) for x in c['rows']);sumkey=sum(len(x[0]) for x in c['rows']);names=sum(len(x[0]) for x in m['members']);edges=min(f*(f-1),144*mm+d['s']+9*d['d'])
 for e,metric in [(0,'requested'),(1,'moving')]:
  cand=expectedrow[metric];detail=cand['details'];exactrow,exactcontrol,held=exact_helpers(c,e);rec,hr,dout=recbound(c,B,e);sp=sparse(d,e)
  radj,hadj=adjacency(n,f,edges,e);hprofile=max(rcm(f,edges,e,4),24*f+pushed(8,f)+e*old(8,f));countsH=max(hadj,radj+hprofile,pushed(8,f)+e*old(8,f),pushed(8,f)+N+insert('pair',2*mm),pushed(8,f)+pushed(20,q)+e*old(20,q),pushed(8,f)+pushed(20,q)+encode(X,e))
  sourceH=source+max(sortmem(88,mm),sortmem(24,d['s']),sortmem(48,d['d']),sortmem(16,r),sortmem(40,d['l']),sortmem(16,d['t']),3*2144,12*N,e*max(old(88,mm),old(24,d['s']),old(48,d['d']),old(16,r),old(40,d['l']),old(16,d['t'])))
  why=max(9,40,75,maxkey+18);Df=max(I+maxkey+maxexpected+6+why,I+maxkey+94,I+93,I+56,I+36,I+39+dout,I+75,I+20,I+16+dout);nf=rows+4
  controlstrings=sum(fmt(I+1+len(x[0])) for x in c['controls']);ctdiag=2*pushed(24,controls)+controlstrings
  diag=ctdiag+pushed(24,nf)+nf*fmt(Df)+pushed(24,rows)+sumkey+2*pushed(24,2*rows)+2*rows*(fmt(I+maxkey+92)+fmt(I+maxkey+1))
  floorR=insert('floor',mm)+names;floorH=names+max(40*mm+sortmem(40,mm),40*mm+bulk('floor',mm));pub=insert('published',q)
  details={'cut_survivors':cut,'source_retained':source,'source_constructor':sourceH,'count_helpers':countsH,'family_load_common':Family[c['family']][e],'model_standalone':M,'case_retained':C,'Exact_row_max':exactrow,'Exact_control_max':exactcontrol,'record_retained':rec,'record_build':hr,'diagnostics':diag,'published_map':pub,'floor_map':floorR,'sparse':sp,'launch_bytes':argv}
  for key,v in details.items():ck(ident+' '+metric+' '+key,v,detail[key])
  # Phase arithmetic below is recomposed from the checked identities, not read
  # from proposed phase values. Owner completeness is assessed in REVIEW.md.
  load=1700+args+I+Family[c['family']][e]
  if ext:
   fl,(jr,jo)=external_data[ident];mr,mo=model(m);eh=max((len(mp)+1 if len(mp)>=384 else 0),readfile(fl,e),max(8,2*(fl+32))+jr+e*jo,max(8,2*(fl+32))+jr+mr+e*mo,max(8,2*(fl+32))+jr+mr+hashing(fl,e));load=max(load,1700+args+I+C+eh)
  else:load=max(load,1700+args+I+C+M)
  ds=I+len(': source refused: ')+75;dh=max(I+len(': K4SRC sha256 ')+64+len(", the generator's ")+64,I+len(': model file sha256 ')+64+len(', committed ')+64)
  prefix=max(1700+argv,load,base+output('start',5+I+(128 if ext else 0)),base+sourceH,base+source+max(encode(X,e),arbitrary(1,X)+hashing(X,e)),cut+source+countsH,base+sourceH+max(fmt(ds)+e*ds,fmt(ds)+output('fail',5+ds,False)),cut+source+max(fmt(dh)+e*dh,fmt(dh)+output('fail',5+dh,False)))
  lookup=insert('borrowed',rows);cb=max(24*rows+sortmem(24,rows),24*rows+bulk('borrowed',rows));dn=I+1+max([0]+[len(x[0]) for x in c['controls']]);control=max(cb,lookup+ctdiag+max(exactcontrol,e*old(24,controls),fmt(dn)+e*dn))
  nc=pushed(24,rows)+sumkey if c.get('s_full') is not None else 0;run=diag+pub+rec+I+FM+16;late=cut+78+rec+nc
  reportchild=6+I+32*min(50,nf)+min(50,nf)*Df+32*rows+sumkey+32*len(c['not_covered'])+sum(len(x) for x in c['not_covered'])+(32*rows+sumkey if c.get('s_full') is not None else 0)+64*rows+2*rows*(I+maxkey+92)+32*controls+sum(I+1+len(x[0]) for x in c['controls'])
  phases={'prefix_before_cut':prefix,'postcut_counts_output':cut+output('counts',6+I+64),'counts_only_summary_sample':cut+632+(54 if e else 32)+I+11,'half_cap_refusal_output':cut+output('half_cap_refusal',7+I+len('estimate_exceeds_half_cap'),False),'sourceparts_controls':cut+parts+control,'lane_source_constructor':cut+ctdiag+sourceH,'lane_encoding_hash':cut+ctdiag+source+max(encode(X,e),arbitrary(1,X)+hashing(X,e)),'source_refused_caller_only':cut+parts+ctdiag+max(fmt(91)+e*91,fmt(I+maxkey+94)+e*(I+maxkey+94),diag+hr+225),'record2_w1_output':cut+78+run+rec+output('w1',2+I+dout),'full_floor':cut+78+run+rec+max(floorH,floorR+nc+max(exactrow,e*old(24,rows))),'report':cut+78+run+rec+nc+8*min(50,nf)+output('report',reportchild),'record3_output':cut+78+run+rec+nc+output('record_wrapper',6+I+rec,False),'late_RCM':max(late+max(hadj,radj+rcm(f,edges,e,4),radj+8*f+rcm(f,edges,e,6),radj+16*f),late+output('rcm',3+I)),'late_sparse_parity':max(late+sp['max'],late+66144+output('binary64_error',8+I+33072),late+200+output('binary64_success',8+I+100)),'summary_sample':late+632+(54 if e else 32)+I}
  for key,v in phases.items():ck(ident+' '+metric+' phase '+key,v,cand['caller_only_phases'][key])
  expectedPaths=len(manifest)+arbitrary(1,len(manifest+'/cases'))+arbitrary(1,len(expected_path))+e*len(expected_path);expH=expectedPaths+max((len(expected_path)+1 if len(expected_path)>=384 else 0),readfile(len(expected),e),max(8,2*(len(expected)+32))+jexpect+max(e*oexpect,78,120 if e else 80))
  dg=max(old(24,nf),old(24,2*rows),old(24,rows),old(24,controls),Df,I+maxkey+92);dp=max(exactrow,375 if e else 300,held+fmt(Df)+e*Df+max(fmt(why),75),held+e*dg);comparison=max(diag+pub+16*N,diag+pub+4*N+floorH,diag+pub+4*N+floorR+dp)
  slots={'solve_max':cut+ctdiag+arbitrary(1,X)+pushed(24,4)+4*fmt(Df),'selected_or_other_outcome_comparison':cut+arbitrary(1,X)+78+comparison,'record1_build_while_outcome_alive':cut+arbitrary(1,X)+78+diag+pub+hr+I+FM+16,'expected_list_initialization_while_outcome_alive':cut+ctdiag+arbitrary(1,X)+expH+pushed(24,4)+4*fmt(Df),'nonselected_diagnostic_while_outcome_alive':cut+arbitrary(1,X)+78+diag+fmt(dout)+fmt(I+39+dout)+e*(I+39+dout)}
  for key,v in slots.items():ck(ident+' '+metric+' kernel addend '+key,v,cand['kernel_phase_addends'][key])
  ck(ident+' '+metric+' maximum',max(phases.values()),cand['caller_only_max']);ck(ident+' '+metric+' dominant',max(phases,key=phases.get),cand['dominant_caller_phase'])
  priorfails=pushed(24,4)+4*fmt(Df)
  results.append({'id':ident,'metric':metric,'details':details,'phases':phases,'kernel_addends':slots,'prelist_prior_failure_upper':priorfails})
  fixed= cut+arbitrary(1,X)+78+ctdiag+pub
  result=cand['result5_components']
  for key,value in {'M':M,'P_pub':pub,'C_cut':cut,'E_lane':arbitrary(1,X),'LIST':78,'T_ctrl':ctdiag,'model_retained':M+pub,'fixed_caller_retained_addend':fixed,'comparison_over_fixed_addend':slots['selected_or_other_outcome_comparison']-fixed,'cut_minus_model':cut-M}.items():ck(ident+' '+metric+' result5 '+key,value,result[key])
  ck(ident+' '+metric+' no unbound caller cell',cand['unbound_source_addends'],{})
