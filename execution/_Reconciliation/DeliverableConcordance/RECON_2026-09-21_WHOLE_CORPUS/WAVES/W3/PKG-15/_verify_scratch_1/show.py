import csv,json,sys
d=sys.argv[1]; mode=sys.argv[2]
sel=json.load(open('_verify_scratch_1/sel.json'))[d]['sel']
inv={k:c for c,ks in sel.items() for k in ks}
for r in csv.DictReader(open(f'DEL-15-{d}/DEL-15-{d}_forward.csv',encoding='utf-8')):
    k=r['ClaimKey']
    if k=='#END': continue
    s=inv.get(k)
    if mode=='all':
        print(f"{'*'+s if s else '  '} {k} | {r['UnitKind']} {r['ClaimType']} | {r['Disposition']} {r['CauseTag']} {r['AuthorityTier']} {r['BaselineClass']} {r['DivergenceLayers']} {r['CanonicalSituation']} {r['Confidence']} AN={r['AuthorityNeeded']} FG={r['FindingGroup']} | {r['ClaimSummary']}")
    elif s:
        print(f"=== [{s}] {k}")
        for f in r:
            if f in ('DeliverableID','SourceStateSHA','SelectableUnderCurrentLoop'): continue
            if r[f]: print(f"  {f}: {r[f]}")
