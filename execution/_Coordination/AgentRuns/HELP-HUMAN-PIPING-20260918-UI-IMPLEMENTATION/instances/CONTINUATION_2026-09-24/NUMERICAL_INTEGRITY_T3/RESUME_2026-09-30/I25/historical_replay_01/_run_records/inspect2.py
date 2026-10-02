from inspect import *
x=js(R/'I21/k0_assembly_16/HISTORICAL_COMPARISON.json')
print('INPUTS',json.dumps(x['inputs'],indent=1));print('LAST TWO',json.dumps(x['rows'][-2:],indent=1))
show(H/'observations/k6b/k6b_packet.json')
x=js(R/'I21/h_numeric_19/PHASES.json')['rows'][0]; print('HKEYS',list(x));print('HLAUNCH',json.dumps({k:v for k,v in x.items() if k not in ['schedules','descriptors','input_upper','owners']},indent=1)[:11000])
