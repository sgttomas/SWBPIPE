"""Read-only Rust text/JSON/integer checks for C2; imports no project code."""
from pathlib import Path
import hashlib
import json
import re

OUT = Path(__file__).resolve().parent
NUM = OUT
while NUM.name != 'numerics':
    NUM = NUM.parent
P = NUM / 'projects/chirality-piping'
FK = P / 'core/solver/frame_kernel'
origins = []


def read(path):
    data = path.read_bytes()
    origins.append({'path': str(path.relative_to(NUM)), 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)})
    return data.decode()


def enum(path, name, expected, excluded=()):
    text = read(path)
    match = re.search(r'\benum '+name+r'\s*\{(.*?)^\}', text, re.S | re.M)
    assert match, name
    variants = re.findall(r'^    ([A-Z][A-Za-z0-9_]*)\s*(?:\{|\(|,)', match[1], re.M)
    assert set(variants) == set(expected) | set(excluded), (name, variants)
    return {'enum': name, 'source_variants': variants, 'mapped': list(expected), 'explicit_product_exclusions': list(excluded), 'complete_type_inventory': True}


checks = []
checks.append(enum(FK/'src/structural/retained/adaptive.rs','AttemptReason', ['Stop','StopRule','VerificationFailed','PublicationEnclosure','VerificationEstimate','Uc','Theta','GValidity','Charge']))
checks.append(enum(FK/'src/structural/retained/adaptive.rs','AttemptStop', ['Budget','Span','Exponent','Arithmetic','Structure','Pivot','ZeroDiagonal','NegativeEnergy','Condition','ResidualGate','ResolutionScale','PublicationCertificate']))
checks.append(enum(FK/'src/structural/retained/adaptive.rs','UnresolvedReason', ['Ceiling','Budget','ExactSumSpan','ExponentRange','ZeroDiagonal','Arithmetic','ResolutionScaleUnencodable','CertifiedBoundUnencodable','PublicationCertificate']))
checks.append(enum(FK/'src/structural/retained/adaptive.rs','Refusal', ['MechanismWitnessed','GeometryUnavailable','NegativeEnergy','LedgerUnavailable','Structure']))
checks.append(enum(FK/'src/structural/retained/combine.rs','CombinationReason', ['CombinationUnresolved','NoOperands','NestedCombination','OperandsDiffer','LedgerUnavailable','Unresolved','Refused']))
checks.append(enum(FK/'src/structural/retained/wide.rs','WideError', ['InvalidPrecision','NonFinite','ExponentRange','DivisionByZero','NegativeSqrt','NotNormalized','AngleDomain','ArctangentLimit','SplitOverflow','Accumulator','OperandPrecision']))
checks.append(enum(FK/'src/exact_sum.rs','SumError', ['NonFinite','AccumulatorOverflow','NonRepresentable']))
checks.append(enum(FK/'src/structural/retained/source.rs','SourceError', ['NoNodes','NonFiniteCoordinate','NodeOutOfRange','DuplicateMemberId','RepeatedMemberNode','NonFiniteProperty','NonPositiveProperty','SubnormalDerivedPrimitive','ZeroLength','DegenerateAxis','DuplicateSpringId','NonPositiveSpring','DuplicateConstraint','NonFiniteValue','EmptyLoadSource','DuplicateStationId','UnknownMember','StationOutOfRange','DuplicateSupportId','SupportMismatch'], ['ZeroDirection','NonFiniteDirection']))
checks.append(enum(FK/'src/structural/retained/recover.rs','QuantityId', ['Displacement','DisplacementMagnitude','EndAction','StationAction','SpringAction','Reaction','SupportForceMagnitude','SupportMomentMagnitude'], ['DirectionalSpringAction']))
checks.append(enum(FK/'src/structural.rs','StructuralError', ['InvalidInput','Range','Asymmetric','NumericallyUnresolved','NegativeEnergy','Mechanism']))
checks.append(enum(FK/'src/lib.rs','FrameKernelError', ['NonFiniteInput','NonPositiveInput','DegenerateAxis','InvalidOrientation','InvalidNodeIndex','RepeatedElementNodeIndex','InvalidMatrixDimensions','InvalidVectorLength','RepeatedRestrainedDof','RestrainedDofOutOfRange','RepeatedPrescribedDof','PrescribedDofOutOfRange','SingularSystem','NumericalRange']))
checks.append(enum(FK/'src/structural.rs','ForceScaleReason', ['SubnormalAtFormation','InfeasibleWindow','ScaledEvaluation','PublicationOutsideBinary64']))
checks.append(enum(P/'core/product_physics/src/lib.rs','ForceScalingFailure', ['Refused','Failed','NotAdmitted','Publication','NotEngaged']))
checks.append(enum(FK/'src/structural.rs','ForceScaledError', ['Formation','Structural','Refused']))

text = read(FK/'src/structural.rs')
literals = []
for match in re.finditer(r'\bunresolved\(\s*"([^"]+)"', text):
    literals.append({'literal': match[1], 'line': text.count('\n',0,match.start())+1})
assert len(literals) == 8
excluded = 'contribution-preserved prescribed coupling changes zero reduced load'
assert sum(x['literal'] == excluded for x in literals) == 1

consumers = []
for rel in [
 'validation/benchmarks/numerical_robustness/src/records.rs',
 'core/solver/frame_kernel/tests/retained_k4/factor_tests.rs',
 'core/solver/frame_kernel/tests/retained_k4/references_tests.rs',
 'core/solver/performance_harness/src/k6/w1/staged.rs',
 'core/solver/performance_harness/src/bin/k6_observe/w1.rs',
 'core/solver/frame_kernel/tests/retained_k4/method_tests.rs',
 'core/solver/frame_kernel/tests/retained_k4/kf1_tracker_tests.rs',
 'core/solver/frame_kernel/tests/retained_k4/publication_tests.rs',
 'validation/benchmarks/numerical_robustness/src/lane.rs']:
    t = read(P/rel)
    sites=[]
    for m in re.finditer(r'CaseOutcome::Refused\s*\{',t):
        # Preserve bounded source excerpts; textual inspection, not a compiler.
        sites.append({'line': t.count('\n',0,m.start())+1,'excerpt':t[m.start():m.start()+220]})
    consumers.append({'path':rel,'sites':sites})
result={'scope':'Static source enum/consumer coverage and exact integer cardinalities only; no Rust/solver/model execution and no upstream no-wrap proof.', 'enums':checks,'ordinary_unresolved_literals':literals,'excluded_unresolved_literal':excluded,'consumer_sites':consumers,'record_cardinality':{'max_physical_records_per_schedule':4,'max_solve_slots':4,'max_verification_slots':3,'max_slot_requests_per_schedule':7,'proof_scope':'Structural schedule cardinality, not operation/byte/no-wrap bound.'}}
(OUT/'SOURCE_CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
# Exact raw whole-file origins; repeated reads collapsed without losing bytes.
unique={x['path']:x for x in origins}
(OUT/'CHECK_ORIGINS.json').write_text(json.dumps(list(unique.values()),indent=2)+'\n')
print(json.dumps({'enum_types':len(checks),'source_variants':sum(len(x['source_variants']) for x in checks),'mapped_variants':sum(len(x['mapped']) for x in checks),'explicit_scope_exclusions':sum(len(x['explicit_product_exclusions']) for x in checks),'ordinary_unresolved_literals':len(literals),'consumer_files':len(consumers)},indent=2))
