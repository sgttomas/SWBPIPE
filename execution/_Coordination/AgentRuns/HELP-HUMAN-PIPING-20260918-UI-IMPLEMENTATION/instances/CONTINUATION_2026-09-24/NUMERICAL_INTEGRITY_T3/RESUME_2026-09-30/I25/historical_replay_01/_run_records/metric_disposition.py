from inspect import *
a=js(REC/'ACTUAL_H_PEAK_COMPARISONS.json');bounds={(x['dataset'],x['run_id']):x for x in a};fields=js(REC/'HISTORICAL_RAW_HEAP_FIELDS.json');out=[]
for obj in fields:
 p=bounds[(obj['dataset'],obj['run_id'])]
 for key,value in obj['heaps'].items():
  x={k:v for k,v in obj.items()if k!='heaps'};x.update(field=key,observed_bytes=value)
  metric='moving'if'move'in key else'requested'
  if key=='heap_cap_bytes':
   x.update(disposition='configuration, not a heap observation',matching_bound_bytes=None)
  elif obj['dataset']!='K6B':
   bound=p[metric+'_bound'];x.update(disposition='VR global reference envelope arithmetic; historical launch/profile unresolved',matching_bound_bytes=bound,slack_bytes=bound-value)
  elif obj['kind']=='stage':
   st=p['stage_checks'][obj['stage']];bound=st[metric+'_bound'];x.update(disposition='H corresponding source/solve/prefix conditional envelope; historical path bound',matching_bound_bytes=bound,slack_bytes=bound-value)
  elif obj['kind']=='summary' and key.startswith('repeats_'):
   bound=p[metric+'_bound'];x.update(disposition='H inner source/solve conditional envelope; historical path bound',matching_bound_bytes=bound,slack_bytes=bound-value)
  elif obj['kind']=='summary' and key.startswith('prefix_'):
   bound=p['prefix_'+metric+'_bound'];x.update(disposition='H outer-prefix conditional envelope; historical path bound',matching_bound_bytes=bound,slack_bytes=bound-value)
  else:
   x.update(disposition='outside H staged/prefix metric claim: global peak or pre-reset stage_begin; preserved without a false window match',matching_bound_bytes=None)
  out.append(x)
comp=[x for x in out if x.get('matching_bound_bytes')is not None]
assert all(x['slack_bytes']>=0 for x in comp)
save('RAW_METRIC_DISPOSITION.json',out)
summary={'raw_heap_fields':len(out),'window_matched_arithmetic_fields':len(comp),'outside_H_metric_fields':sum(x['disposition'].startswith('outside H')for x in out),'configuration_fields':sum(x['disposition'].startswith('configuration')for x in out),'negative_slack_fields':0,'limits':'H global and pre-reset measurements remain outside the qualified staged/prefix expression; no new whole-process H theorem'}
save('RAW_METRIC_DISPOSITION_SUMMARY.json',summary)
print(json.dumps(summary,indent=2))
print('10000 row summaries')
replay=js(REC/'ACTUAL_H_REPLAY.json')
for x in replay:
 if x['members']==10000:
  p=bounds[x['dataset'],x['run_id']];new=x['new_as_recorded_history'];print(x['dataset'],x['run_id'],x['old_estimate_bytes'],x['new_estimate_bytes'],x['estimate_delta_bytes'],new.get('rho'),new.get('projected_rss_bytes'),p['requested_slack'],p['moving_slack'])
print('minimums', {ds:{m:min(p[m+'_slack']for p in a if p['dataset']==ds)for m in ['requested','moving']}for ds in ['K6B','VK','KF3']})
