#!/usr/bin/env python3
"""Replay accepted candidate validator on actual DAG-011 bytes without editing it.

Only I/O roots are rebound below; the seven negative probes and graph predicates
are executed unchanged. Outputs live in this application evidence directory.
"""
import hashlib,json,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
OUT=SCA/'application/dependencies';DAG=ROOT/'projects/chirality-piping/execution/_DAG/DAG-011'
source=SCA/'dependencies/validate_candidate.py';code=source.read_text()
replacements=[
 ('OUT=Path(__file__).resolve().parent', 'OUT=Path('+repr(str(SCA/'dependencies'))+')'),
 ("rows=read(OUT/'DependencyEdges.csv'); nodes=read(OUT/'DeliverableNodes.csv')", "rows=read(Path("+repr(str(DAG/'DependencyEdges.csv'))+")); nodes=read(Path("+repr(str(DAG/'DeliverableNodes.csv'))+"))"),
 ("reg=OUT.parent/'candidate/projects/chirality-piping/docs/_Registers/Deliverables.csv'", "reg=ROOT/'projects/chirality-piping/docs/_Registers/Deliverables.csv'"),
 ("(OUT/name).read_bytes()", "(Path("+repr(str(DAG))+")/name).read_bytes()"),
 ("digest(OUT/name)", "digest(Path("+repr(str(DAG))+")/name)"),
 ("(OUT/'StageGraph.json').write_text", "(Path("+repr(str(OUT/'StageGraph.json'))+")).write_text"),
 ("(OUT/'VALIDATION.json').write_text", "(Path("+repr(str(OUT/'STAGED_VALIDATION.json'))+")).write_text"),
]
for old,new in replacements:
 assert code.count(old)==1,old
 code=code.replace(old,new)
try:exec(compile(code,str(source),'exec'),{'__file__':str(source),'__name__':'__main__'})
except SystemExit as exc:
 if exc.code:raise
report=json.loads((OUT/'STAGED_VALIDATION.json').read_text())
report.update({'phase':'GROUP2_STAGED_NOT_APPROVED','read_subject':'actual execution/_DAG/DAG-011 bytes and canonical applied Deliverables.csv','validator_source':str(source.relative_to(ROOT)),'validator_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'io_rebindings':[old for old,new in replacements],'adoption_claim':False})
(OUT/'STAGED_VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
