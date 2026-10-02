import json, pathlib, importlib.util, tempfile,sys
from unittest import mock
out=pathlib.Path(__file__).parent;arc=json.loads((out/'ARCHIVE.json').read_text());path=pathlib.Path(arc['archive'])/'projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py';spec=importlib.util.spec_from_file_location('rv33_runner',path);r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
run=next(x for x in r.schedule() if x['tier']=='W1-T2' and x['mode']=='w1a' and not x['conditional']);events=[]
with tempfile.TemporaryDirectory(dir=arc['archive'],prefix='hold_repro_') as folder:
 counts_path=pathlib.Path(folder)/'counts.jsonl';counts_path.write_text(json.dumps({'kind':'counts','model':run['model'],'estimate_adm_bytes_w1a':1})+'\n');(pathlib.Path(folder)/'metadata.json').write_text('{}\n')
 def fake_launch(argv,**kw):
  events.append({'event':'launch','argv':argv,'counts_only':'--counts-only' in argv})
  (pathlib.Path(folder)/(kw['run_id']+'.jsonl')).write_text(json.dumps({'kind':'counts','model':run['model'],'estimate_adm_bytes_w1a':321})+'\n')
  return {'classification':'ok','wall_s':0,'exit_code':0,'rss':{},'peak_rss_bytes':1}
 original=r.admission
 def admission(*args,**kw):
  result=original(*args,**kw);events.append({'event':'admission','result':result});return result
 with mock.patch.object(r,'schedule',return_value=[run]),mock.patch.object(r,'launch',side_effect=fake_launch),mock.patch.object(r,'baseline_run',return_value={'rss':{}}),mock.patch.object(r,'wait_for_quiet_host',return_value={}),mock.patch.object(r,'admission',side_effect=admission):
  result=r.run_tier(run['tier'],'NOT_EXECUTED_BINARY',str(counts_path),folder,'candidate','tree',log=lambda *args:None)
 rows=r.run_entries(r.read_records(folder))
result={'candidate':'10315a8167c47f43aa41402beb88ed2c70e62cf2','run':run,'events':events,'records':rows,'expected':'ascent hold checked before any model counts prepass launch','observed':'no model-specific prepass, numeric admission or normal launch; recorded ascent hold','no_real_subprocesses':True}
(out/'RUNNER_HOLD_REPRO.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));assert events==[];assert rows[-1]['classification']=='not_run';assert rows[-1]['admission']['reason'].startswith('deferred:ascent_previous_size_not_recorded');assert rows[-1]['admission']['estimate_adm_bytes'] is None
