import pathlib,json,hashlib,re,datetime
r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30");q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02");rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
source={}
for line in (rr/'GIT_SOURCE_TREE.txt').read_text().splitlines():
 left,path=line.split('\t',1); mode,kind,oid=left.split()
 assert kind=='blob'
 b=(q/'source'/path).read_bytes();actual=hashlib.sha1(('blob '+str(len(b))+'\0').encode()+b).hexdigest()
 assert actual==oid,(path,actual,oid)
 source[path]={'git_blob':oid,'content_matches':True,'archive_sha256':hashlib.sha256(b).hexdigest()}
assert len(source)==299
(rr/'SOURCE_GIT_BLOB_CHECK.json').write_text(json.dumps({'freeze':'81c03849033f3ce745668f581f446530789397b8','file_count':len(source),'matches':source},indent=2)+'\n')
inputs=['I21/layout_04/LABEL_TYPE_MAP.json','I21/layout_04/H_LABEL_TYPE_MAP.json','I21/layout_04/ARC_SOURCE_BINDING.json','I21/layout08_prep/LABEL_TYPE_MAP.json','I21/layout08_run/PREFLIGHT.json','source_review_RV30/layout08_03/backcheck_run/RETURN.md','I21/source_12/LIBRARY_SOURCE.json','I21/source_13/LIBRARY_SOURCE.json','I23/vr_nodes_source_16/_run_records/BINDING.json','I23/wrapped_error_artifacts_18/_run_records/SOURCE_BINDINGS.json','I23/serde_binding_07/PACKAGE_BINDING.json','source_review_RV30/vr_nodes_17/RETURN.md','source_review_RV30/h_request_bindings_10/RETURN.md','source_review_RV30/wrapped_errors_19/RETURN.md','source_review_RV30/format_stream_18/RETURN.md','verification/public_layout_20/_run_records/public_layout.rs','verification/public_layout_20/_run_records/VALUES.json','source_review_RV30/public_layout_result_22/RETURN.md','I28/ordinary_qualification_preflight_01/_run_records/EVIDENCE_ORIGINS.json']
(rr/'CONSUMED_SOURCE_RECORDS.json').write_text(json.dumps([{'path':str(r/p),'sha256':sha(r/p),'relative_to_R':p} for p in inputs],indent=2)+'\n')
args=json.loads((rr/'DIRECT_ALLOCATION_ARGUMENTS.json').read_text())
by={x['label']:x for x in args}
expected={'h_mutex':[(64,8)],'vr_mutex':[(64,8)],'h_registry':[(544,8)],'vr_registry':[(544,8)],'h_geometry_root':[(144,8)],'vr_geometry_root':[(144,8)],'h_geometry_split':[(144,8),(240,8)],'vr_geometry_split':[(144,8),(240,8)],'h_tracker_root':[(1072,8)],'vr_tracker_root':[(1072,8)],'h_tracker_split':[(1072,8),(1168,8)],'vr_tracker_split':[(1072,8),(1168,8)],'h_holding_split':[(104,8),(200,8)],'vr_holding_split':[(104,8),(200,8)],'vr_json_root':[(632,8)],'vr_publication_split':[(848,8),(944,8)],'vr_allowance_split':[(368,8),(464,8)],'vr_pair_set_split':[(104,8),(200,8)],'vr_usize_set_split':[(104,8),(200,8)],'vr_case_bulk':[(544,8)],'vr_floor_bulk':[(456,8)],'vr_controls_bulk':[(280,8)],'vr_io_new_str':[(24,8)],'vr_json_error_io':[(40,8)]}
checks=[]
for label,pairs in expected.items():
 calls=by[label]['calls']
 for size,align in pairs:
  hits=[c['address'] for c in calls if c['size']==size and c['alignment']==align];assert hits,(label,size,align)
  checks.append({'range':label,'size':size,'alignment':align,'calls':hits,'exact_type':by[label]['demangled_symbol'],'note':'Numeric presence check is supplemental; typed constructor/caller/source attribution is in TRANSFER.md and raw ranges.'})
# Public reporter rows and all fresh artifact/raw-output hashes remain intact.
art=json.loads((rr/'ARTIFACTS.json').read_text())
assert all(sha(x['path'])==x['sha256'] for x in art.values())
native=json.loads((rr/'NATIVE_COMMANDS.json').read_text())
for x in native:
 assert x['exit']==0 and sha(rr/x['stdout_file'])==x['stdout_sha256'] and sha(rr/x['stderr_file'])==x['stderr_sha256']
stats={'commands':len(native),'bounded_disassembly_ranges':sum(x['argv'][0]=='/usr/bin/objdump' for x in native),'code_bytes':sum(x.get('range_bytes',0) for x in native),'all_native_exit0':True,'all_stderr_empty':all((rr/x['stderr_file']).stat().st_size==0 for x in native)}
(rr/'REQUEST_ARGUMENT_CHECKS.json').write_text(json.dumps({'matches':checks,'native_summary':stats,'unknown_private_required_cells':[],'source_composed_internal_pairs':{'String_Value':[728,8],'borrowed_controls':[376,8],'floor_pair':[552,8],'String_String':[640,8]},'not_claimed':'No constructor execution/population or global runtime peak inferred; tuple3 spring-set request remains outside these bindings.'},indent=2)+'\n')
print(json.dumps({'source_blob_matches':len(source),'numeric_pair_checks':len(checks),**stats,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}))

